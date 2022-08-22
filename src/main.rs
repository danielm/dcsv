use clap::Parser;
use csv::ReaderBuilder;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   #[clap(short, long, value_parser)]
   path: String,
   
   #[clap(short, long, value_parser)]
   column: String,
}

fn main() {
    let args = Args::parse();

    // println!("We are going to open: {}, and look for {}", args.filename, args.column);

    match ReaderBuilder::new()
        .delimiter(b',')
        // .has_headers(false)
        .from_path(args.path) {
            Ok(mut reader) => {
                if let Ok(headers) = reader.headers() {
                    // println!("{:?}", headers);
                    if let Some(col_index) = headers.iter().position(|r| r == args.column) {
                        // println!("Found header at index: {}", colIndex);
                        while let Some(Ok(result)) = reader.records().next() {
                            if let Some(value) = result.get(col_index) {
                                println!("{:?}: {}", result.position().unwrap().line(), value);
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
