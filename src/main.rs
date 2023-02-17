use anyhow::Result;
use ohlc::{InputEvent, Window};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Error: Expected exactly two arguments.");
        return Ok(());
    }

    let input_path = &args[1];
    let output_path = &args[2];

    process_ticker_events(input_path, output_path)?;

    Ok(())
}


fn process_ticker_events(input_path: &str, output_path: &str) -> Result<()> {
    let input_file = File::open(input_path)?;
    let input_reader = BufReader::new(input_file);
    let mut output_file = File::create(output_path)?;

    let mut window = Window::new(300000);

    for line in input_reader.lines() {
        let line = line?;
        let input_event: InputEvent = line.parse::<InputEvent>()?;

        let output = window.add(input_event);
        let output = serde_json::to_string(&output)?;
        writeln!(output_file, "{}", output)?;
    }

    Ok(())
}