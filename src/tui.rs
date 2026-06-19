use crate::backend::Event;
use crossterm::event::{KeyCode, KeyEventKind};
use std::collections::VecDeque;
use std::io;
use std::sync::mpsc;

use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    symbols::{border, Marker},
    text::Line,
    widgets::{Axis, Block, Chart, Dataset, GraphType, Widget},
};

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
                    while self.cpu_history.len() > 60 {
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
        let vertical = Layout::vertical([
            Constraint::Length(1),        // Titelzeile
            Constraint::Percentage(50),   // obere Reihe
            Constraint::Percentage(50),   // untere Reihe
        ]);
        let [title_area, top_area, bottom_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ]);
        let [graph1_area, graph2_area, graph3_area] = horizontal.areas(top_area);
        let [graph4_area, graph5_area, graph6_area] = horizontal.areas(bottom_area);

        Line::from("Process overview").bold().render(title_area, buf);

        let current = self.cpu_history.back().copied().unwrap_or(0);

        let len = self.cpu_history.len() as f64;

        // Datenpunkte so verschieben dass der neueste immer bei x=60 liegt
        let offset = 60.0 - len;
        let data: Vec<(f64, f64)> = self
            .cpu_history
            .iter()
            .enumerate()
            .map(|(i, &v)| (offset + i as f64, v as f64 / 2.0))
            .collect();

        Chart::new(vec![Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Cyan))
            .data(&data)])
            .block(
                Block::bordered()
                    .title(Line::from(format!(" CPU {}% ", current)).fg(Color::Cyan).bold())
                    .border_set(border::THICK),
            )
            .x_axis(Axis::default().bounds([0.0, 60.0]))
            .y_axis(Axis::default().bounds([0.0, 50.0]))
            .render(graph1_area, buf);
    }
}