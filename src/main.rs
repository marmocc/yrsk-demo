use yrsk::{
    diagnostics::chaos::Chaos,
    rendering::{ansi::terminal::Terminal, renderer::Renderer},
};

fn main() {
    let mut surface = Terminal::new().expect("Unable to create Terminal.");
    let result = surface.benchmark(&mut Chaos::new(), 10_000);
    drop(surface);

    let (average_paint, average_present) = result.expect("Something went wrong.");
    println!(
        "Paint: {}us - Present: {}us",
        average_paint.as_micros(),
        average_present.as_micros()
    );
}
