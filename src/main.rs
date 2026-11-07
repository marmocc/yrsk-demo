use yrsk::{ansi::Terminal, common::Position};

fn main() {
    let mut terminal = Terminal::new();
    terminal.push_move(Position::new(5, 5));
    terminal.push_byte(b'H');
    let _ = terminal.flush();
}
