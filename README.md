# DCSV

Simple CSV utility to fetch/print data from CSV files and perform some really odd operations.

## About

I like to experiment with data so I use CSV files alot; I'm also learning Rust => This repo is a combination of both.

For me this simple application is a boilerplate for my real usage, I got alot of commented that I will be adding overtime as I clean it up and learn more about Rust.

## Usage

```
    $ dcsv -p path/to/file/.csv -c Name
```

### Options

- `-c, --column <COLUMN>`: Column name (as in headers, case-sensitive)
- `-e, --evens`: Print only even rows 
- `-l, --line-numbers`: Print line numbers before column value
- `-n, --number <NUMBER>`: Number of rows to print
- `-o, --odds`: Print only odd rows
- `-p, --path <PATH>`: CSV location
- `-s, --seek <SEEK>`: How many rows to seek before print (Zero-indexed)
