use clap::Parser;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;

#[derive(Debug, Default)]
struct File {
    records: Vec<Vec<String>>,
}

impl File {
    fn new<R: Read>(reader: R, delim: &str) -> Self {
        let mut records: Vec<Vec<String>> = Vec::new();

        let buffer = BufReader::new(reader);

        for line in buffer.lines() {
            let record: Vec<String> = line.unwrap().split(delim).map(|s| s.to_string()).collect();

            records.push(record);
        }

        Self { records }
    }

    fn display_field(&self, field: usize) {
        for record in &self.records {
            if field < record.len() {
                println!("{}", record[field]);
            }
        }
    }
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short)]
    field: Option<usize>,

    #[arg(long, short)]
    bytes: Option<usize>,

    #[arg(long, short)]
    character: Option<usize>,

    #[arg(value_name = "FILE")]
    file: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    // read from file or stdin
    let file = if args.file.is_some() {
        File::new(std::fs::File::open(args.file.unwrap()).unwrap(), "\t")
    } else {
        File::new(std::io::stdin(), "\t")
    };

    if args.field.is_some() {
        file.display_field(args.field.unwrap())
    }
}
