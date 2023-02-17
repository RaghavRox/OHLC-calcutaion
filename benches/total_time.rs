use ohlc::{self, Window, InputEvent};
use criterion::{self, criterion_group, Criterion, criterion_main};
use anyhow;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};


fn process_ticker_events(input_path: &str, output_path: &str) -> anyhow::Result<()> {
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

fn benchmark(c: &mut Criterion) {
    c.bench_function("total time including IO", |b| b.iter(|| process_ticker_events("./data/dataset-b.txt", "./data/ohlc-5m-b.txt").unwrap()));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

