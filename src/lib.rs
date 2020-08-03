//! Utilities for drawing in ascii for Rust applications.
//!
//! # Quick Example:
//! ```
//! use ascii_graphics::*;
//! let mut window = screen::create(10, 10);
//! window
//!     .background(' ')
//!     .border(border::settings('+', '-', '|'))
//!     .stroke('0')
//!     .line(2, 6, 6, 2)
//!     .text("hello", 2, 8)
//!		.print();
//! // +--------+
//! // |        |
//! // |     0  |
//! // |    0   |
//! // |   0    |
//! // |  0     |
//! // | 0      |
//! // |        |
//! // | hello  |
//! // +--------+
//! ```

/// Customize the border.
pub mod border;

/// Main module for drawing items.
pub mod screen;
