use crate::backend::Event;
use crossterm::event::{self, KeyCode, KeyEventKind};
use std::collections::VecDeque;
use std::io;
use std::ptr::null;
use std::sync::mpsc;
use nvml_wrapper::error::Bits::U32;
use nvml_wrapper::error::NvmlError;
use nvml_wrapper::struct_wrappers::device::MemoryInfo;
use ratatui::prelude::Buffer;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Constraint, Layout, Offset, Rect},
    style::{Color, Style, Stylize},
    symbols,
    symbols::{Marker, border},
    text::{Line, Span},
    widgets::{Axis, Block, Chart, Dataset, GraphType, Paragraph, Tabs, Widget},
};
use crate::utils::logo_rata;

pub struct App {
    exit: bool,
    progress_bar_color: Color,
    tab_selection: i16,
    cpu_history: VecDeque<u64>,
    ram_history: VecDeque<u64>,
    gpu_history: VecDeque<u64>,
    gpu_brand: String,
    fan_speed: u32,
    power_limit: u32,
    memory_info: Option<MemoryInfo>,
    gpu_not_available: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            exit: false,
            progress_bar_color: Color::Green,
            tab_selection: 0,
            cpu_history: VecDeque::new(),
            ram_history: VecDeque::new(),
            gpu_history: VecDeque::new(),
            gpu_brand: String::new(),
            fan_speed: 0,
            power_limit: 0,
            memory_info: None,
            gpu_not_available: true,
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
                    Event::GpuNotAvailable => {
                        self.gpu_not_available = true;
                    }
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
                    Event::RamProgress(progress) => {
                        self.ram_history.push_back((progress) as u64);
                        while self.ram_history.len() > 60 {
                            self.ram_history.pop_front();
                        }
                    }
                    // HIER: Das Struct-Matching für die GPU
                    Event::GpuProgress {
                        utilization,
                        brand,
                        fan_speed,
                        power_limit,
                        memory_info
                    } => {
                        // 1. Auslastung in den Verlauf pushen
                        self.gpu_history.push_back(utilization as u64);
                        while self.gpu_history.len() > 60 {
                            self.gpu_history.pop_front();
                        }

                        // 2. Werte an dein App-Struct übertragen
                        self.fan_speed = fan_speed;
                        self.power_limit = power_limit / 1000; // von Milliwatt in Watt
                        self.memory_info = Some(memory_info);

                        // 3. Das NVML-Brand-Enum in Text für die UI übersetzen
                        self.gpu_brand = match brand {
                            nvml_wrapper::enum_wrappers::device::Brand::Nvidia => "NVIDIA".to_string(),
                            nvml_wrapper::enum_wrappers::device::Brand::GeForce => "GeForce".to_string(),
                            nvml_wrapper::enum_wrappers::device::Brand::Tesla => "Tesla".to_string(),
                            nvml_wrapper::enum_wrappers::device::Brand::Quadro => "Quadro".to_string(),
                            _ => "Unknown GPU".to_string(),
                        };
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
        let mut sys = sysinfo::System::new_all();

        // CPU

        let cpu_current = self.cpu_history.back().copied().unwrap_or(0);

        let cpu_len = self.cpu_history.len() as f64;

        let cpu_offset = 60.0 - cpu_len;
        let cpu_data: Vec<(f64, f64)> = self
            .cpu_history
            .iter()
            .enumerate()
            .map(|(i, &v)| (cpu_offset + i as f64, v as f64))
            .collect();

        let x_labels = vec![
            Line::from("60s").left_aligned(),
            Line::from("30s").centered(),
            Line::from("0s").right_aligned(),
        ];

        let y_labels = vec![Line::from("0%"), Line::from("50%"), Line::from("100%")];

        Chart::new(vec![
            Dataset::default()
                .marker(Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(Color::Cyan))
                .data(&cpu_data),
        ])
        .block(
            Block::bordered()
                .title(
                    Line::from(format!(" CPU {}% ", cpu_current))
                        .fg(Color::Cyan)
                        .bold(),
                )
                .border_set(border::THICK),
        )
        .x_axis(Axis::default().bounds([0.0, 60.0]).labels(x_labels))
        .y_axis(Axis::default().bounds([0.0, 100.0]).labels(y_labels))
        .render(graph1_area, buf);

        // Ram

        let max_ram = (sys.total_memory() as f64) / 1_073_741_824.0;

        let max_ram_string = format!("{:.0} GB", max_ram);

        let ram_current = self.ram_history.back().copied().unwrap_or(0);

        let ram_current_procent = (((ram_current as f64) / max_ram) * 100.0).round() as u64;

        let half_max_ram_string = format!("{:.0} GB", max_ram / 2.0);

        let ram_len = self.ram_history.len() as f64;

        let ram_offset = 60.0 - ram_len;

        let ram_data: Vec<(f64, f64)> = self
            .ram_history
            .iter()
            .enumerate()
            .map(|(i, &v)| (ram_offset + i as f64, v as f64))
            .collect();

        let x_labels = vec![
            Line::from("60s").left_aligned(),
            Line::from("30s").centered(),
            Line::from("0s").right_aligned(),
        ];

        let y_labels = vec![
            Line::from("0 GB"),
            Line::from(half_max_ram_string.as_str()),
            Line::from(max_ram_string.as_str()),
        ];

        Chart::new(vec![
            Dataset::default()
                .marker(Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(Color::Blue))
                .data(&ram_data),
        ])
        .block(
            Block::bordered()
                .title(
                    Line::from(format!(" RAM {} % ", ram_current_procent))
                        .fg(Color::Blue)
                        .bold(),
                )
                .border_set(border::THICK),
        )
        .x_axis(Axis::default().bounds([0.0, 60.0]).labels(x_labels))
        .y_axis(Axis::default().bounds([0.0, max_ram]).labels(y_labels))
        .render(graph2_area, buf);

        //GPU 1

        if (self.gpu_not_available == false) {
            let gpu_current = self.gpu_history.back().copied().unwrap_or(0);

            let gpu_len = self.gpu_history.len() as f64;

            let gpu_offset = 60.0 - gpu_len;

            let gpu_data: Vec<(f64, f64)> = self
                .gpu_history
                .iter()
                .enumerate()
                .map(|(i, &v)| (gpu_offset + i as f64, v as f64))
                .collect();

            let x_labels = vec![
                Line::from("60s").left_aligned(),
                Line::from("30s").centered(),
                Line::from("0s").right_aligned(),
            ];

            let y_labels = vec![Line::from("0%"), Line::from("50%"), Line::from("100%")];

            Chart::new(vec![
                Dataset::default()
                    .marker(Marker::Braille)
                    .graph_type(GraphType::Line)
                    .style(Style::default().fg(Color::Cyan))
                    .data(&gpu_data),
            ])
                .block(
                    Block::bordered()
                        .title(
                            Line::from(format!(" GPU {}% ", gpu_current))
                                .fg(Color::Cyan)
                                .bold(),
                        )
                        .border_set(border::THICK),
                )
                .x_axis(Axis::default().bounds([0.0, 60.0]).labels(x_labels))
                .y_axis(Axis::default().bounds([0.0, 100.0]).labels(y_labels))
                .render(graph3_area, buf);
        } else {
            logo_rata(graph3_area, buf)
        }

        //disk

        let disk_current = self.disk_history.back().copied().unwrap_or(0);

        let disk_len = self.disk_history.len() as f64;

        let disk_offset = 60.0 - disk_len;
        let disk_data: Vec<(f64, f64)> = self
            .disk_history
            .iter()
            .enumerate()
            .map(|(i, &v)| (ram_offset + i as f64, v as f64))
            .collect();

        let x_labels = vec![
            Line::from("60s").left_aligned(),
            Line::from("30s").centered(),
            Line::from("0s").right_aligned(),
        ];

        let y_labels = vec![Line::from("0%"), Line::from("50%"), Line::from("100%")];

        Chart::new(vec![
            Dataset::default()
                .marker(Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(Color::Cyan))
                .data(&disk_data),
        ])
            .block(
                Block::bordered()
                    .title(
                        Line::from(format!(" disk {}% ", disk_current))
                            .fg(Color::Cyan)
                            .bold(),
                    )
                    .border_set(border::THICK),
            )
            .x_axis(Axis::default().bounds([0.0, 60.0]).labels(x_labels))
            .y_axis(Axis::default().bounds([0.0, 100.0]).labels(y_labels))
            .render(graph1_area, buf);

        //GPU 0

        //WLAN
    }

    fn render_containers(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Containers")
            .block(Block::bordered())
            .render(area, buf);
    }

    fn render_network(&self, area: Rect, buf: &mut Buffer) {
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
        let vertical = Layout::vertical([Constraint::Length(1), Constraint::Percentage(100)]);
        let [tabs_area, content_area] = vertical.areas(area);

        let content_layout =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);

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
