use std::collections::VecDeque;
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
    #[arg(short = 'l')]
    show_line_number: bool,
    /// PATTERN is regular expression
    #[arg(short)]
    regex: bool,
}

fn main() {
    let args = Cli::parse();

    let reader: Box<dyn BufRead> = if args.file_path == "-" {
        let stdin = io::stdin();
        Box::new(stdin.lock())
    } else {
        let file = File::open(args.file_path.clone()).unwrap();
        Box::new(BufReader::new(file))
    };

    if let Some(ctx) = args.ctx {
        search_with_context(reader, ctx, args);
    } else {
        search_without_context(reader, args);
    }
}

fn search_without_context<T: BufRead>(reader: T, args: Cli) {
    let re = Regex::new(&args.pattern).unwrap();

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let is_match = if args.regex {
            re.find(&line).is_some()
        } else {
            line.contains(&args.pattern)
        };

        if is_match {
            if args.show_line_number {
                println!("{}:{}", i + 1, line)
            } else {
                println!("{}", line)
            }
        }
    }
}

fn search_with_context<T: BufRead>(reader: T, ctx: u8, args: Cli) {
    let re = Regex::new(&args.pattern).unwrap();
    let mut ctx_buffer = VecDeque::with_capacity(ctx.into());

    let mut after_match = false;
    let mut print_separator = false;
    let mut first_match = true;

    for (line_number, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let is_match = if args.regex {
            re.find(&line).is_some()
        } else {
            line.contains(&args.pattern)
        };

        if is_match {
            if print_separator && !after_match && !first_match {
                println!("---")
            }

            first_match = false;

            print_ctx(&mut ctx_buffer, args.show_line_number);
            print_line(&line, line_number, args.show_line_number);

            after_match = true;
            continue;
        }

        if ctx_buffer.len() == ctx.into() {
            if after_match {
                print_ctx(&mut ctx_buffer, args.show_line_number);
                after_match = false;
                print_separator = false;
            } else {
                ctx_buffer.pop_front();
                print_separator = true;
            }
        }

        ctx_buffer.push_back((line_number, line));
    }

    if after_match {
        print_ctx(&mut ctx_buffer, args.show_line_number);
    }
}

fn print_line(line: &str, line_number: usize, show_line_number: bool) {
    if show_line_number {
        println!("{}:{}", line_number + 1, line);
    } else {
        println!("{}", line);
    }
}

fn print_ctx(ctx_buffer: &mut VecDeque<(usize, String)>, show_line_number: bool) {
    while let Some((line_number, line)) = ctx_buffer.pop_front() {
        print_line(&line, line_number, show_line_number);
    }
}
