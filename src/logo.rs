use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

pub const MICROPHONE_IMG: [[u8; 17]; 26] = [
    [0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 1, 2, 1, 2, 1, 2, 1, 1, 0, 0, 0, 0],
    [0, 0, 0, 1, 1, 2, 3, 1, 4, 1, 4, 5, 1, 1, 0, 0, 0],
    [0, 0, 0, 1, 2, 2, 4, 4, 4, 4, 4, 5, 5, 1, 0, 0, 0],
    [0, 0, 0, 1, 2, 2, 4, 4, 4, 4, 4, 5, 5, 1, 0, 0, 0],
    [0, 0, 0, 1, 1, 1, 1, 4, 4, 4, 1, 1, 1, 1, 0, 0, 0],
    [0, 0, 0, 1, 2, 2, 4, 4, 4, 4, 4, 5, 5, 1, 0, 0, 0],
    [0, 0, 0, 1, 1, 1, 1, 4, 4, 4, 1, 1, 1, 1, 0, 0, 0],
    [0, 0, 0, 1, 2, 2, 4, 4, 4, 4, 4, 5, 5, 1, 0, 0, 0],
    [0, 0, 0, 1, 1, 1, 1, 4, 4, 4, 1, 1, 1, 1, 0, 0, 0],
    [0, 1, 1, 1, 2, 2, 4, 4, 4, 4, 4, 5, 5, 1, 1, 1, 0],
    [1, 3, 5, 1, 1, 1, 1, 4, 4, 4, 1, 1, 1, 1, 4, 5, 1],
    [1, 3, 5, 1, 2, 2, 4, 4, 4, 4, 4, 5, 5, 1, 4, 5, 1],
    [0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0],
    [0, 1, 0, 1, 2, 2, 4, 4, 4, 4, 4, 5, 5, 1, 0, 1, 0],
    [0, 1, 0, 1, 1, 2, 4, 4, 4, 4, 5, 5, 1, 1, 0, 1, 0],
    [0, 1, 0, 0, 1, 1, 5, 5, 5, 5, 5, 1, 1, 0, 0, 1, 0],
    [0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0],
    [0, 0, 1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 1, 0, 0],
    [0, 0, 0, 1, 1, 5, 5, 5, 5, 5, 5, 5, 1, 1, 0, 0, 0],
    [0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 1, 5, 1, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 1, 1, 1, 4, 1, 1, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 2, 2, 1, 1, 1, 5, 5, 1, 0, 0, 0, 0],
    [0, 0, 0, 1, 2, 2, 2, 2, 2, 4, 5, 5, 5, 1, 0, 0, 0],
    [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0],
];

pub const COLORS: [Option<Color>; 6] = [
    None,
    Some(Color::Rgb(0x20, 0x20, 0x22)),
    Some(Color::Rgb(0xfa, 0xe2, 0xc6)),
    Some(Color::Rgb(0xc5, 0xc9, 0xc8)),
    Some(Color::Rgb(0xb2, 0xbd, 0xbf)),
    Some(Color::Rgb(0x54, 0x62, 0x6b)),
];

pub fn build_image_lines() -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    for y in (0..MICROPHONE_IMG.len()).step_by(2) {
        let mut spans = Vec::new();
        for (top, bottom) in MICROPHONE_IMG[y].into_iter().zip(
            // SAFETY: img rows is known to be even at compiletime
            MICROPHONE_IMG[y + 1].into_iter(),
        ) {
            let top_color = COLORS[top as usize];
            let bottom_color = COLORS[bottom as usize];
            let span = match (top_color, bottom_color) {
                (None, None) => Span::raw(" "),
                (Some(fg), None) => Span::styled("▀", Style::default().fg(fg)),
                (None, Some(fg)) => Span::styled("▄", Style::default().fg(fg)),
                (Some(fg), Some(bg)) => Span::styled("▀", Style::default().fg(fg).bg(bg)),
            };
            spans.push(span);
        }
        lines.push(Line::from(spans));
    }

    lines
}
