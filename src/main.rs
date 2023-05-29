use std::{fs, io::Seek};

use clap::Parser;

use crate::snbt::ToSnbt;

mod nbt;
mod snbt;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the NBT file
    #[arg(short, long)]
    path: String,

    /// How many bytes to skip at the start of the file
    #[arg(short, long, default_value_t = 0)]
    skip: u64,
}

fn main() {
    let args = Args::parse();

    let mut file = fs::File::open(args.path).expect("failed to open file");
    file.seek(std::io::SeekFrom::Start(args.skip))
        .expect("failed to seek to start");

    let mut tag_reader = nbt::TagReader::new(&mut file);
    let root_tag = tag_reader.read_tag().expect("failed to read root tag");

    println!("{}", root_tag.to_snbt(0));
}
