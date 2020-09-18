use crossterm::{cursor, event, execute, queue, style, terminal};
use std::io::Write;
mod board;
mod help;

fn run() -> crossterm::Result<()> {
    let mut stdout = std::io::stdout();
    let mut state = State::Board;
    let dur = std::time::Duration::from_millis(50);

    let (width, height) = terminal::size()?;
    let mut board = board::Board::new(width as usize, (2 * height) as usize - 2);

    queue!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        terminal::SetTitle("Game of Life"),
        cursor::Hide,
        event::EnableMouseCapture,
    )?;

    loop {
        if event::poll(dur)? {
            match event::read()? {
                event::Event::Key(e) => match e.code {
                    event::KeyCode::Char(c) => {
                        if e.modifiers.contains(event::KeyModifiers::CONTROL) && c == 'c' {
                            return Ok(());
                        }

                        match c {
                            'h' => {
                                queue!(stdout, terminal::Clear(terminal::ClearType::All))?;
                                state = State::Help;
                            }
                            's' => {
                                if let State::Edit(a) = state {
                                    board.edit(a);
                                    state = State::Board;
                                };
                            }
                            'b' => state = State::Board,
                            'q' => return Ok(()),
                            'r' => board.reset(),
                            'e' => {
                                state = State::Edit(vec![
                                    vec![false; width as usize];
                                    2 * (height - 1) as usize
                                ])
                            }
                            _ => {}
                        }
                    }
                    event::KeyCode::Esc => return Ok(()),
                    _ => {}
                },
                event::Event::Mouse(e) => {
                    if let State::Edit(ref mut alive) = state {
                        let e = match e {
                            event::MouseEvent::Down(button, x, y, _) => Some((button, x, y)),
                            event::MouseEvent::Drag(button, x, y, _) => Some((button, x, y)),
                            _ => None,
                        };

                        if let Some((button, x, y)) = e {
                            let x = x as usize;
                            let y = y as usize * 2; // Double pixels

                            if x < alive[0].len() || y < alive.len() {
                                match button {
                                    event::MouseButton::Left => alive[y + 1][x] = !alive[y + 1][x],
                                    event::MouseButton::Right => alive[y][x] = !alive[y][x],
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        match state {
            State::Help => {
                help::show(&mut stdout)?;

                queue!(
                    stdout,
                    cursor::SavePosition,
                    cursor::MoveTo(1, height - 1),
                    style::Print("type b to go back"),
                    style::Print(" ".repeat((width as usize) - 18)),
                    cursor::RestorePosition,
                )?;
            }
            State::Edit(ref vec) => {
                board::display_alive(&mut stdout, vec)?;

                queue!(
                    stdout,
                    cursor::SavePosition,
                    cursor::MoveTo(1, height - 1),
                    style::Print("type b to cancel, s to use save"),
                    style::Print(" ".repeat((width as usize) - 32)),
                    cursor::RestorePosition,
                )?;
            }
            State::Board => {
                board.next();
                board.display(&mut stdout)?;

                let text = format!("renders: {}", board.renders);
                queue!(
                    stdout,
                    cursor::SavePosition,
                    cursor::MoveTo(1, height - 1),
                    style::Print("type h for help"),
                    style::Print(" ".repeat((width as usize) - text.len() - 16)),
                    style::Print(text),
                    cursor::RestorePosition,
                )?;
            }
        }

        stdout.flush()?;
    }
}

fn main() -> crossterm::Result<()> {
    terminal::enable_raw_mode()?;
    run()?;
    terminal::disable_raw_mode()?;
    execute!(std::io::stdout(), cursor::Show, event::DisableMouseCapture)?;

    Ok(())
}

#[derive(PartialEq)]
enum State {
    Board,
    Help,
    Edit(Vec<Vec<bool>>),
}
