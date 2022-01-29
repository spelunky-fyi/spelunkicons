use bitvec::order::Msb0;
use bitvec::view::BitView;
use image::png::PngEncoder;
use image::{ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;

const NUM_ROWS: usize = 6;
const NUM_COLS: usize = 6;

const TILE_WIDTH: u32 = 128;
const TILE_HEIGHT: u32 = 128;
const IMAGE_WIDTH: u32 = TILE_WIDTH * NUM_ROWS as u32;
const IMAGE_HEIGHT: u32 = TILE_HEIGHT * NUM_COLS as u32;

// We're going to be mirroring the icon vertically
// so we only care about half the columns.
const NUM_BITS_NEEDED: usize = NUM_ROWS * (NUM_COLS / 2);

pub struct Spelunkicon {
    grid: Vec<Vec<bool>>,
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

        Spelunkicon { grid }
    }

    pub fn print_grid(&self) {
        for row in &self.grid {
            for col in row {
                print!("{}", *col as i32);
            }
            println!("");
        }
    }

    pub fn to_png(&self) -> Option<Vec<u8>> {
        // Generate Image
        let mut image: RgbaImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

        let black = Rgba([0, 0, 0, 255]);
        for (row_idx, row) in self.grid.iter().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                if *col {
                    continue;
                }
                let rect = Rect::at(
                    col_idx as i32 * TILE_HEIGHT as i32,
                    row_idx as i32 * TILE_WIDTH as i32,
                )
                .of_size(TILE_WIDTH, TILE_HEIGHT);
                draw_filled_rect_mut(&mut image, rect, black);
            }
        }

        // Convert to PNG
        let mut out = Vec::new();
        let png = PngEncoder::new(&mut out).encode(
            image.as_raw(),
            IMAGE_WIDTH,
            IMAGE_HEIGHT,
            image::ColorType::Rgba8,
        );

        if png.is_err() {
            dbg!(&png);
        }

        png.map(|_| out).ok()
    }
}
