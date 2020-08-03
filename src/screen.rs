use std::ops::{Index, IndexMut, Shl};
use std::process::Command;
use std::{thread, time};

use crate::border;

/// A struct to store drawing and buffer information.
pub struct Screen {
    /// the width of the screen
    pub width: u32,
    /// the height of the screen
    pub height: u32,
    buffer: Vec<char>,
    fill_value: char,
    fill: bool,
    stroke: bool,
    stroke_value: char,
    update_function: Option<fn(&mut Screen, u64) -> bool>,
}

/// Creates a new empty `Screen` of with a width and height.
///
/// # Examples
///
/// Basic usage:
/// ```
/// # use ascii_graphics::screen;
/// let mut window = screen::create(10, 10);
///
/// assert_eq!(window.width, 10);
/// assert_eq!(window.height, 10);
/// ```
pub fn create(width: u32, height: u32) -> Screen {
    Screen::new(width, height, ' ')
}

/// Clears the terminal screen.
///
/// # Examples
///
/// ```
/// # use ascii_graphics::screen;
/// screen::clear();
/// ```
pub fn clear() {
    match if cfg!(target_os = "windows") {
        Command::new("cls")
    } else {
        Command::new("clear")
    }
    .spawn()
    .expect("clear failed")
    .wait()
    {
        Err(x) => println!("{:?}", x),
        _ => {}
    }
}

impl Screen {
    fn new(width: u32, height: u32, default: char) -> Screen {
        Screen {
            width: width,
            height: height,
            buffer: (0..(width * height)).map(|_x| default).collect(),
            fill: false,
            fill_value: ' ',
            stroke: false,
            stroke_value: ' ',
            update_function: None,
        }
    }

    /// Clears the screen and prints the current buffer to stdout.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii_graphics::screen;
    /// # let mut window = screen::create(10, 10);
    /// window.print();
    /// ```
    pub fn print(&self) {
        clear();
        for line in self.buffer.chunks(self.width as usize) {
            println!("{}", line.iter().collect::<String>());
        }
    }

    /// Sets the fill value for future draws.
    ///
    /// # Examples
    ///
    /// ```
    /// use ascii_graphics::screen;
    /// # let mut window = screen::create(10, 10);
    /// window.fill('*');
    /// ```
    pub fn fill(&mut self, value: char) -> &mut Screen {
        self.fill_value = value;
        self.fill = true;
        self
    }

    /// Removes fill for future draws.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii_graphics::screen;
    /// # let mut window = screen::create(10, 10);
    /// window.no_fill();
    /// ```
    pub fn no_fill(&mut self) -> &mut Screen {
        self.fill = false;
        self
    }

    /// Sets the stroke value for future draws.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii_graphics::screen;
    /// # let mut window = screen::create(10, 10);
    /// window.stroke('*');
    /// ```
    pub fn stroke(&mut self, value: char) -> &mut Screen {
        self.stroke_value = value;
        self.stroke = true;
        self
    }

    /// Sets the stroke value for future draws.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii_graphics::screen;
    /// # let mut window = screen::create(10, 10);
    /// window.no_stroke();
    /// ```
    pub fn no_stroke(&mut self) -> &mut Screen {
        self.stroke = false;
        self
    }

    /// Sets the entire buffer to the value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii_graphics::screen;
    /// # let mut window = screen::create(4, 4);
    /// window.background('#');
    /// // ####
    /// // ####
    /// // ####
    /// // ####
    /// ```
    pub fn background(&mut self, value: char) -> &mut Screen {
        let (width, height) = (self.width, self.height);
        for x in 0..width {
            for y in 0..height {
                self[(x, y)] = value;
            }
        }

        self
    }

    /// Outlines the screen buffer with the value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii_graphics::screen;
    /// # let mut window = screen::create(5, 5);
    /// window.soild_border('*');
    /// // *****
    /// // *   *
    /// // *   *
    /// // *   *
    /// // *****
    /// ```
    pub fn soild_border(&mut self, value: char) -> &mut Screen {
        let (width, height) = (self.width, self.height);
        for x in 0..width {
            self[(x, 0)] = value;
            self[(x, height - 1)] = value;
        }

        for y in 0..height {
            self[(0, y)] = value;
            self[(width - 1, y)] = value;
        }

        self
    }

    /// Outlines the screen buffer with the specified settings.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii_graphics::{screen, border};
    /// # let mut window = screen::create(5, 5);
    /// window.border(border::settings('+', '-', '|'));
    /// // +---+
    /// // |   |
    /// // |   |
    /// // |   |
    /// // +---+
    /// ```
    pub fn border(&mut self, settings: border::Settings) -> &mut Screen {
        let (width, height) = (self.width, self.height);

        for x in 0..width {
            self[(x, 0)] = settings.top;
            self[(x, height - 1)] = settings.bottom;
        }

        for y in 0..height {
            self[(0, y)] = settings.left;
            self[(width - 1, y)] = settings.right;
        }

        self[(0, 0)] = settings.corners;
        self[(0, height - 1)] = settings.corners;
        self[(width - 1, 0)] = settings.corners;
        self[(width - 1, height - 1)] = settings.corners;

        self
    }

    /// Writes the value starting at (x, y). Panics if x + value.len() is greater than the width.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii_graphics::screen;
    /// # let mut window = screen::create(5, 5);
    /// window.background('-').text("hello", 0, 3);
    /// // -----
    /// // -----
    /// // -----
    /// // hello
    /// // -----
    /// ```
    pub fn text(&mut self, value: &str, x: u32, y: u32) -> &mut Screen {
        let width = self.width;

        let mut iter = value.chars();

        for i in x..width {
            match iter.next() {
                Some(c) => self[(i, y)] = c,
                None => break,
            }
        }

        self
    }

    /// Draws a rectangle centered at (x, y) with (width, height). Uses the fill and stroke values when applicable.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii_graphics::screen;
    /// # let mut window = screen::create(5, 5);
    /// window
    ///     .background('-')
    ///     .fill('#')
    ///     .stroke('*')
    ///     .rect(2, 2, 2, 2);
    /// // -----
    /// // -***-
    /// // -*#*-
    /// // -***-
    /// // -----
    /// ```
    pub fn rect(&mut self, x: u32, y: u32, width: u32, height: u32) -> &mut Screen {
        let (half_w, half_h) = (width / 2, height / 2);

        if self.fill {
            for i in (x - half_w)..(x + half_w + 1) {
                for j in (y - half_h)..(y + half_h + 1) {
                    self[(i, j)] = self.fill_value;
                }
            }
        }

        if self.stroke {
            for i in (x - half_w)..(x + half_w + 1) {
                self[(i, y - half_h)] = self.stroke_value;
                self[(i, y + half_h)] = self.stroke_value;
            }
            for j in (y - half_h)..(y + half_h + 1) {
                self[(x - half_w, j)] = self.stroke_value;
                self[(x + half_w, j)] = self.stroke_value;
            }
        }

        self
    }

    /// Draws a line from (x1, y1) to (x2, y2) if stroke has been set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii_graphics::screen;
    /// # let mut window = screen::create(5, 5);
    /// window.stroke('*').line(0, 1, 3, 4);
    /// //
    /// // *
    /// //  *
    /// //   *
    /// //    *
    /// ```
    pub fn line(&mut self, x1: u32, y1: u32, x2: u32, y2: u32) -> &mut Screen {
        if self.stroke {
            let dx: f32 = x2 as f32 - x1 as f32;
            let dy: f32 = y2 as f32 - y1 as f32;

            let slope: f32 = dy / dx;

            let mut x: u32 = std::cmp::min(x1, x2);
            let max = std::cmp::max(x1, x2);
            let mut y: f32 = if slope > 0.0 {
                std::cmp::min(y1, y2)
            } else {
                std::cmp::max(y1, y2)
            } as f32;

            while x <= max {
                self[(x, y.round() as u32)] = self.stroke_value;
                x += 1;
                y += slope;
            }
        }
        self
    }

    /// Draws a rough circle centered at (x, y) with the radius.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii_graphics::screen;
    /// # let mut window = screen::create(10, 10);
    /// window.circle(5, 5, 3);
    /// ```
    pub fn circle(&mut self, x: u32, y: u32, radius: u32) -> &mut Screen {
        let r2 = radius as i64 * radius as i64;
        for i in (x - radius)..(x + radius + 1) {
            for j in (y - radius)..(y + radius + 1) {
                let dx: i64 = i as i64 - x as i64;
                let dy: i64 = j as i64 - y as i64;

                let dist = dx * dx + dy * dy;
                if dist < r2 && self.fill {
                    self[(i, j)] = self.fill_value;
                } else if (dist == r2 || dist == r2 + 1) && self.stroke {
                    self[(i, j)] = self.stroke_value;
                }
            }
        }
        self
    }

    // pub fn triangle(&mut self, x1: u32, y1: u32, x2: u32, y2: u32) {
    //     let miny = std::cmp::min(std::cmp::min(y1, y2), y3);
    //     let maxy = std::cmp::max(std::cmp::max(y1, y2), y3);

    //     let minx = std::cmp::min(std::cmp::min(x1, x2), x3);
    //     let maxx = std::cmp::max(std::cmp::max(x1, x2), x3);
    // }

    /// Sets the update function for the screen. The function must take a mutable screen and a frame number.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ascii_graphics::{screen, border};
    /// # let mut window = screen::create(5, 5);
    /// let update = |window: &mut screen::Screen, frame: u64| {
    ///     if frame > 99 {
    ///         false
    ///     } else {
    ///         window
    ///             .background(' ')
    ///             .border(border::settings('+', '-', '|'))
    ///             .text(&format!("{}", frame), 3, 3);
    ///         true
    ///     }
    /// };
    /// window.set_update(update);
    /// ```
    pub fn set_update(&mut self, update: fn(&mut Screen, u64) -> bool) {
        self.update_function = Some(update);
    }

    /// Starts an animation at a rate of fps. It uses the update function for each frame and will end if the update function returns false.
    pub fn run(&mut self, fps: u64) {
        let delay = time::Duration::from_millis(1000 / fps);
        let mut count = 0;

        loop {
            if !self.update(count) {
                break;
            }
            self.print();
            count += 1;
            thread::sleep(delay);
        }
    }

    fn update(&mut self, frame: u64) -> bool {
        match self.update_function {
            Some(x) => x(self, frame),
            None => false,
        }
    }
}

/// Index a Screen struct by a tuple of (x, y).
///
/// # Examples
///
/// Basic Usage
/// ```
/// # use ascii_graphics::screen;
/// # let window = screen::create(5, 5);
/// let x = window[(0, 0)];
///
/// assert_eq!(x, ' ');
/// ```
impl Index<(u32, u32)> for Screen {
    type Output = char;
    fn index(&self, index: (u32, u32)) -> &Self::Output {
        if index.0 >= self.width || index.1 >= self.height {
            panic!(
                "index out of bounds: the size is ({}, {}) but got ({}, {})",
                self.width, self.height, index.0, index.1
            );
        }
        &self.buffer[(index.1 * self.width + index.0) as usize]
    }
}

/// Index a mutable Screen struct by a tuple of (x, y).
/// Panics if x or y is out of bounds.
///
/// # Examples
///
/// Basic Usage
/// ```
/// # use ascii_graphics::screen;
/// # let mut window = screen::create(5, 5);
/// window[(3, 2)] = 'x';
///
/// assert_eq!(window[(3, 2)], 'x');
///
/// window[(3, 2)] = 'y';
/// assert_eq!(window[(3, 2)], 'y');
/// ```
impl IndexMut<(u32, u32)> for Screen {
    fn index_mut(&mut self, index: (u32, u32)) -> &mut Self::Output {
        if index.0 >= self.width || index.1 >= self.height {
            panic!(
                "index out of bounds: the size is ({}, {}) but got ({}, {})",
                self.width, self.height, index.0, index.1
            );
        }
        &mut self.buffer[(index.1 * self.width + index.0) as usize]
    }
}

/// Shifts the screen buffer left by some number of pixels.
///
/// # Examples
///
/// Basic usage:
/// ```
/// # use ascii_graphics::screen;
/// let mut window = screen::create(10, 10);
/// window.soild_border('*');
/// window = window << 1;
/// ```
impl Shl<u32> for Screen {
    type Output = Self;

    fn shl(mut self, value: u32) -> Self::Output {
        for x in 0..self.width - value {
            for y in 0..self.height {
                let temp = self[(x + value, y)];
                self[(x + value, y)] = self[(x, y)];
                self[(x, y)] = temp;
            }
        }
        self
    }
}

/// Create a window from a width and height tuple.
///
/// # Examples
///
/// Basic usage:
/// ```
/// # use ascii_graphics::screen;
/// let mut window: screen::Screen = (10, 20).into();
/// ```
impl From<(u32, u32)> for Screen {
    fn from((width, height): (u32, u32)) -> Self {
        Screen::new(width, height, ' ')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let screen = Screen::new(10, 10, ' ');

        assert_eq!(screen.width, 10);
        assert_eq!(screen.height, 10);
        assert_eq!(screen.buffer.capacity(), 100);
    }

    #[test]
    fn indexing() {
        let mut screen = Screen::new(5, 5, '*');
        screen[(0, 0)] = 'a';
        screen[(1, 4)] = 'b';
        assert_eq!(screen[(0, 0)], 'a');
        assert_eq!(screen[(1, 4)], 'b');
    }
}
