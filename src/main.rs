use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   #[clap(short, long, value_parser)]
   filename: String,
   
   #[clap(short, long, value_parser)]
   column: String,
}

fn main() {
   let args = Args::parse();

   println!("We are going to open: {}, and look for {}", args.filename, args.column);
}
