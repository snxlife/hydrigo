
/// get the render width of a char
/// 
/// # Examples
/// 
/// ```
/// use hydrigo::util::get_render_width;
/// 
/// assert_eq!(0, get_render_width('\u{0000}'));
/// assert_eq!(0, get_render_width('\r'));
/// assert_eq!(1, get_render_width(' '));
/// assert_eq!(1, get_render_width('H'));
/// assert_eq!(2, get_render_width('你'));
/// ```
pub fn get_render_width(ch: char) -> usize {
    match ch {
        '\u{0800}'..='\u{9fa5}' |
        '\u{ac00}'..='\u{d7ff}' => 2,
        '\u{0000}' | '\r'       => 0,
        _                       => 1,
    }
}