use bitvec::order::Msb0;
use bitvec::view::BitView;

pub struct Spelunkicon {
    pub height: u8,
    pub hash: u32,
    pub grid: Vec<Vec<bool>>,
}

impl Spelunkicon {
    pub fn from_input(input: &str, height: u8) -> Self {
        let mut grid = Vec::with_capacity(height as usize);

        let hash = crc32fast::hash(input.as_bytes());
        let bits_needed = height * (height / 2);

        let bits: Vec<bool> = hash
            .view_bits::<Msb0>()
            .into_iter()
            .take(bits_needed as usize)
            .map(|x| x.as_ref().clone())
            .collect();

        for row in bits.chunks((height / 2) as usize) {
            let mut grid_row = Vec::with_capacity(height as usize);
            for col in row {
                grid_row.push(*col);
            }
            // Mirror
            for col in row.iter().rev() {
                grid_row.push(*col);
            }
            grid.push(grid_row);
        }

        Spelunkicon { hash, height, grid }
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
