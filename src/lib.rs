#![allow(dead_code)]

// Needed:
//  - Find an easy way for users to get callbacks specific to their message type
//  - Users should have access to the temrinal renderer... somehow...

use std::{cell::RefCell, cmp::Ordering, rc::Rc};

use backend::{DehydratedSpan, YewBackend};
use ratatui::{prelude::Rect, Frame, Terminal};
use touch_scroll::TouchScroll;
use web_sys::{
    js_sys::Function,
    wasm_bindgen::{prelude::Closure, JsValue},
    TouchEvent, WheelEvent,
};
use yew::{Component, Context, Properties};

pub mod backend;
pub mod prelude;
pub mod palette;
mod touch_scroll;

/// A container for a TUI app that renders to HTML.
pub struct WebTerminal<A> {
    app: A,
    term: RefCell<Terminal<YewBackend>>,
}

/// In the public API because of the component impl of WebTerminal
pub enum WebTermMessage<M> {
    Inner(M),
    Resized,
    Scrolled(ScrollMotion),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollMotion {
    Up,
    Down,
}

impl<M> WebTermMessage<M> {
    pub fn new<I: Into<M>>(inner: I) -> Self {
        Self::Inner(inner.into())
    }
}

impl<M> From<M> for WebTermMessage<M> {
    fn from(value: M) -> Self {
        Self::Inner(value)
    }
}

/// In the public API because of the component impl of WebTerminal
#[derive(Properties, PartialEq)]
pub struct WebTermProps<M: PartialEq> {
    pub inner: M,
}

impl<M: PartialEq> WebTermProps<M> {
    pub fn new(inner: M) -> Self {
        Self { inner }
    }
}

/// The core user-facing abstraction of this crate. A terminal app is a type that can be wrapped by
/// a [`WebTerminal`] and be displayed by Yew.
///
/// Because the app needs to be passed via properties, it needs to be `'static`, `Clone`, and
/// `PartialEq`.
pub trait TerminalApp: 'static + Clone + PartialEq {
    /// The message type that this type uses to update.
    type Message;

    /// Allows the app to initialize its environment, such as setting up callbacks to window
    /// events.
    #[allow(unused_variables)]
    fn setup(&mut self, ctx: &Context<WebTerminal<Self>>) {}

    // TODO: Add optional resize method

    /// Allows the app to initialize its environment, such as setting up callbacks to window
    /// events.
    #[allow(unused_variables)]
    fn scroll(&mut self, scroll: ScrollMotion) -> bool {
        false
    }

    /// Updates the app with a message.
    fn update(&mut self, ctx: &Context<WebTerminal<Self>>, msg: Self::Message) -> bool;

    /// Takes a Ratatui [`Frame`] and renders widgets onto it.
    fn render(&self, area: Rect, frame: &mut Frame<'_>);

    /// Takes a dehydrated spans from the backend and hydrates them by adding callbacks,
    /// hyperlinks, etc.
    #[allow(unused_variables)]
    fn hydrate(&self, ctx: &Context<WebTerminal<Self>>, span: &mut DehydratedSpan) {}
}

impl<A: TerminalApp> Component for WebTerminal<A> {
    type Message = WebTermMessage<A::Message>;
    type Properties = WebTermProps<A>;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let mut app = ctx.props().inner.clone();
        app.setup(ctx);
        let term = RefCell::new(Terminal::new(YewBackend::new()).unwrap());
        /* ---------- Window callback setup --------- */
        let window = web_sys::window().unwrap();
        // Bind a function to the "on-resize" window event
        let cb = ctx.link().callback(|()| WebTermMessage::Resized);
        let func = move || cb.emit(());
        let func: Function = Closure::<dyn 'static + Fn()>::new(func)
            .into_js_value()
            .into();
        window.set_onresize(Some(&func));
        // Bind a function to the "on-wheel" window event
        let cb = ctx.link().callback(|msg: Self::Message| msg);
        let func = move |event: JsValue| {
            let event: WheelEvent = event.into();
            match event.delta_y().partial_cmp(&0.0) {
                Some(Ordering::Less) => cb.emit(WebTermMessage::Scrolled(ScrollMotion::Down)),
                Some(Ordering::Greater) => cb.emit(WebTermMessage::Scrolled(ScrollMotion::Up)),
                _ => {}
            }
        };
        let func: Function = Closure::<dyn 'static + Fn(JsValue)>::new(func)
            .into_js_value()
            .into();
        window.set_onwheel(Some(&func));

        // In order to emulate scrolling on mobile, a simple (perhaps too simple) approach is
        // taken. Touch events are started in an accumulator behind a `RefCell`. This accumulator
        // tracks when two touches should be connected and tracks the overall progress. When enough
        // progress has been made, a scroll message is emitted. This approach is a bit naive, but
        // we're going for functional first

        // Bind a function to the "touch-start" window event
        let acc = Rc::new(RefCell::new(TouchScroll::new()));
        let acc_start = Rc::clone(&acc);
        let func = move |event: JsValue| {
            let event: TouchEvent = event.into();
            if let Some(touch) = event.touches().get(0) {
                acc_start.borrow_mut().init_touch(&touch);
            }
        };
        let func: Function = Closure::<dyn 'static + Fn(JsValue)>::new(func)
            .into_js_value()
            .into();
        window.set_ontouchstart(Some(&func));

        // Bind a function to the "touch-move" window event
        let acc_move = Rc::clone(&acc);
        let cb = ctx.link().callback(|msg: Self::Message| msg);
        let func = move |event: JsValue| {
            let event: TouchEvent = event.into();
            if let Some(touch) = event.touches().get(0) {
                acc_move
                    .borrow_mut()
                    .add_touch(&touch)
                    .for_each(|scroll| cb.emit(WebTermMessage::Scrolled(scroll)));
            }
        };
        let func: Function = Closure::<dyn 'static + Fn(JsValue)>::new(func)
            .into_js_value()
            .into();
        window.set_ontouchmove(Some(&func));
        Self { app, term }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            WebTermMessage::Inner(msg) => self.app.update(ctx, msg),
            WebTermMessage::Scrolled(dir) => self.app.scroll(dir),
            WebTermMessage::Resized => {
                self.term.get_mut().backend_mut().resize_buffer();
                true
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let mut term = self.term.borrow_mut();
        let area = term.size().unwrap();
        term.draw(|frame| self.app.render(area, frame)).unwrap();
        term.backend_mut()
            .hydrate(|span| self.app.hydrate(ctx, span))
    }
}
