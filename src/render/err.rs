use super::util::{center_truncate, clear_screen, get_terminal_width};

pub fn render_error(err: String) {
    let width = get_terminal_width();
    clear_screen();
    println!("Error\n{}", center_truncate(&err, width));
}
