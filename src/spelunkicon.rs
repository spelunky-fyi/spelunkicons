use bitvec::order::Msb0;
use bitvec::view::BitView;

use crate::constants::{NUM_BITS_NEEDED, NUM_COLS, NUM_ROWS};

pub struct Spelunkicon {
    pub hash: u32,
    pub grid: Vec<Vec<bool>>,
}

impl Spelunkicon {
    pub fn from_input(input: &str) -> Self {
        let mut grid = Vec::with_capacity(NUM_ROWS);

        let hash = crc32fast::hash(input.as_bytes());

        let bits: Vec<bool> = hash
            .view_bits::<Msb0>()
            .into_iter()
            .take(NUM_BITS_NEEDED)
            .map(|x| x.as_ref().clone())
            .collect();

        for row in bits.chunks(3) {
            let mut grid_row = Vec::with_capacity(NUM_COLS);
            for col in row {
                grid_row.push(*col);
            }
            // Mirrow
            for col in row.iter().rev() {
                grid_row.push(*col);
            }
            grid.push(grid_row);
        }

        Spelunkicon { hash, grid }
    }

    pub fn print_grid(&self) {
        for row in &self.grid {
            for col in row {
                print!("{}", *col as i32);
            }
            println!("");
        }
    }
}
