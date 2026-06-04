use crate::backend::Event;
use crossterm::event::{KeyCode, KeyEventKind};
use std::collections::VecDeque;
use std::io;
use std::sync::mpsc;

use ratatui::widgets::{Axis, Chart, Dataset, GraphType, RenderDirection, Sparkline};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Gauge, Widget},
};
use ratatui::symbols::Marker;

pub struct App {
    exit: bool,
    progress_bar_color: Color,
    background_progress: f64,
    cpu_history: VecDeque<u64>,
}

impl App {
    pub fn new() -> Self {
        App {
            exit: false,
            progress_bar_color: Color::Green,
            background_progress: 0_f64,
            cpu_history: VecDeque::new(),
        }
    }
    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        rx: mpsc::Receiver<Event>,
    ) -> io::Result<()> {
        while !self.exit {
            match rx.recv().unwrap() {
                Event::Input(key_event) => self.handle_key_event(key_event)?,
                Event::Progress(progress) => self.background_progress = progress,
                Event::CpuProgress(progress) => {
                    self.cpu_history.push_back((progress * 100.0) as u64);
                    while self.cpu_history.len() > 10 {
                        self.cpu_history.pop_front();
                    }
                }
            }
            terminal.draw(|frame| self.draw(frame))?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('q') {
            self.exit = true;
        } else if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('c') {
            if self.progress_bar_color == Color::Green {
                self.progress_bar_color = Color::Yellow;
            } else {
                self.progress_bar_color = Color::Green;
            }
        }
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let vertical_layout =
            Layout::vertical([Constraint::Percentage(20), Constraint::Percentage(80)]);
        let [title_area, gauge_area] = vertical_layout.areas(area);

        let gauges_layout =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);
        let [gauge1_area, gauge2_area] = gauges_layout.areas(gauge_area);

        //Render title
        Line::from("Process overview")
            .bold()
            .render(title_area, buf);

        let instructions = Line::from(vec![
            "change color".into(),
            "<C>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ])
        .centered();
        /*
                let example_block = Block::bordered()
                    .title(Line::from("Background Processes"))
                    .title_bottom(instructions.clone())
                    .border_set(border::THICK);

                let example_progress_bar = Gauge::default()
                    .gauge_style(Style::default().fg(self.progress_bar_color))
                    .block(example_block)
                    .label(format!(
                        "Process 2: {:.2}%",
                        self.background_progress * 100_f64
                    ))
                    .ratio(self.background_progress);

                example_progress_bar.render(
                    Rect {
                        x: gauge1_area.left(),
                        y: gauge1_area.top(),
                        width: gauge1_area.width,
                        height: 3,
                    },
                    buf,
                );
        */
        /*let cpu_block = Block::bordered()
            .title(Line::from("Background Processes"))
            .title_bottom(instructions.clone())
            .border_set(border::THICK);

        let cpu_progress_bar = Gauge::default()
            .gauge_style(Style::default().fg(self.progress_bar_color))
            .block(cpu_block)
            .label(format!(
                "Process 1: {:.2}%",
                self.cpu_load_percentage * 100_f64
            ))
            .ratio(self.cpu_load_percentage);

        cpu_progress_bar.render(
            Rect {
                x: gauge2_area.left(),
                y: gauge2_area.top(),
                width: gauge2_area.width,
                height: 3,
            },
            buf,
        );
         */
        /* Sparkline
                let data: Vec<u64> = self.cpu_history.iter().copied().collect();
                let current = data.last().copied().unwrap_or(0);

                let data: Vec<u64> = self.cpu_history.iter().copied().collect();
                Sparkline::default()
                    .block(Block::bordered().title(format!("CPU: {current}%")))
                    .data(&data)
                    .max(100)
                    .render(area, buf);
            }
        }
        */

        let chart_layout = Layout::horizontal([
            Constraint::Percentage(20),  // Chart nimmt 50% der Breite
            Constraint::Percentage(50),  // Rest bleibt frei (oder für anderes Widget)
        ]);
        let [chart_area, _rest] = chart_layout.areas(gauge2_area);

        let data: Vec<(f64, f64)> = self
            .cpu_history
            .iter()
            .enumerate()
            .map(|(i, &v)| (i as f64, v as f64))
            .collect();

        let current = self.cpu_history.back().copied().unwrap_or(0);

        let dataset = Dataset::default()
            .name("Cpu Usage")
            .marker(Marker::Sextant)
            .graph_type(GraphType::Bar)
            .style(Style::default().fg(Color::Blue))
            .data(&data);

        let x_axis = Axis::default()
            .bounds([0.0, 9.0])
            .labels([""]);

        let y_axis = Axis::default()
            .title(format!("CPU: {}%", current).blue())
            .bounds([0.0, 100.0]) // CPU-Auslastung 0–100%
            .labels(["0", "50", "100"]);

        let chart = Chart::new(vec![dataset])
            .x_axis(x_axis)
            .y_axis(y_axis);

        chart.render(chart_area, buf);
    }
}
