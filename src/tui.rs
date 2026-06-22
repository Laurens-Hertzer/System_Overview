use crate::backend::Event;
use crossterm::event::{self, KeyCode, KeyEventKind};
use std::collections::VecDeque;
use std::io;
use std::ptr::null;
use std::sync::mpsc;

use ratatui::{DefaultTerminal, Frame, layout::{Alignment, Constraint, Layout, Offset, Rect}, style::{Color, Style, Stylize}, symbols::{border, Marker}, text::{Line, Span}, widgets::{Axis, Block, Chart, Dataset, GraphType, Widget, Paragraph, Tabs}, symbols};
use ratatui::prelude::Buffer;

pub struct App {
    exit: bool,
    progress_bar_color: Color,
    tab_selection : i16,
    cpu_history: VecDeque<u64>,
}

impl App {
    pub fn new() -> Self {
        App {
            exit: false,
            progress_bar_color: Color::Green,
            tab_selection: 0,
            cpu_history: VecDeque::new(),
        }
    }

    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        rx: mpsc::Receiver<Event>,
    ) -> io::Result<()> {
        terminal.draw(|frame| self.draw(frame))?;
        while !self.exit {
            if let Ok(event) = rx.recv_timeout(std::time::Duration::from_millis(100)) {
                match event {
                    Event::Input(key_event) => {
                        if key_event.kind == KeyEventKind::Press {
                            self.handle_key_event(key_event)?;
                        }
                    }
                    Event::CpuProgress(progress) => {
                        self.cpu_history.push_back((progress * 100.0) as u64);
                        while self.cpu_history.len() > 60 {
                            self.cpu_history.pop_front();
                        }
                    }
                }
                terminal.draw(|frame| self.draw(frame))?;
            }
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.exit = true;
            }

            KeyCode::Char('c') => {
                if self.progress_bar_color == Color::Green {
                    self.progress_bar_color = Color::Yellow;
                } else {
                    self.progress_bar_color = Color::Green;
                }
            }

            KeyCode::Char('l') | KeyCode::Right => {
                self.tab_selection = (self.tab_selection + 1) % 3;
            }

            KeyCode::Char('h') | KeyCode::Left => {
                self.tab_selection = (self.tab_selection + 2) % 3;
            }
            _ => {}
        }

        Ok(())
    }

    fn render_resources(
        &self,
        graph1_area: Rect,
        graph2_area: Rect,
        graph3_area: Rect,
        graph4_area: Rect,
        graph5_area: Rect,
        graph6_area: Rect,
        buf: &mut Buffer,
    ) {
        let current = self.cpu_history.back().copied().unwrap_or(0);

        let len = self.cpu_history.len() as f64;

        let offset = 60.0 - len;
        let data: Vec<(f64, f64)> = self
            .cpu_history
            .iter()
            .enumerate()
            .map(|(i, &v)| (offset + i as f64, v as f64))
            .collect();

        let x_labels = vec![
            Line::from("60s").left_aligned(),
            Line::from("30s").centered(),
            Line::from("0s").right_aligned(),
        ];

        let y_labels = vec![
            Line::from("0%"),
            Line::from("50%"),
            Line::from("100%"),
        ];

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
            .x_axis(Axis::default().bounds([0.0, 60.0]).labels(x_labels))
            .y_axis(Axis::default().bounds([0.0, 100.0]).labels(y_labels))
            .render(graph1_area, buf);
    }

    fn render_containers(
        &self,
        area: Rect,
        buf: &mut Buffer,
    ) {
        Paragraph::new("Containers")
            .block(Block::bordered())
            .render(area, buf);
    }

    fn render_network(
        &self,
        area: Rect,
        buf: &mut Buffer,
    ) {
        Paragraph::new("Network")
            .block(Block::bordered())
            .render(area, buf);
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Percentage(100)
        ]);
        let [tabs_area, content_area] = vertical.areas(area);

        let content_layout = Layout::vertical([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]);

        let [top_area, bottom_area] = content_layout.areas(content_area);

        let horizontal_content_layout = Layout::horizontal([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ]);
        let [graph1_area, graph2_area, graph3_area] = horizontal_content_layout.areas(top_area);
        let [graph4_area, graph5_area, graph6_area] = horizontal_content_layout.areas(bottom_area);

        //let tab_selection = self.tab_selection;

        Tabs::new(vec!["Resources", "Containers", "Network"])
            .style(Color::White)
            .highlight_style(Style::default().magenta().on_black().bold())
            .select(self.tab_selection as usize)
            .divider(symbols::DOT)
            .padding(" ", " ")
            .render(tabs_area, buf);

        match self.tab_selection {
            0 => self.render_resources(
                graph1_area,
                graph2_area,
                graph3_area,
                graph4_area,
                graph5_area,
                graph6_area,
                buf,
            ),
            1 => self.render_containers(content_area, buf),
            2 => self.render_network(content_area, buf),
            _ => {}
        }
    }
}