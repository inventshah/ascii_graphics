/// Stores values for a border.
///
/// # Examples
///
/// ```
/// # use ascii_graphics::border;
/// let mut settings = border::Settings {
///     corners: '+',
///     top: '-',
///     left: '|',
///     right: '|',
///     bottom: '-'
/// };
/// ```
// #[derive(Debug)]
pub struct Settings {
    pub corners: char,
    pub top: char,
    pub left: char,
    pub bottom: char,
    pub right: char,
}

/// Wrapper function for constructing border settings.
///
/// # Examples
///
/// ```
/// # use ascii_graphics::border;
/// let settings = border::full_settings('+', '-', '-', '|', '|');
/// ```
pub fn full_settings(corners: char, top: char, bottom: char, left: char, right: char) -> Settings {
    Settings {
        corners: corners,
        top: top,
        left: left,
        bottom: bottom,
        right: right,
    }
}

/// Wrapper function for constructing border settings with the top/bottom and left/right being the same.
///
/// # Examples
///
/// ```
/// # use ascii_graphics::border;
/// let mut settings = border::settings('+', '-', '-');
/// ```
pub fn settings(corners: char, horizontal: char, vertical: char) -> Settings {
    Settings {
        corners: corners,
        top: horizontal,
        left: vertical,
        bottom: horizontal,
        right: vertical,
    }
}
