use std::io;

use tui::Terminal;
use tui::backend::CrosstermBackend;

fn main() {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend).expect("Can not start terminal UI. ");
    
}