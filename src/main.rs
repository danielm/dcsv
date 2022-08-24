use clap::Parser;
use csv::{ReaderBuilder,Reader};
use std::fs::File;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   #[clap(short, long, value_parser)]
   path: String,
   
   #[clap(short, long, value_parser)]
   column: String,

   #[clap(short='l', long, action)]
   line_number: bool,

   #[clap(short, long, action)]
   verbose: bool,

//    #[clap(short, long, value_parser)]
//    seek: Option<u32>,

//    #[clap(short, long, value_parser)]
//    number: Option<u32>,
}

fn main() {
    let args = Args::parse();

    if args.verbose {
        println!("----");
        println!("* File: {}", args.path);
        println!("* Header Column: {}", args.column);
    }

    match ReaderBuilder::new()
        .delimiter(b',')
        // .has_headers(args.has_headers)
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

    // Output data
    while let Some(Ok(result)) = reader.records().next() {
        if let Some(value) = result.get(col_index) {
            if args.line_number {
                println!("{:?}: {}", result.position().unwrap().line(), value);
            } else {
                println!("{}", value);
            }
        }
    }
}

