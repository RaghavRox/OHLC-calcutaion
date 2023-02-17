## Description

A Rust library crate that provides a way of computing rolling OHLC (open-high-low-close) for a stream of numeric 
prices and timestamps for a given time window.

E.g., if the window is 5 minutes, once a new price/timestamp is given,it returns the rolling 5-minute OHLC over the last 
5 minutes - the earliest price in the time period, the highest/lowest prices and the latest price.

## Instructions to run
The program takes two command line arguments.
```
cargo run --release <input_file> <output_file>
```
Example:
```
cargo run --release ./files/input.txt ./files/output.txt
```

## How to use the crate
The crate provides 2 main structs InputEvent and Window
1)Create a window object with desired time in milliseconds
```
let mut window = Window::new(300000);
```
2)Parse a json string to get an InputEvent
```
let input_event: InputEvent = line.parse::<InputEvent>()?;
```
3)Pass the InputEvent to the window object to receive the OHLC values
```
let output = window.add(input_event);
```


## Dataset

Two files with ticker updates for several symbols. Each line represents a single event with bid/ask price 
and quantity and a timestamp `T` . The binary crate calculates rolling OHLC for a 5 minutes window.

