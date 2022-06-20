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
