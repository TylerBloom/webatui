use std::{cell::RefCell, cmp::Ordering, rc::Rc};

use web_sys::{
    js_sys::Function,
    wasm_bindgen::{prelude::Closure, JsValue},
    TouchEvent, WheelEvent,
};
use yew::Context;

use crate::{ScrollMotion, TerminalApp, WebTermMessage, WebTerminal};

pub(crate) fn process_resize_event<A: TerminalApp>(ctx: &Context<WebTerminal<A>>) -> Function {
    let cb = ctx.link().callback(|()| WebTermMessage::Resized);
    let func = move || cb.emit(());
    Closure::<dyn 'static + Fn()>::new(func)
        .into_js_value()
        .into()
}

pub(crate) fn process_wheel_event<A: TerminalApp>(ctx: &Context<WebTerminal<A>>) -> Function {
    let cb = ctx.link().callback(|msg: WebTermMessage<A::Message>| msg);
    let func = move |event: JsValue| {
        let event: WheelEvent = event.into();
        match event.delta_y().partial_cmp(&0.0) {
            Some(Ordering::Less) => cb.emit(WebTermMessage::Scrolled(ScrollMotion::Down)),
            Some(Ordering::Greater) => cb.emit(WebTermMessage::Scrolled(ScrollMotion::Up)),
            _ => {}
        }
    };
    Closure::<dyn 'static + Fn(JsValue)>::new(func)
        .into_js_value()
        .into()
}

// In order to emulate scrolling on mobile, a simple (perhaps too simple) approach is
// taken. Touch events are started in an accumulator behind a `RefCell`. This accumulator
// tracks when two touches should be connected and tracks the overall progress. When enough
// progress has been made, a scroll message is emitted. This approach is a bit naive, but
// we're going for functional first

pub(crate) fn process_touch_init_event(acc: Rc<RefCell<TouchScroll>>) -> Function {
    let func = move |event: JsValue| {
        let event: TouchEvent = event.into();
        if let Some(touch) = event.touches().get(0) {
            acc.borrow_mut().init_touch(&touch);
        }
    };
    Closure::<dyn 'static + Fn(JsValue)>::new(func)
        .into_js_value()
        .into()
}

pub(crate) fn process_touch_move_event<A: TerminalApp>(
    ctx: &Context<WebTerminal<A>>,
    acc: Rc<RefCell<TouchScroll>>,
) -> Function {
    let cb = ctx.link().callback(|msg: WebTermMessage<A::Message>| msg);
    let func = move |event: JsValue| {
        let event: TouchEvent = event.into();
        if let Some(touch) = event.touches().get(0) {
            acc.borrow_mut()
                .add_touch(&touch)
                .for_each(|scroll| cb.emit(WebTermMessage::Scrolled(scroll)));
        }
    };
    Closure::<dyn 'static + Fn(JsValue)>::new(func)
        .into_js_value()
        .into()
}

use web_sys::Touch;

/// Touch events (for mobile) are not emitted from the browser in a continuous stream. As such,
/// they are not tightly linked and require a bit of interpretation in order to mimic scrolling.
/// This is a contain for all of the data needed to initialize tracking a series of touch
/// movements, calculating when the user has scrolled far enough, and when a touch as ended.
#[derive(Debug, Default, Clone)]
pub(crate) struct TouchScroll {
    last: Position,
    acc: i32,
}

impl TouchScroll {
    /// The distance needed for the user to scroll in a continuous motion in order to scroll a
    /// single line.
    const SCROLL_THRES: usize = 20;

    /// Constructs a new accumulator.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Initializes a new touch series, resetting any accumulated values.
    pub(crate) fn init_touch(&mut self, event: &Touch) {
        let pos = Position::new(event);
        self.last = pos;
        self.acc = 0;
    }

    /// Adds a new touch to a series and returns the number of scrolls that occurred.
    pub(crate) fn add_touch(&mut self, event: &Touch) -> impl Iterator<Item = ScrollMotion> {
        let pos = Position::new(event);
        // Is this position is reasonable distance from the last one?
        // If so, update the position and acc, reduce the acc, and return an iter
        // If not, ignore this event and return an empty iter
        if self.last.is_connected(pos) {
            self.acc += self.last.y - pos.y;
            // Get the number of scrolls
            let digest = self.acc / Self::SCROLL_THRES as i32;
            let rem = self.acc.abs() % Self::SCROLL_THRES as i32;
            let val = if self.acc > 0 {
                self.acc = rem;
                ScrollMotion::Up
            } else {
                self.acc = -rem;
                ScrollMotion::Down
            };
            self.last = pos;
            std::iter::repeat(val).take(digest.unsigned_abs() as usize)
        } else {
            std::iter::repeat(ScrollMotion::Down).take(0)
        }
    }
}

/// A container for the position at which an event occurred.
#[derive(Debug, Default, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    /// The max distance between two touch positions in order for them to be considered connected.
    const CONNECT_THRES: usize = 200;

    fn new(event: &Touch) -> Self {
        Self {
            x: event.page_x(),
            y: event.page_y(),
        }
    }

    /// Returns if the given position is feasibly part of a connected to this touch.
    fn is_connected(&self, pos: Position) -> bool {
        ((((self.x - pos.x).pow(2) + (self.y - pos.y).pow(2)) as f64).sqrt() as usize)
            <= Self::CONNECT_THRES
    }
}
