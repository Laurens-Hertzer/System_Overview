//old tui code

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

/*let chart_layout = Layout::horizontal([
    Constraint::Percentage(20),  // Chart nimmt 50% der Breite
    Constraint::Percentage(80),  // Rest bleibt frei (oder für anderes Widget)
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
    .marker(Marker::Braille)
    .graph_type(GraphType::Line)
    .style(Style::default().fg(Color::Blue))
    .data(&data);

let x_axis = Axis::default()
    .bounds([0.0, 9.0])
    .labels(["60", "30", "0"]);

let y_axis = Axis::default()
    .title(format!("CPU: {}%", current).blue())
    .bounds([0.0, 100.0]) // CPU-Auslastung 0–100%
    .labels(["0", "50", "100"]);

let chart = Chart::new(vec![dataset])
    .x_axis(x_axis)
    .y_axis(y_axis);

chart.render(chart_area, buf);

 */


// main code for the tui code

//let tx_to_input_events = event_tx.clone();

/*thread::spawn(move || {
    backend::handle_input_events(tx_to_input_events);
});*/

//let tx_to_background_progress_events = event_tx.clone();
//thread::spawn(move || backend::run_background_thread(tx_to_background_progress_events));


// for clap

//let cli = Cli::parse();

/*match cli.command {
    None                            => println!("Kein Subcommand → Dashboard"),
    Some(Commands::Disk)            => println!("Disk-Ansicht"),
    Some(Commands::Procs { limit }) => println!("Top {} Prozesse", limit),

}*/


// backend 

/*use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn run_background_thread(tx: mpsc::Sender<Event>) {
    let mut progress = 0_f64;
    let increment = 0.01_f64;
    loop {
        thread::sleep(Duration::from_millis(100));
        progress += increment;
        progress = progress.min(1_f64);
        tx.send(Event::Progress(progress)).unwrap();
    }
}*/