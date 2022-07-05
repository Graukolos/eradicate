use std::env;
use clap::Parser;
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::Path;

fn main() {
    let args = Args::parse();
    let path_to_known_hosts = format!("{}/.ssh/known_hosts", env::var("HOME").expect("Failed to find home directory"));
    let path_to_known_hosts = Path::new(&path_to_known_hosts);
    let mut known_hosts_buffer = String::new();
    let mut known_hosts = File::open(path_to_known_hosts).expect("Failed to open file");
    known_hosts.read_to_string(&mut known_hosts_buffer).expect("Failed to read file");

    let mut known_hosts_writer = BufWriter::new(File::create(path_to_known_hosts).expect("Failed to open file"));

    let mut eradicated = false;
    for line in known_hosts_buffer.lines() {
        if !line.starts_with(&args.host) {
            known_hosts_writer.write_all(line.as_bytes()).expect("Failed to write to file");
            known_hosts_writer.write_all("\n".as_bytes()).expect("Failed to write to file");
        } else {
            println!("{}", line);
            eradicated = true;
        }
    }
    if eradicated {
        println!("\nSuccesfully eradicated!");
    } else {
        println!("\nFailure, target not found");
    }
}

#[derive(Parser)]
#[clap(version)]
struct Args {
    host: String
}