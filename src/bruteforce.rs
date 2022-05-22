use std::process::exit;

use bitvec::order::Msb0;
use bitvec::view::BitView;
use clap::Parser;
use itertools::Itertools;

/// Brute force a grid for spelunkicons
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// The input to find.
    #[clap(short, long)]
    input: String,

    /// The size of the grid.
    #[clap(short, long, default_value_t = 6)]
    size: u8,
}

fn main() {
    let args = Args::parse();

    if args.size < 3 || args.size > 8 {
        eprintln!("Size must be in [3, 8]");
        exit(1);
    }

    let bits_needed = args.size * (args.size / 2);

    if args.input.len() != bits_needed as usize {
        eprintln!("Input must be {} characters", bits_needed);
        exit(1)
    }

    let word_len = 4;
    let corpus: Vec<char> = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();

    for permutation in corpus.iter().permutations(word_len) {
        let string: String = permutation.iter().map(|x| **x).collect();
        let hash = crc32fast::hash(string.as_bytes());

        let bits: String = hash
            .view_bits::<Msb0>()
            .into_iter()
            .take(18)
            .map(|x| (x.as_ref().clone() as u8).to_string())
            .collect();

        if bits == args.input {
            println!("Found {}", string);
        }
    }
}
