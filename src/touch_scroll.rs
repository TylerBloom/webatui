#![allow(dead_code, unused)]

use web_sys::{Touch, TouchEvent};
use yew::platform::spawn_local;

use crate::ScrollMotion;

/// Touch events (for mobile) are not emitted from the browser in a continuous stream. As such,
/// they are not tightly linked and require a bit of interpretation in order to mimic scrolling.
/// This is a contain for all of the data needed to initialize tracking a series of touch
/// movements, calculating when the user has scrolled far enough, and when a touch as ended.
#[derive(Debug, Default, Clone)]
pub struct TouchScroll {
    last: Position,
    acc: i32,
}

impl TouchScroll {
    /// The distance needed for the user to scroll in a continuous motion in order to scroll a
    /// single line.
    const SCROLL_THRES: usize = 20;

    /// Constructs a new accumulator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Initializes a new touch series, resetting any accumulated values.
    pub fn init_touch(&mut self, event: &Touch) {
        let pos = Position::new(event);
        self.last = pos;
        self.acc = 0;
    }

    /// Adds a new touch to a series and returns the number of scrolls that occurred.
    pub fn add_touch(&mut self, event: &Touch) -> impl Iterator<Item = ScrollMotion> {
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
