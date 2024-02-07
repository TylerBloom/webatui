use base16_palettes::{Base16Accent, Base16Color};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use webatui::prelude::*;
use yew::prelude::*;

fn main() {
    run_tui(Counter(0));
}

#[derive(Clone, Default, PartialEq)]
struct Counter(usize);

#[derive(Clone, Default, PartialEq)]
enum CounterMsg {
    #[default]
    Inc,
    Dec,
}

impl TerminalApp for Counter {
    // This is a static app, so there's no need for a message type
    type Message = CounterMsg;

    // Yew is message-based (reactive), but this is a static example, so this method is not needed
    fn update(&mut self, _ctx: TermContext<'_, Self>, msg: Self::Message) -> bool {
        match msg {
            CounterMsg::Inc => self.0 += 1,
            CounterMsg::Dec => self.0 = self.0.saturating_sub(1),
        }
        true
    }

    // Put your existing rendering logic here.
    fn render(&self, mut area: Rect, frame: &mut Frame<'_>) {
        area.height = 3;
        let areas = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Length(4),
                Constraint::Length(4),
                Constraint::Min(10),
            ],
        )
        .split(area);
        let button_style = Style::new()
            .fg(Base16Accent::Accent01.to_color())
            .to_hydrate();
        let para = Paragraph::new("+1").set_style(button_style).block(
            Block::default()
                .style(Base16Color::default_style())
                .borders(Borders::ALL),
        );
        frame.render_widget(para, areas[0]);
        let para = Paragraph::new("-1").set_style(button_style).block(
            Block::new()
                .style(Base16Color::default_style())
                .borders(Borders::ALL),
        );
        frame.render_widget(para, areas[1]);
        let para = Paragraph::new(format!("count: {}", self.0)).block(
            Block::new()
                .style(Base16Color::default_style())
                .borders(Borders::ALL),
        );
        frame.render_widget(para, areas[2]);
    }

    fn hydrate(&self, ctx: &Context<WebTerminal<Self>>, span: &mut DehydratedSpan) {
        match span.text() {
            "+1" => span.on_click(ctx.link().callback(|_| CounterMsg::Inc)),
            "-1" => span.on_click(ctx.link().callback(|_| CounterMsg::Dec)),
            _ => {}
        }
    }
}
