#![allow(dead_code)]

// Design goals:
//  - Some users want a single-page, plug-and-play expirence. For them, it should be as easy as
//  possible to port their existing TUI app to a web app
//  - Some users want the full flexiblity of Yew, specifically the use of the Yew router.
//  - Users should have access to the temrinal renderer... somehow...
//  -

// Needed:
//  - Type that implements Component and wraps a user's type
//  - Trait for users to implement that make the wrapper component a component
//  - Basic web terminal
//  - Add scroll processing for inner apps
//  - Find an easy way for users to get callbacks specific to their message type

use std::cell::RefCell;

use backend::{DehydratedSpan, YewBackend};
use ratatui::{prelude::Rect, Frame, Terminal};
use yew::{Component, Context, Properties};

pub mod backend;

// TODO: Needs to set up callbacks to resize backend's buffers
/// A container for a TUI app that renders to HTML.
pub struct WebTerminal<A> {
    app: A,
    term: RefCell<Terminal<YewBackend>>,
}

/// In the public API because of the component impl of WebTerminal
pub enum WebTermMessage<M> {
    Inner(M),
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

    // TODO: Add (optional) scroll and resize methods

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
        // TODO:
        //  - Set scroll callbacks (for both standard scroll and touch scroll)
        //  - Set resize callback
        Self { app, term }
    }

    #[allow(unused_variables)]
    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            WebTermMessage::Inner(msg) => self.app.update(ctx, msg),
        }
    }

    #[allow(unused_variables)]
    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let mut term = self.term.borrow_mut();
        let area = term.size().unwrap();
        term.draw(|frame| self.app.render(area, frame)).unwrap();
        term.backend_mut().hydrate(|span| self.app.hydrate(ctx, span))
    }
}
