// TODO:
//  - Improve the calculations for the character grid.
//  - Explicit set font size, margins, etc (we can't rely on the user defining CSS for us)

use ratatui::{
    buffer::Cell,
    prelude::{Backend, Rect},
    style::{Color, Modifier, Styled},
};
use std::{borrow::Cow, io::Result, marker::PhantomData};
use web_sys::{wasm_bindgen::JsValue, MouseEvent, console};
use yew::{html, Callback, Html};

use crate::palette::{Base16Color, Palette, default::DefaultDark};

/// The backend used to render text to HTML.
/// The backend used to take ratatui widgets and render them into HTML. This is achieved through a
/// three-step rendering process.
///
/// First is the text rendering step. Here, a cell grid is populated from a ratatui rendering. This
/// grid is, in essense, the grid of characters as it will show up in the broswer. Once the text is
/// rendered, it is parsed into spans for use in the second step.
///
/// Second is the hydration step. Ratatui was not meant to run in the browser, so it does not
/// natively support associating callbacks and such with widgets. The hydration process is where
/// that occurs. Certain cell modifiers are used as flags to inform the renderer that additional
/// data might be needed. This provides an opportunity for the app to inject data such as callback
/// into the spans created after the text rendering step.
///
/// Finally, once the data has had a chance to hydrate, it is rendered into HTML, cached, and
/// served.
///
/// From the user's perspective, this process only involves rendering a frame in the Ratatui
/// terminal and then calling `WebTerm::hydrate`. The HTML that is returned from this method is
/// hydrated and ready to serve.
#[derive(Debug)]
pub struct YewBackend<P = DefaultDark> {
    buffer: Vec<Vec<Cell>>,
    pre_hydrated: Vec<Vec<TermSpan>>,
    rendered: Html,
    marker: PhantomData<P>,
}

/// The intermediate representation used for the hydration process.
#[derive(Debug)]
enum TermSpan {
    /// The data is plain data that will be rendered in a styled HTML-span tag.
    Plain((Color, Color), Modifier, String),
    /// The data might need to contain additional data, such as a callback. These will be yielded
    /// to the app for hydration before being rendered into an HTML-span tag.
    Dehydrated(DehydratedSpan),
}

/// A span that might need additional data such as a callback or hyperlink
#[derive(Debug, Default)]
pub struct DehydratedSpan {
    style: (Color, Color),
    mods: Modifier,
    text: String,
    interaction: Interaction,
}

/// A container for the different ways that a span might be interacted with.
#[derive(Debug, Default)]
struct Interaction {
    on_click: Option<Callback<MouseEvent>>,
    hyperlink: Option<String>,
}

impl DehydratedSpan {
    fn new(fg: Color, bg: Color, mods: Modifier, text: String) -> Self {
        Self {
            style: (fg, bg),
            mods,
            text,
            interaction: Interaction::default(),
        }
    }

    /// Returns a reference to the inner style.
    pub fn style(&self) -> &(Color, Color) {
        &self.style
    }

    /// Returns a reference to the inner text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Sets the `on_click` callback for the span.
    pub fn on_click(&mut self, on_click: Callback<MouseEvent>) {
        let _ = self.interaction.on_click.insert(on_click);
    }

    /// Adds a hyperlink to the span.
    pub fn hyperlink(&mut self, link: String) {
        let _ = self.interaction.hyperlink.insert(link);
    }
}

impl Default for YewBackend {
    fn default() -> Self {
        Self::new()
    }
}

const HYDRATION: Modifier = Modifier::SLOW_BLINK;

impl<P> YewBackend<P>
where
    P: Palette,
{
    /// The constructor for the terminal.
    pub fn new() -> Self {
        Self {
            buffer: Self::get_sized_buffer(),
            pre_hydrated: Vec::new(),
            rendered: Html::default(),
            marker: PhantomData,
        }
    }

    fn get_sized_buffer() -> Vec<Vec<Cell>> {
        let (width, height) = if is_mobile() {
            get_screen_size()
        } else {
            get_window_size()
        };
        vec![vec![Cell::default(); width as usize]; height as usize]
    }

    /// The method that renders the temrinal data into HTML.
    pub fn view(&mut self) -> Html {
        self.rendered.clone()
    }

    /// The rendering process is split into three steps.
    fn prerender(&mut self) {
        let Some(cell) = self.buffer.first().and_then(|l| l.first()) else {
            return;
        };

        let mut fg = cell.fg;
        let mut bg = cell.bg;
        let mut mods = cell.modifier;
        for line in self.buffer.iter() {
            let mut text = String::with_capacity(line.len());
            let mut line_buf: Vec<TermSpan> = Vec::new();
            for c in line {
                if fg != c.fg || bg != c.bg || mods != c.modifier {
                    // Create a new node, clear the text buffer, update the foreground/background
                    if !text.is_empty() {
                        let span = if mods.contains(HYDRATION) {
                            TermSpan::Dehydrated(DehydratedSpan::new(fg, bg, mods, text.to_owned()))
                        } else {
                            TermSpan::Plain((fg, bg), mods, text.to_owned())
                        };
                        line_buf.push(span);
                    }
                    mods = c.modifier;
                    fg = c.fg;
                    bg = c.bg;
                    text.clear();
                }
                text.push_str(c.symbol())
            }
            // Create a new node, combine into a `pre` tag, push onto buf
            if !text.is_empty() {
                let span = if mods.contains(HYDRATION) {
                    TermSpan::Dehydrated(DehydratedSpan::new(fg, bg, mods, text.to_owned()))
                } else {
                    TermSpan::Plain((fg, bg), mods, text.to_owned())
                };
                line_buf.push(span);
            }
            self.pre_hydrated.push(line_buf);
        }
    }

    pub fn hydrate<F>(&mut self, mut hydrator: F) -> Html
    where
        F: FnMut(&mut DehydratedSpan),
    {
        let mut buffer: Vec<Html> = Vec::with_capacity(self.pre_hydrated.len());
        for line in self.pre_hydrated.drain(0..) {
            let mut inner: Vec<Html> = Vec::with_capacity(line.len());
            for span in line {
                match span {
                    TermSpan::Plain((fg, bg), mods, text) => {
                        inner.push(create_span::<P>(fg, bg, mods, &text))
                    }
                    TermSpan::Dehydrated(mut span) => {
                        hydrator(&mut span);
                        let DehydratedSpan {
                            style: (fg, bg),
                            text,
                            interaction,
                            mods,
                        } = span;
                        let Interaction {
                            on_click,
                            hyperlink,
                        } = interaction;
                        let mut element = create_span_with_callback::<P>(fg, bg, mods, &text, on_click);
                        if let Some(link) = hyperlink {
                            element = html! { <a href = { link } target = "_blank" style="text-decoration:none"> { element } </a> };
                        }
                        inner.push(element);
                    }
                }
            }
            buffer.push(html! { <pre> { for inner.drain(0..) } </pre> })
        }
        html! { <div> { for buffer.into_iter() } </div> }
    }

    pub fn resize_buffer(&mut self) {
        let (width, height) = if is_mobile() {
            get_screen_size()
        } else {
            get_window_size()
        };
        if self.buffer.len() != height as usize || self.buffer[0].len() != width as usize {
            // Reset the buffer only if the size is actually different
            self.buffer = Self::get_sized_buffer();
        }
    }
}

impl Backend for YewBackend {
    fn draw<'a, I>(&mut self, content: I) -> Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        for (x, y, cell) in content {
            let y = y as usize;
            let x = x as usize;
            let line = &mut self.buffer[y];
            line.extend(std::iter::repeat_with(Cell::default).take(x.saturating_sub(line.len())));
            line[x] = cell.clone();
        }
        Ok(())
    }

    fn hide_cursor(&mut self) -> Result<()> {
        Ok(())
    }

    fn show_cursor(&mut self) -> Result<()> {
        todo!()
    }

    fn get_cursor(&mut self) -> Result<(u16, u16)> {
        todo!()
    }

    fn set_cursor(&mut self, _x: u16, _y: u16) -> Result<()> {
        todo!()
    }

    fn clear(&mut self) -> Result<()> {
        self.buffer = Self::get_sized_buffer();
        Ok(())
    }

    fn size(&self) -> Result<Rect> {
        Ok(Rect::new(
            0,
            0,
            self.buffer.first().unwrap().len().saturating_sub(1) as u16,
            self.buffer.len().saturating_sub(1) as u16,
        ))
    }

    fn window_size(&mut self) -> Result<ratatui::backend::WindowSize> {
        todo!()
    }

    fn flush(&mut self) -> Result<()> {
        self.prerender();
        Ok(())
    }
}

fn create_span<P: Palette>(fg: Color, bg: Color, mods: Modifier, text: &str) -> Html {
    create_span_with_callback::<P>(fg, bg, mods, text, None)
}

fn create_span_with_callback<P: Palette>(
    fg: Color,
    bg: Color,
    mods: Modifier,
    text: &str,
    cb: Option<Callback<MouseEvent>>,
) -> Html {
    console::log_1(&format!("Foreground: {fg:?}").into());
    let fg = to_css_color::<P>(fg).unwrap_or_default();
    console::log_1(&format!("Foreground: {fg:?}").into());
    console::log_1(&format!("Background: {bg:?}").into());
    let bg = to_css_color::<P>(bg).unwrap_or_default();
    console::log_1(&format!("Background: {bg:?}").into());
    let mut style = format!("color: {fg}; background-color: {bg};");
    extend_css(mods, &mut style);
    console::log_1(&format!("Style: {style:?}").into());
    match cb {
        Some(cb) => html! { <span style={ style } onclick = { cb }> { text } </span> },
        None => html! { <span style={ style }> { text } </span> },
    }
}

fn to_css_color<P: Palette>(c: Color) -> Option<Cow<'static, str>> {
    match c {
        Color::Reset => None,
        Color::Black => Some("black".into()),
        Color::Red => Some("red".into()),
        Color::Green => Some("green".into()),
        Color::Yellow => Some("yellow".into()),
        Color::Blue => Some("blue".into()),
        Color::Magenta => Some("magenta".into()),
        Color::Cyan => Some("cyan".into()),
        Color::Gray => Some("gray".into()),
        Color::DarkGray => Some("darkgray".into()),
        Color::LightRed => Some("#de2b56".into()),
        Color::LightGreen => Some("lightgreen".into()),
        Color::LightYellow => Some("LightGoldenRodYellow".into()),
        Color::LightBlue => Some("LightSkyBlue".into()),
        Color::LightMagenta => Some("#ff00ff".into()),
        Color::LightCyan => Some("lightcyan".into()),
        Color::White => Some("white".into()),
        Color::Rgb(r, g, b) => Some(format!("#{r:X}{g:X}{b:X}").into()),
        Color::Indexed(i) => Some(P::to_rgb_str(Base16Color::from_index(i).into()).into()),
    }
}

/// Calculates the number of characters that can fit in the window.
pub fn get_window_size() -> (u16, u16) {
    let (w, h) = get_raw_window_size();
    // These are mildly magical numbers... make them more precise
    (w / 10, h / 20)
}

/*
/// Calculates the number of characters that can fit in the Ratatui buffer.
pub fn get_max_window_size() -> (u16, u16) {
    let (w, h) = get_raw_window_size();
    (w / 10, u16::MAX / ( w / 10 ))
}
*/

pub fn get_raw_window_size() -> (u16, u16) {
    fn js_val_to_int<I: TryFrom<usize>>(val: JsValue) -> Option<I> {
        val.as_f64().and_then(|i| I::try_from(i as usize).ok())
    }

    web_sys::window()
        .and_then(|s| {
            s.inner_width()
                .ok()
                .and_then(js_val_to_int::<u16>)
                .zip(s.inner_height().ok().and_then(js_val_to_int::<u16>))
        })
        .unwrap_or((120, 120))
}

pub fn get_raw_screen_size() -> (i32, i32) {
    let s = web_sys::window().unwrap().screen().unwrap();
    (s.width().unwrap(), s.height().unwrap())
}

/// Calculates the number of characters that can fit in the window.
pub fn get_screen_size() -> (u16, u16) {
    let (w, h) = get_raw_screen_size();
    // These are mildly magical numbers... make them more precise
    (w as u16 / 10, h as u16 / 19)
}

/// An abstraction to allow for method chain to mark a something as hydratable
pub trait NeedsHydration: Sized + Styled {
    fn to_hydrate(self) -> Self::Item {
        let style = self.style().add_modifier(HYDRATION);
        self.set_style(style)
    }
}

impl<T> NeedsHydration for T where T: Styled {}

/// Extends a CSS style string to include the necessary segments for the current modifiers.
fn extend_css(mods: Modifier, css: &mut String) {
    if mods.contains(Modifier::BOLD) {
        css.push_str(" font-weight: bolder;");
    }
    if mods.contains(Modifier::ITALIC) {
        css.push_str(" font-style: oblique;");
    }

    if mods.contains(Modifier::UNDERLINED) {
        css.push_str(" text-decoration: underline;");
    }
}

// TODO: Improve this...
pub fn is_mobile() -> bool {
    get_raw_screen_size().0 < 550
}
