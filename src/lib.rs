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

use std::cell::RefCell;

use backend::{YewBackend, DehydratedSpan};
use ratatui::{Terminal, prelude::Rect, Frame};
use yew::{Component, Properties};

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

/// In the public API because of the component impl of WebTerminal
#[derive(Properties, PartialEq)]
pub struct WebTermProps<M: PartialEq> {
    inner: M,
}

/// The core user-facing abstraction of this crate. A terminal app is a type that can be wrapped by
/// a [`WebTerminal`] and be displayed by Yew.
pub trait TerminalApp: 'static + Clone + PartialEq {
    /// The message type that this type uses to update.
    type Message;

    /// Allows the app to initialize its environment, such as setting up callbacks to window
    /// events.
    fn setup(&mut self) {}

    /// Updates the app with a message.
    fn update(&mut self, msg: Self::Message) -> bool;

    /// Takes a Ratatui [`Frame`] and renders widgets onto it.
    fn render(&self, area: Rect, frame: &mut Frame);

    /// Takes a dehydrated spans from the backend and hydrates them by adding callbacks,
    /// hyperlinks, etc.
    #[allow(unused_variables)]
    fn hydrate(&self, span: &mut DehydratedSpan) {}
}

impl<A: TerminalApp> Component for WebTerminal<A> {
    type Message = WebTermMessage<A::Message>;
    type Properties = WebTermProps<A>;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let mut app = ctx.props().inner.clone();
        app.setup();
        let term = RefCell::new(Terminal::new(YewBackend::new()).unwrap());
        Self { app, term }
    }

    #[allow(unused_variables)]
    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            WebTermMessage::Inner(msg) => self.app.update(msg),
        }
    }

    #[allow(unused_variables)]
    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let mut term = self.term.borrow_mut();
        let area = term.size().unwrap();
        term.draw(|frame| self.app.render(area, frame)).unwrap();
        term.backend_mut().hydrate(|span| self.app.hydrate(span))
    }
}
