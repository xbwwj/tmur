use std::{
    io::{self, stdout},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread,
};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ratatui::{
    Terminal,
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    layout::{Alignment, Constraint, Layout},
    prelude::CrosstermBackend,
    style::Stylize,
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::logo::{MICROPHONE_IMG, build_image_lines};

mod logo;

#[derive(Debug, Clone)]
struct GlobalState {
    is_running: Arc<AtomicBool>,
    is_recording: Arc<AtomicBool>,
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(true)),
            is_recording: Default::default(),
        }
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    let state = GlobalState::default();

    let handler = thread::spawn({
        let state = state.clone();
        || {
            record_audio(state).unwrap();
        }
    });

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    while state.is_running.load(Ordering::Relaxed) {
        terminal.draw(|frame| {
            let area = frame.area();

            let mut text_lines = build_image_lines();

            text_lines.push(Line::from(""));
            text_lines.push(Line::from_iter([
                Span::raw("<Space>").underlined(),
                Span::raw(" to "),
                Span::raw(if state.is_recording.load(Ordering::Relaxed) {
                    "stop"
                } else {
                    "start"
                }),
            ]));

            let paragraph = Paragraph::new(text_lines).alignment(Alignment::Center);

            let content_height = (MICROPHONE_IMG.len() / 2 + 2) as u16;

            let [_, center_area, _] = Layout::vertical([
                Constraint::Fill(1),
                Constraint::Length(content_height),
                Constraint::Fill(1),
            ])
            .areas(area);

            frame.render_widget(paragraph, center_area);
        })?;

        if event::poll(std::time::Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
        {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    state.is_running.store(false, Ordering::Relaxed);
                }
                KeyCode::Char(' ') => {
                    state.is_recording.fetch_not(Ordering::Relaxed);
                }
                _ => {}
            }
        }
    }

    handler.join().unwrap();

    disable_raw_mode()?;
    execute!(stdout(), DisableMouseCapture, LeaveAlternateScreen)?;

    Ok(())
}

fn record_audio(state: GlobalState) -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let device = host.default_input_device().expect("no input device");
    let config = device.default_input_config()?;
    let sample_format = config.sample_format();
    let err_fn = |err| eprintln!("Error: {}", err);

    let spec = hound::WavSpec {
        channels: config.channels() as u16,
        sample_rate: config.sample_rate(),
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("foo.wav", spec)?;

    let (tx, rx) = mpsc::channel::<f32>();

    let stream = match sample_format {
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data: &[f32], _: &_| {
                for &sample in data {
                    tx.send(sample).ok();
                }
            },
            err_fn,
            None,
        )?,
        cpal::SampleFormat::I16 => device.build_input_stream(
            &config.into(),
            move |data: &[i16], _: &_| {
                for &sample in data {
                    tx.send(cpal::Sample::to_sample::<f32>(sample)).ok();
                }
            },
            err_fn,
            None,
        )?,
        _ => return Err("unsupported sample format".into()),
    };

    stream.play()?;

    while state.is_running.load(Ordering::Relaxed) {
        while let Ok(sample) = rx.try_recv() {
            if state.is_recording.load(Ordering::Relaxed) {
                let s = (sample * i16::MAX as f32) as i16;
                writer.write_sample(s)?;
            }
        }
    }

    drop(stream);
    writer.finalize()?;

    Ok(())
}
