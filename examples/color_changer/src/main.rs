use base16_palettes::{
    palettes::{
        DefaultDark, DefaultLight, DefaultPalette, GruvboxDarkHard, GruvboxLightSoft,
        GruvboxPalette,
    },
    Base16Accent, Base16Color, Base16Shade, Palette, Shade,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Tabs},
};
use webatui::prelude::*;
use yew::prelude::*;

fn main() {
    WebTerminal::<PaletteSelector>::render()
}

#[derive(Clone, Default, PartialEq)]
struct PaletteSelector;

impl TerminalApp for PaletteSelector {
    // This is a static app, so there's no need for a message type
    type Message = Palette;

    // Yew is message-based (reactive), but this is a static example, so this method is not needed
    fn update(&mut self, mut ctx: TermContext<'_, Self>, msg: Self::Message) -> bool {
        ctx.terminal().backend_mut().update_palette(msg);
        true
    }

    // Put your existing rendering logic here.
    fn render(&self, area: Rect, frame: &mut Frame<'_>) {
        let areas = Layout::new(
            Direction::Vertical,
            [Constraint::Length(3), Constraint::Min(3)],
        )
        .split(area);
        let style = Style::new().to_hydrate();
        let tabs = Tabs::new(vec![
            Line::styled("Base16 Default Dark", style),
            Line::styled("Base16 Default Light", style),
            Line::styled("Gruvbox Dark Hard", style),
            Line::styled("Gruvbox Light Soft", style),
        ])
        .block(
            Block::new()
                .borders(Borders::ALL)
                .style(Base16Color::default_style())
                .title(" Select theme "),
        );
        frame.render_widget(tabs, areas[0]);
        let para = Paragraph::new(vec![
            Line::styled(
                "Shade #1",
                Style::new().fg(Base16Shade::Dark(Shade::Darkest).to_color()),
            ),
            Line::styled(
                "Shade #2",
                Style::new().fg(Base16Shade::Dark(Shade::Darker).to_color()),
            ),
            Line::styled(
                "Shade #3",
                Style::new().fg(Base16Shade::Dark(Shade::Lighter).to_color()),
            ),
            Line::styled(
                "Shade #4",
                Style::new().fg(Base16Shade::Dark(Shade::Lightest).to_color()),
            ),
            Line::styled(
                "Shade #5",
                Style::new().fg(Base16Shade::Light(Shade::Darkest).to_color()),
            ),
            Line::styled(
                "Shade #6",
                Style::new().fg(Base16Shade::Light(Shade::Darker).to_color()),
            ),
            Line::styled(
                "Shade #7",
                Style::new().fg(Base16Shade::Light(Shade::Lighter).to_color()),
            ),
            Line::styled(
                "Shade #8",
                Style::new().fg(Base16Shade::Light(Shade::Lightest).to_color()),
            ),
            Line::styled(
                "Accent #1",
                Style::new().fg(Base16Accent::Accent00.to_color()),
            ),
            Line::styled(
                "Accent #2",
                Style::new().fg(Base16Accent::Accent01.to_color()),
            ),
            Line::styled(
                "Accent #3",
                Style::new().fg(Base16Accent::Accent02.to_color()),
            ),
            Line::styled(
                "Accent #4",
                Style::new().fg(Base16Accent::Accent03.to_color()),
            ),
            Line::styled(
                "Accent #5",
                Style::new().fg(Base16Accent::Accent04.to_color()),
            ),
            Line::styled(
                "Accent #6",
                Style::new().fg(Base16Accent::Accent05.to_color()),
            ),
            Line::styled(
                "Accent #7",
                Style::new().fg(Base16Accent::Accent06.to_color()),
            ),
            Line::styled(
                "Accent #8",
                Style::new().fg(Base16Accent::Accent07.to_color()),
            ),
        ]);
        frame.render_widget(para, areas[1]);
    }

    fn hydrate(&self, ctx: &Context<WebTerminal<Self>>, span: &mut DehydratedSpan) {
        match span.text() {
            "Base16 Default Dark" => span.on_click(
                ctx.link()
                    .callback(|_| Palette::DefaultPalette(DefaultPalette::DefaultDark(DefaultDark))),
            ),
            "Base16 Default Light" => span.on_click(
                ctx.link()
                    .callback(|_| Palette::DefaultPalette(DefaultPalette::DefaultLight(DefaultLight))),
            ),
            "Gruvbox Dark Hard" => span.on_click(
                ctx.link()
                    .callback(|_| Palette::GruvboxPalette(GruvboxPalette::GruvboxDarkHard(GruvboxDarkHard))),
            ),
            "Gruvbox Light Soft" => span.on_click(
                ctx.link()
                    .callback(|_| Palette::GruvboxPalette(GruvboxPalette::GruvboxLightSoft(GruvboxLightSoft))),
            ),
            _ => {}
        }
    }
}
