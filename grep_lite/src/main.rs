use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::Parser;
use regex::Regex;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// PATTERN to search for
    pattern: String,
    /// Path to the input file (use '-' for stdin)
    file_path: String,
    /// Print NUM lines of output context
    #[arg(short, value_name = "NUM")]
    ctx: Option<u8>,
    /// Print line number with output lines
    #[arg(short)]
    line_number: bool,
    /// PATTERN is regular expression
    #[arg(short)]
    regex: bool,
}

fn main() {
    let args = Cli::parse();

    if args.file_path == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();

        search_lines(reader, args);
    } else {
        let file = File::open(args.file_path.clone()).unwrap();
        let reader = BufReader::new(file);

        search_lines(reader, args);
    }
}

fn search_lines<T: BufRead>(reader: T, args: Cli) {
    match args.ctx {
        Some(ctx) => with_context(reader, ctx, args),
        None => without_context(reader, args),
    }
}

fn without_context<T: BufRead>(reader: T, args: Cli) {
    let re = Regex::new(&args.pattern).unwrap();

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let is_match = if args.regex {
            re.find(&line).is_some()
        } else {
            line.contains(&args.pattern)
        };

        if is_match {
            if args.line_number {
                println!("{}:{}", i + 1, line)
            } else {
                println!("{}", line)
            }
        }
    }
}

fn with_context<T: BufRead>(reader: T, ctx: u8, args: Cli) {
    let lines = reader.lines().collect::<Result<Vec<_>, _>>().unwrap();

    let re = Regex::new(&args.pattern).unwrap();

    let matches: Vec<_> = lines
        .iter()
        .enumerate()
        .filter(|(_, l)| {
            if args.regex {
                re.find(l).is_some()
            } else {
                l.contains(&args.pattern)
            }
        })
        .map(|(i, _)| i)
        .collect();

    let mut evaluated_line_numbers = HashSet::new();

    for (i, &line_number) in matches.iter().enumerate() {
        let start = line_number.saturating_sub(ctx.into());

        for (j, line) in lines
            .iter()
            .enumerate()
            .skip(start)
            .take((ctx * 2 + 1).into())
        {
            if evaluated_line_numbers.insert(j) {
                if j == start && i != 0 {
                    println!("---")
                }
                if args.line_number {
                    println!("{}:{}", j + 1, line)
                } else {
                    println!("{}", line)
                }
            }
        }
    }
}
