use std::io::stdout;

use crossterm::{cursor, execute, terminal};
use gitlab::StatusState;

use crate::emoji::*;

pub fn clear_screen() {
    execute!(stdout(), terminal::Clear(terminal::ClearType::All))
        .expect("Your terminal does not support clearing terminals.");
    execute!(stdout(), cursor::MoveTo(0, 0))
        .expect("Your terminal does not support moving cursors.");
}

fn flip<T: Clone>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let height = v.len();
    let width = v[0].len();
    let mut res = Vec::new();
    for i in 0..width {
        let mut inner = Vec::new();
        for j in 0..height {
            inner.push(v[j][i].clone());
        }
        res.push(inner)
    }
    res
}

pub fn duration_to_string(d: f64) -> String {
    let mut parts = Vec::new();
    let duration_seconds: u32 = d as u32;
    if duration_seconds == 0 {
        return "not startet yet".to_string();
    }
    let hours = duration_seconds / 3600;
    let minutes = (duration_seconds % 3600) / 60;
    let seconds = duration_seconds % 60;
    match hours {
        0 => (),
        1 => parts.push("1 hour".to_string()),
        n => parts.push(format!("{} hours", n)),
    }
    match minutes {
        0 => (),
        1 => parts.push("1 minute".to_string()),
        n => parts.push(format!("{} minutes", n)),
    }
    match seconds {
        0 => (),
        1 => parts.push("1 second".to_string()),
        n => parts.push(format!("{} seconds", n)),
    }

    parts.join(" ")
}

pub fn center_truncate(text: &String, width: usize) -> String {
    let mut text = text.emoji_truncate(width);
    text.truncate(width);
    let spaces_left = (width - text.emoji_len()) / 2;
    text = " ".repeat(spaces_left).to_string() + &text;
    let spaces_right = width - text.emoji_len();
    text + " ".repeat(spaces_right).to_string().as_ref()
}

pub fn status_to_emoji(status: StatusState) -> &'static str {
    match status {
        StatusState::Created => PAUSE,
        StatusState::WaitingForResource => PAUSE,
        StatusState::Preparing => PAUSE,
        StatusState::Pending => PAUSE,
        StatusState::Running => PLAY,
        StatusState::Success => GREEN_CHECK,
        StatusState::Failed => FAILED,
        StatusState::Canceled => STOP,
        StatusState::Skipped => FAST_FORWARD,
        StatusState::Manual => PAUSE_TOGGLE,
        StatusState::Scheduled => ALARM,
    }
}

pub fn get_terminal_width() -> usize {
    termsize::get().unwrap().cols as usize - 1
}

pub enum RenderColumnsAlignment {
    _Left,
    Center,
    _Right,
}

impl RenderColumnsAlignment {
    pub fn align(&self, text: String, width: usize, pad: Option<char>) -> String {
        let spaces_left_count = (width - text.emoji_len()) / 2;
        let spaces_right_count = width - spaces_left_count - text.emoji_len();
        let pad = pad.unwrap_or(' ').to_string();
        let spaces_left = pad.repeat(spaces_left_count);
        let spaces_right = pad.repeat(spaces_right_count);
        match self {
            Self::_Left => text + &spaces_left + &spaces_right,
            Self::_Right => spaces_left + &spaces_right + &text,
            Self::Center => spaces_left + &text + &spaces_right,
        }
    }
}

pub fn render_columns(
    columns: Vec<Vec<String>>,
    widths: Vec<usize>,
    alignment: Vec<RenderColumnsAlignment>,
) -> String {
    let mut line_break_columns = Vec::new();
    for col in columns {
        let mut lines = Vec::new();
        for line in col {
            lines.append(
                &mut line
                    .split("\n")
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
            );
        }
        line_break_columns.push(lines);
    }
    let mut forced_line_breaks = Vec::new();
    for col_i in 0..line_break_columns.len() {
        let col = &line_break_columns[col_i];
        let mut lines = Vec::new();
        for line in col {
            // TODO: break if too long here
            lines.push(line.emoji_truncate(widths[col_i]));
        }
        forced_line_breaks.push(lines);
    }

    let height = forced_line_breaks
        .iter()
        .map(|col| col.len())
        .max()
        .unwrap();

    for col in forced_line_breaks.iter_mut() {
        while col.len() < height {
            col.push("".to_string());
        }
    }

    let mut result = String::new();
    let flipped = flip(forced_line_breaks);
    for line_i in 0..flipped.len() {
        let line = flipped[line_i].clone();
        for cell_i in 0..line.len() {
            let cell = line[cell_i].clone();
            result += &alignment[cell_i].align(cell, widths[cell_i], None)
        }
        result += "\n";
    }
    result
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn test_flip() {
        let v = vec![vec![1, 2, 3], vec![3, 4, 5]];
        assert_eq!(flip(v), vec![vec![1, 3], vec![2, 4], vec![3, 5]],)
    }

    #[test]
    fn test_render_columns_1() {
        let columns: Vec<Vec<String>> = vec![
            vec!["abc".to_string(), "123".to_string()],
            vec!["asdasdasd".to_string(), "456".to_string()],
        ];
        let rendering = render_columns(
            columns,
            vec![11, 10],
            vec![RenderColumnsAlignment::_Left, RenderColumnsAlignment::Left],
        );
        assert_eq!(
            rendering,
            "abc        asdasdasd \n\
            123        456       \n"
                .to_string(),
        );
    }

    #[test]
    fn test_render_columns_2() {
        let columns: Vec<Vec<String>> = vec![
            vec!["abc".to_string(), "123".to_string()],
            vec!["asdasdasd".to_string(), "456".to_string()],
        ];
        let rendering = render_columns(
            columns,
            vec![11, 10],
            vec![RenderColumnsAlignment::_Left, RenderColumnsAlignment::Right],
        );
        assert_eq!(
            rendering,
            "abc         asdasdasd\n\
            123               456\n"
                .to_string(),
        );
    }
}
