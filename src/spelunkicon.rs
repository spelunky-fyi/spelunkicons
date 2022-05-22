use bitvec::order::Msb0;
use bitvec::view::BitView;

pub struct Spelunkicon {
    pub grid_width: u8,
    pub grid_height: u8,
    pub max_misc: u8,
    pub input: String,
    pub egg: Option<String>,
    pub hash: u32,
    pub grid: Vec<Vec<bool>>,
}

impl Spelunkicon {
    pub fn from_input(input: &str, egg: Option<String>, height: u8, max_misc: u8) -> Self {
        let grid_width = height;
        let grid_height = height;

        let mut grid = Vec::with_capacity(grid_width as usize);

        let hash = crc32fast::hash(input.as_bytes());
        let odd_size = grid_width % 2 == 1;
        let center_bits_needed = if odd_size { height } else { 0 } as usize;
        let side_bits_needed = (height * (height / 2)) as usize;
        let bits_needed = side_bits_needed + center_bits_needed;

        let bits: Vec<bool> = hash
            .view_bits::<Msb0>()
            .into_iter()
            .take(bits_needed)
            .map(|x| x.as_ref().clone())
            .collect();
        let center_bits: Vec<bool> = bits
            .iter()
            .rev()
            .take(center_bits_needed)
            .map(|x| x.clone())
            .collect();
        let side_bits: Vec<bool> = bits
            .iter()
            .take(side_bits_needed)
            .map(|x| x.clone())
            .collect();

        for (i, row) in side_bits.chunks((height / 2) as usize).enumerate() {
            let mut grid_row = Vec::with_capacity(height as usize);
            for col in row {
                grid_row.push(*col);
            }
            // Add center column for odd sizes
            if odd_size {
                grid_row.push(center_bits[i])
            }
            // Mirror of left side
            for col in row.iter().rev() {
                grid_row.push(*col);
            }
            grid.push(grid_row);
        }

        let input = input.to_string();
        Spelunkicon {
            grid_height,
            grid_width,
            max_misc,
            input,
            egg,
            hash,
            grid,
        }
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
