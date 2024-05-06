
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};

use ratatui::{
    prelude::{Color, CrosstermBackend, Stylize,
        Line, Span, Style, Terminal, Text},
    widgets::{Block,Borders,Paragraph},
};

use std::io::{stdout, Result};


struct App {
    headers:   Vec<String>,
    sequences: Vec<String>,
}

impl App {
    fn new() -> App {
        App {
            headers: vec![
                String::from("seq1"),
            ],
            sequences: vec![
                String::from("ATGC"),
                String::from("A-GC"),
                String::from("A-CC"),
                String::from("A-GC"),
            ]
        }
    }
}

fn make_span(c: &str) -> Span {
    Span::styled(c, Style::default().fg(Color::Red))
}

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut app = App::new();
    let saln: Vec<&str> = app.sequences.iter().map(String::as_str).collect();

    // main loop
    loop {
        // Draw UI
        terminal.draw(|frame| {
            let area = frame.size();
            let aln_block = Block::default().title("Alignment").borders(Borders::ALL);
            let mut laln: Vec<Line> = saln.iter().map(|l| Line::from(*l)).collect();

            // let spanv: Vec<Span> = saln[0].chars().map(|c| make_span(c.to_string().as_str())).collect();
            let span = Line::from(make_span("G"));
            laln.push(span);

            let text = Text::from(laln);
            //let text = Text::from_iter(&app.sequences);
            let para = Paragraph::new(text).white().block(aln_block);
            frame.render_widget(
                para,
                area,
            );
        })?;
        // handle events
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press 
                    && (key.code == KeyCode::Char('q')
                        ||
                        key.code == KeyCode::Char('Q'))
                {
                    break;
                }
            }
        }
    }
    
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

/*
fn main() {
    //print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    print!("{}", termion::clear::All);
    println!("ATGCCGTAGTGAA");
    println!("ATGCCGTAGTGAA");
    println!("ATGCCGTAGTGAA");
    println!("ATGCCGTAGTGAA");
    println!("ATGCCGTAGTGAA");
    println!("ATGCCGTAGTGAA");
    println!("Size is {:?}", terminal_size().unwrap());
}
*/
