use unicode_segmentation::UnicodeSegmentation;

pub const PAUSE: &'static str = "⏸";
pub const PLAY: &'static str = "▶️";
pub const FAILED: &'static str = "❌";
pub const GREEN_CHECK: &'static str = "✅";
pub const STOP: &'static str = "⏹";
pub const PAUSE_TOGGLE: &'static str = "⏯";
pub const GREY_EXCLAMATION: &'static str = "❕";
pub const _GREY_QUESTION_MARK: &'static str = "❔";
pub const FAST_FORWARD: &'static str = "⏩";
pub const ALARM: &'static str = "⏰";

pub trait EmojiLength {
    fn emoji_len(&self) -> usize;
    fn emoji_truncate(&self, n: usize) -> String;
}

impl EmojiLength for String {
    fn emoji_len(&self) -> usize {
        self.graphemes(true)
            .map(|c| {
                // emojis are rendered with width 2
                if c.as_bytes()[0] == PAUSE.as_bytes()[0] {
                    return 2;
                }
                1
            })
            .sum()
    }

    fn emoji_truncate(&self, n: usize) -> String {
        self.graphemes(true).take(n).collect()
    }
}
