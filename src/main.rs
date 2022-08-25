use clap::Parser;
use csv::{ReaderBuilder,Reader};
use std::fs::File;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   #[clap(short, long, value_parser, help="CSV location")]
   path: String,
   
   #[clap(short, long, value_parser, help="Column name (as in headers, case-sensitive)")]
   column: String,

   #[clap(short='l', long, action, help="Print line numbers before column value")]
   line_numbers: bool,

   #[clap(short, long, action)]
   verbose: bool,

   #[clap(short, long, action, help="Print only odd rows")]
   odds: bool,

   #[clap(short, long, action, help="Print only even rows")]
   evens: bool,

   #[clap(short, long, value_parser, help="How many rows to seek before print (Zero-indexed)")]
   seek: Option<usize>,

   #[clap(short, long, value_parser, help="Number of rows to print")]
   number: Option<usize>,
}

fn main() {
    let args = Args::parse();

    if args.verbose {
        println!("----");
        println!("* File: {}", args.path);
        println!("* Header Column: {}", args.column);
    }

    match ReaderBuilder::new()
        .from_path(args.path.clone()) {
            Ok(reader) => process_csv(reader, args),
            Err(error) => println!("Cannot read CSV: {}", error),
    }
}

fn process_csv(mut reader: Reader<File>, args: Args)
{
    let headers = reader.headers()
        .expect("Cannot read CSV headers");

    let col_index = headers.iter().position(|r| r == args.column)
        .expect("Header not found");
    if args.verbose {
        println!("* Header found at column: {}", col_index + 1);
        println!("----");
    }

    // User options
    let number = match args.number {
        Some(number) => number,
        None => 0
    };
    let seek = match args.seek {
        Some(seek) => seek,
        None => 0
    };

    // Output data
    let mut count = 0;

    // Output data
    for (i, row) in reader.records().enumerate() {
        if number > 0 && count >= number { break; }
        if i < seek || (!(args.odds && args.evens) &&
            ((args.odds && i % 2 == 0) || (args.evens && i % 2 != 0))
        ) {
            continue;
        }

        match row {
            Ok(result) => {
                let value = result.get(col_index).unwrap();

                if args.line_numbers {
                    println!("{:?}: {}", result.position().unwrap().line(), value);
                } else {
                    println!("{}", value);
                }
                count += 1;
            },
            Err(_) => {}
        }
    }
}

