use crate::input_event::InputEvent;
use crate::multiset::MultiSet;
use crate::ohlc::Ohlc;
use std::collections::{HashMap, VecDeque};

pub struct Window {
    time: i64,
    window_queue: HashMap<String, VecDeque<Event>>,
    sorted_prices_set: HashMap<String, MultiSet<i32>>,
}

impl Window {
    pub fn new(window_length: i64) -> Self {
        Window {
            time: window_length,
            window_queue: HashMap::new(),
            sorted_prices_set: HashMap::new(),
        }
    }

    pub fn add(&mut self, event: InputEvent) -> Ohlc {
        let event = Event::from(event);
        let sorted_prices_set = self
            .sorted_prices_set
            .entry(event.symbol.clone())
            .or_insert(MultiSet::new());
        let window_queue = self
            .window_queue
            .entry(event.symbol.clone())
            .or_insert(VecDeque::new());

        //Add the event to the queue
        window_queue.push_back(event.clone());
        sorted_prices_set.insert(event.price);

        //Remove the events from queue and its corresponding prices which are outside the window time frame
        while window_queue.front().unwrap().timestamp <= event.timestamp - self.time {
            println!("Removing event: {:?}", window_queue.front().unwrap());
            let event = window_queue.pop_front().unwrap();
            sorted_prices_set.remove(&event.price);
        }

        //Calculate the ohlc
        Ohlc {
            symbol: event.symbol,
            timestamp: event.timestamp,
            open: window_queue.front().unwrap().price as f64 / 100000000.0,
            high: *sorted_prices_set.iter().next_back().unwrap() as f64 / 100000000.0,
            low: *sorted_prices_set.iter().next().unwrap() as f64 / 100000000.0,
            close: window_queue.back().unwrap().price as f64 / 100000000.0,
        }
    }
}

//Event with only necessary stuff for now
#[derive(Debug, Clone)]
struct Event {
    symbol: String,
    timestamp: i64,
    price: i32,
}

impl From<InputEvent> for Event {
    fn from(input_event: InputEvent) -> Self {
        Event {
            symbol: input_event.s,
            timestamp: input_event.T,
            price: {
                //A workaround for the fact that f64 does not implement ord trait
                let mut price = (input_event.a + input_event.b) / 2.0;
                price *= 100000000.0;
                price as i32
            },
        }
    }
}
