pub(crate) const NUM_ROWS: usize = 6;
pub(crate) const NUM_COLS: usize = 6;

pub(crate) const TILE_WIDTH: u32 = 128;
pub(crate) const TILE_HEIGHT: u32 = 128;
pub(crate) const IMAGE_WIDTH: u32 = TILE_WIDTH * NUM_ROWS as u32;
pub(crate) const IMAGE_HEIGHT: u32 = TILE_HEIGHT * NUM_COLS as u32;

// We're going to be mirroring the icon vertically
// so we only care about half the columns.
pub(crate) const NUM_BITS_NEEDED: usize = NUM_ROWS * (NUM_COLS / 2);
