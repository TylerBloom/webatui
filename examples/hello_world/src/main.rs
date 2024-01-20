use ratatui::{prelude::*, widgets::Paragraph};
use webatui::prelude::*;
use yew::prelude::*;

fn main() {
    WebTerminal::<HelloWorld>::render()
}

#[derive(Clone, Default, PartialEq)]
struct HelloWorld;

impl TerminalApp for HelloWorld {
    // This is a static app, so there's no need for a message type
    type Message = ();

    // Yew is message-based (reactive), but this is a static example, so this method is not needed
    fn update(&mut self, _ctx: &Context<WebTerminal<Self>>, _msg: Self::Message) -> bool {
        false
    }

    // Put your existing rendering logic here.
    fn render(&self, area: Rect, frame: &mut Frame<'_>) {
        let para = Paragraph::new("Hello World!");
        frame.render_widget(para, area);
    }
}
