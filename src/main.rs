use crossterm::cursor;
use crossterm::event;
use crossterm::event::DisableMouseCapture;
use crossterm::event::EnableMouseCapture;
use crossterm::execute;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use ratatui::backend::CrosstermBackend;
use ratatui::prelude::Backend;
use ratatui::prelude::Constraint;
use ratatui::prelude::Direction;
use ratatui::prelude::Layout;
use ratatui::style::Color;
use ratatui::style::Modifier;
use ratatui::style::Style;
//use ratatui::symbols::DOT;
//use ratatui::text::Line;
use ratatui::widgets::calendar::CalendarEventStore;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
//use ratatui::widgets::Tabs;
use ratatui::widgets::calendar::Monthly;
use ratatui::Frame;
use ratatui::Terminal;
use std::io;
use std::panic;
use std::thread;
use std::time::Duration;
use time::Date;

fn destruct_terminal() {
    disable_raw_mode().unwrap();
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture).unwrap();
    execute!(io::stdout(), cursor::Show).unwrap();
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(f.size());
    let block = Block::default().title("Block").borders(Borders::ALL);
    f.render_widget(block, chunks[0]);

    // let titles = ["Tab1", "Tab2", "Tab3", "Tab4"]
    //     .iter()
    //     .cloned()
    //     .map(Line::from)
    //     .collect();
    // let tabs = Tabs::new(titles)
    //     .block(Block::default().borders(Borders::ALL))
    //     .style(
    //         Style::default()
    //             .fg(Color::White)
    //     )
    //     .highlight_style(Style::default().fg(Color::Yellow))
    //     .divider("|")
    //     .select(1);
    // f.render_widget(tabs, chunks[1]);
    let default_style = Style::default()
        .add_modifier(Modifier::BOLD)
        .bg(Color::Rgb(50, 50, 50));
    let calendar = Monthly::new(
        Date::from_calendar_date(2023, time::Month::September, 10).unwrap(),
        CalendarEventStore::default(),
    )
    .default_style(default_style);
    f.render_widget(calendar, chunks[1]);

    let block = Block::default().title("Block 2").borders(Borders::ALL);
    f.render_widget(block, chunks[2]);
}

fn main() -> Result<(), io::Error> {
    panic::set_hook(Box::new(|panic_info| {
        destruct_terminal();
        better_panic::Settings::auto().create_panic_handler()(panic_info);
    }));

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // terminal.draw(|f| {
    //     let size = f.size();
    //     let block = Block::default().title(" Block ").borders(Borders::ALL);
    //     f.render_widget(block, size);
    // })?;
    terminal.draw(ui);

    thread::spawn(|| loop {
        let _ = event::read();
    });

    thread::sleep(Duration::from_millis(5000));

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
