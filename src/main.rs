#[macro_use(value_t)]
extern crate clap;
extern crate simple_signal;

use simple_signal::{Signal, Signals};
use clap::{Arg, App};
use std::f64;
use std::process;
use std::io;
use std::io::Write;
use std::time;
use std::thread;
use std::sync::mpsc::sync_channel;

fn input() -> io::Result<String> {
    let mut string = String::new();

    let bytes_read = try!(io::stdin().read_line(&mut string));
    if bytes_read == 0 {
        return Err(io::Error::new(io::ErrorKind::Other,
                                  "Could not read line."));
    }

    string = string.trim_right_matches("\r\n").to_owned();
    string = string.trim_right_matches("\n").to_owned();

    return Ok(string);
}

fn roll1(string: String, reverse: bool) -> String {
    if string.len() < 1 {
        return string;
    }

    let mut new_string = String::from("");
    let mut chars = string.chars();

    if !reverse {
        let first = chars.next();

        for c in chars {
            new_string.push(c);
        }

        if let Some(val) = first {
            new_string.push(val);
        }
    } else {
        if let Some(val) = chars.clone().last() {
            new_string.push(val)
        }

        for c in chars.take(string.len() - 1) {
            new_string.push(c);
        }
    }

    return new_string;
}

fn open_loop(interval: time::Duration,
             mutate: bool,
             separator: String,
             truncate: bool,
             truncate_len: usize,
             reverse: bool,
             count: f64,
             prefix: String,
             postfix: String) {
    let (tx, rx) = sync_channel(0);
    let inp_thread = thread::spawn(move || {
        loop {
            match input() {
                Ok(val) => {
                    tx.send(val).unwrap();
                },
                Err(_) => break
            };
        }
    });

    let mut string = String::new();
    let mut i: f64 = 0f64;
    while i < count {
        match rx.try_recv() {
            Ok(val) => {
                string = val;
                if (truncate && truncate_len < string.len()) || !truncate {
                        string.push_str(&separator);
                }
            },
            Err(_) => {
                if (truncate && truncate_len < string.len()) || !truncate {
                        string = roll1(string, reverse);
                }
            },
        }

        let mut modified_string = string.clone();

        if truncate {
            modified_string.truncate(truncate_len);
        }

        if mutate {
            print!("{}{}{}\r", prefix, modified_string, postfix);
            let _ = io::stdout().flush();
        } else {
            println!("{}{}{}", prefix, modified_string, postfix);
        }

        i += 1f64;
        thread::sleep(interval);
    }

    inp_thread.join().unwrap();
}

fn main() {
    let args = App::new("Roller")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Kabir Goel <kabirgoel.kg@gmail.com>")
        .about("Truncate text by rolling it like a news ticker.")
        .arg(Arg::with_name("interval")
            .short("i")
            .long("interval")
            .takes_value(true)
            .value_name("INTERVAL")
            .help("Set a custom interval in milliseconds"))
        .arg(Arg::with_name("mutate")
            .short("m")
            .long("mutate")
            .help("Roll in place instead of on separate lines"))
        .arg(Arg::with_name("separator")
            .short("s")
            .long("separator")
            .takes_value(true)
            .value_name("SEPARATOR")
            .help("Place a separator between consecutive rolls"))
        .arg(Arg::with_name("truncate")
            .short("t")
            .long("truncate")
            .takes_value(true)
            .value_name("LENGTH")
            .help("Only roll if text is longer than LENGTH, \
                  effectively truncating it"))
        .arg(Arg::with_name("count")
            .short("c")
            .long("count")
            .takes_value(true)
            .value_name("NUMBER")
            .help("Only roll this many times"))
        .arg(Arg::with_name("reverse")
            .short("r")
            .long("reverse")
            .help("Roll in reverse"))
        .arg(Arg::with_name("prefix")
            .short("b")
            .long("prefix")
            .takes_value(true)
            .value_name("PREFIX")
            .help("Prepend a static prefix to the text"))
        .arg(Arg::with_name("postfix")
            .short("a")
            .long("postfix")
            .takes_value(true)
            .value_name("POSTFIX")
            .help("Append a static postfix to the text"))
        .get_matches();

    let count = value_t!(args, "count", f64).unwrap_or(f64::INFINITY);
    let raw_interval = value_t!(args, "interval", u64).unwrap_or(200);
    let interval = time::Duration::from_millis(raw_interval);
    let reverse = args.is_present("reverse");
    let truncate = args.is_present("truncate");
    let truncate_len = value_t!(args, "truncate", usize).unwrap_or(80);
    let separator = String::from(args.value_of("separator").unwrap_or(" "));
    let prefix = String::from(args.value_of("prefix").unwrap_or(""));
    let postfix = String::from(args.value_of("postfix").unwrap_or(""));
    let mutate = args.is_present("mutate");

    Signals::set_handler(&[Signal::Pipe], |_signals| {
        process::exit(0);
    });

    open_loop(interval,
              mutate,
              separator,
              truncate,
              truncate_len,
              reverse,
              count,
              prefix,
              postfix);
}
