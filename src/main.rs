use clap::Parser;
use csv::ReaderBuilder;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   #[clap(short, long, value_parser)]
   path: String,
   
   #[clap(short, long, value_parser)]
   column: String,

   #[clap(short='n', long, action)]
   line_number: bool,

   // #[clap(short='h', long, action)]
   // no_headers: bool,

   #[clap(short, long, action)]
   verbose: bool,
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
        .from_path(args.path) {
            Ok(mut reader) => {
                if let Ok(headers) = reader.headers() {
                    if let Some(col_index) = headers.iter().position(|r| r == args.column) {
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
                            } else {
                                println!("Failed to access value column");
                            }
                        }
                    } else {
                        println!("Cannot find the requested column: '{}'", args.column);
                    }
                } else {
                    println!("Cannot read headers");
                }
            },
            Err(error) => println!("Cannot read CSV: {}", error),
    }
    
}
