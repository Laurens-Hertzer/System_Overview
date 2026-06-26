use std::collections::VecDeque;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Line, Stylize, Widget};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Paragraph};
use ratatui::prelude::Buffer;

pub fn bytes_to_terabytes(bytes: u64) -> f64 {
    bytes as f64 / 1_099_511_627_776.0
}
pub fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / 1_073_741_824.0
}

pub fn bytes_to_mb(bytes: u64) -> f64 {
    bytes as f64 / 1_048_576.0
}

pub fn push_value(history: &mut VecDeque<u64>, value: u64) {
    history.push_back(value);
    if history.len() > 10 {
        history.pop_front();
    }
}
pub fn logo_print () {
    println!(r#" ____            _
/ ___| _   _ ___| |_ ___ _ __ ___
\___ \| | | / __| __/ _ \ '_ ` _ \
 ___) | |_| \__ \ ||  __/ | | | | |
|____/ \__, |___/\__\___|_| |_| |_|
 / _ \_|___/____ _ ____   _(_) _____      __
| | | \ \ / / _ \ '__\ \ / / |/ _ \ \ /\ / /
| |_| |\ V /  __/ |   \ V /| |  __/\ V  V /
 \___/  \_/ \___|_|    \_/ |_|\___| \_/\_/  "#);
}

pub fn logo_rata(graph3_area: Rect, buf: &mut Buffer) {
    Paragraph::new(vec![
        Line::from(""),
        Line::from(r" ____            _").fg(Color::Green),
        Line::from(r"/ ___| _   _ ___| |_ ___ _ __ ___").fg(Color::Green),
        Line::from(r"\___ \| | | / __| __/ _ \ '_ ` _ \").fg(Color::Green),
        Line::from(r" ___) | |_| \__ \ ||  __/ | | | | |").fg(Color::Green),
        Line::from(r"|____/ \__, |___/\__\___|_| |_| |_|").fg(Color::Green),
        Line::from(r" / _ \_|___/____ _ ____   _(_) _____      __").fg(Color::Green),
        Line::from(r"| | | \ \ / / _ \ '__\ \ / / |/ _ \ \ /\ / /").fg(Color::Green),
        Line::from(r"| |_| |\ V /  __/ |   \ V /| |  __/\ V  V /").fg(Color::Green),
        Line::from(r" \___/  \_/ \___|_|    \_/ |_|\___| \_/\_/  ").fg(Color::Green),
        Line::from(""),
        Line::from("  No GPU found").fg(Color::DarkGray),
    ])
        .block(
            Block::bordered()
                .title(Line::from(" GPU ").fg(Color::DarkGray).bold())
                .border_set(border::THICK),
        )
        .render(graph3_area, buf);
}