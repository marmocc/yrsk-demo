use yrsk::{
    painters::chaos::Chaos,
    rendering::{ansi::terminal::Terminal, renderer::Renderer},
};

fn main() {
    let result;

    {
        let mut chaos = Chaos;
        let mut surface = Terminal::new().expect("Unable to create Terminal.");
        result = surface.benchmark(&mut chaos, 10_000);
    }

    match result {
        Ok(timings) => println!(
            "Paint: {} - Play: {} - Present: {}",
            timings.paint.as_micros(),
            timings.play.as_micros(),
            timings.present.as_micros()
        ),
        Err(error) => println!("{}", error),
    }
}
