use image::imageops::{flip_horizontal, overlay};
use image::ImageFormat::Png;
use image::{load_from_memory_with_format, DynamicImage, SubImage};
use image::{GenericImageView, RgbaImage};
use rand::prelude::*;

use crate::constants::{TILE_HEIGHT, TILE_WIDTH};
use crate::pngs;
use crate::spelunkicon::Spelunkicon;

pub struct Sheets {
    floor_cave: DynamicImage,
    floor_jungle: DynamicImage,
    floor_babylon: DynamicImage,
    floor_eggplant: DynamicImage,
    floor_ice: DynamicImage,
    floor_sunken: DynamicImage,
    floor_surface: DynamicImage,
    floor_temple: DynamicImage,
    floor_tidepool: DynamicImage,
    floor_volcano: DynamicImage,

    floorstyled_vlad: DynamicImage,
    floorstyled_wood: DynamicImage,
    floorstyled_babylon: DynamicImage,
    floorstyled_beehive: DynamicImage,
    floorstyled_duat: DynamicImage,
    floorstyled_gold: DynamicImage,
    floorstyled_guts: DynamicImage,
    floorstyled_mothership: DynamicImage,
    floorstyled_pagoda: DynamicImage,
    floorstyled_palace: DynamicImage,
    floorstyled_stone: DynamicImage,
    floorstyled_sunken: DynamicImage,
    floorstyled_temple: DynamicImage,

    items: DynamicImage,
}

impl Sheets {
    pub fn new() -> Self {
        Self {
            floor_cave: load_from_memory_with_format(pngs::FLOOR_CAVE, Png).unwrap(),
            floor_jungle: load_from_memory_with_format(pngs::FLOOR_JUNGLE, Png).unwrap(),
            floor_babylon: load_from_memory_with_format(pngs::FLOOR_BABYLON, Png).unwrap(),
            floor_eggplant: load_from_memory_with_format(pngs::FLOOR_EGGPLANT, Png).unwrap(),
            floor_ice: load_from_memory_with_format(pngs::FLOOR_ICE, Png).unwrap(),
            floor_sunken: load_from_memory_with_format(pngs::FLOOR_SUNKEN, Png).unwrap(),
            floor_surface: load_from_memory_with_format(pngs::FLOOR_SURFACE, Png).unwrap(),
            floor_temple: load_from_memory_with_format(pngs::FLOOR_TEMPLE, Png).unwrap(),
            floor_tidepool: load_from_memory_with_format(pngs::FLOOR_TIDEPOOL, Png).unwrap(),
            floor_volcano: load_from_memory_with_format(pngs::FLOOR_VOLCANO, Png).unwrap(),

            floorstyled_vlad: load_from_memory_with_format(pngs::FLOORSTYLED_VLAD, Png).unwrap(),
            floorstyled_wood: load_from_memory_with_format(pngs::FLOORSTYLED_WOOD, Png).unwrap(),
            floorstyled_babylon: load_from_memory_with_format(pngs::FLOORSTYLED_BABYLON, Png)
                .unwrap(),
            floorstyled_beehive: load_from_memory_with_format(pngs::FLOORSTYLED_BEEHIVE, Png)
                .unwrap(),
            floorstyled_duat: load_from_memory_with_format(pngs::FLOORSTYLED_DUAT, Png).unwrap(),
            floorstyled_gold: load_from_memory_with_format(pngs::FLOORSTYLED_GOLD, Png).unwrap(),
            floorstyled_guts: load_from_memory_with_format(pngs::FLOORSTYLED_GUTS, Png).unwrap(),
            floorstyled_mothership: load_from_memory_with_format(pngs::FLOORSTYLED_MOTHERSHIP, Png)
                .unwrap(),
            floorstyled_pagoda: load_from_memory_with_format(pngs::FLOORSTYLED_PAGODA, Png)
                .unwrap(),
            floorstyled_palace: load_from_memory_with_format(pngs::FLOORSTYLED_PALACE, Png)
                .unwrap(),
            floorstyled_stone: load_from_memory_with_format(pngs::FLOORSTYLED_STONE, Png).unwrap(),
            floorstyled_sunken: load_from_memory_with_format(pngs::FLOORSTYLED_SUNKEN, Png)
                .unwrap(),
            floorstyled_temple: load_from_memory_with_format(pngs::FLOORSTYLED_TEMPLE, Png)
                .unwrap(),

            items: load_from_memory_with_format(pngs::ITEMS, Png).unwrap(),
        }
    }

    fn sheet_floor_from_biome(&self, biome: &Biome) -> Option<&DynamicImage> {
        match biome {
            Biome::Cave => Some(&self.floor_cave),
            Biome::Jungle => Some(&self.floor_jungle),
            Biome::Babylon => Some(&self.floor_babylon),
            Biome::Eggplant => Some(&self.floor_eggplant),
            Biome::Ice => Some(&self.floor_ice),
            Biome::Sunken => Some(&self.floor_sunken),
            Biome::Surface => Some(&self.floor_surface),
            Biome::Temple => Some(&self.floor_temple),
            Biome::TidePool => Some(&self.floor_tidepool),
            Biome::Volcana => Some(&self.floor_volcano),

            _ => None,
        }
    }

    fn sheet_floorstyled_from_biome(&self, biome: &Biome) -> Option<&DynamicImage> {
        match biome {
            Biome::Cave => Some(&self.floorstyled_wood),
            Biome::Jungle => Some(&self.floorstyled_stone),
            Biome::Babylon => Some(&self.floorstyled_babylon),
            Biome::Sunken => Some(&self.floorstyled_sunken),
            Biome::Temple => Some(&self.floorstyled_temple),
            Biome::TidePool => Some(&self.floorstyled_pagoda),

            Biome::Beehive => Some(&self.floorstyled_beehive),
            Biome::Vlad => Some(&self.floorstyled_vlad),
            Biome::CityOfGold => Some(&self.floorstyled_gold),
            Biome::Duat => Some(&self.floorstyled_duat),
            Biome::Mothership => Some(&self.floorstyled_mothership),
            Biome::PalaceOfPleasure => Some(&self.floorstyled_palace),
            Biome::Guts => Some(&self.floorstyled_guts),

            _ => None,
        }
    }
}

pub enum Biome {
    Cave,
    Jungle,
    Beehive,
    Babylon,
    PalaceOfPleasure,
    Eggplant,
    Ice,
    Mothership,
    Sunken,
    Guts,
    Surface,
    Temple,
    CityOfGold,
    Duat,
    TidePool,
    Volcana,
    Vlad,
}

pub enum GenKind {
    Floor,
    FloorStyled,
    FloorAndFloorStyled,
}

pub enum GenSheet {
    Floor(Biome),
    FloorStyled(Biome),
    FloorAndFloorStyled(Biome),
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum PlacedTile {
    None,
    Floor,
    FloorStyled,
}
type PlacedTileGrid = Vec<Vec<PlacedTile>>;

static DIR_LEFT: (i64, i64) = (-1, 0);
static DIR_UP_LEFT: (i64, i64) = (-1, -1);
static DIR_UP: (i64, i64) = (0, -1);
static DIR_UP_RIGHT: (i64, i64) = (1, -1);
static DIR_RIGHT: (i64, i64) = (1, 0);
static DIR_DOWN_RIGHT: (i64, i64) = (1, 1);
static DIR_DOWN: (i64, i64) = (0, 1);
static DIR_DOWN_LEFT: (i64, i64) = (-1, 1);
fn neighbour_empty(
    config: &Spelunkicon,
    grid: &PlacedTileGrid,
    pos: (usize, usize),
    dir: (i64, i64),
    filled_type: Option<PlacedTile>,
) -> bool {
    let (x, y) = pos;
    let (dx, dy) = dir;

    if x as i64 + dx < 0 || x as i64 + dx >= config.grid_width as i64 {
        return false;
    }

    if y as i64 + dy < 0 || y as i64 + dy >= config.grid_height as i64 {
        return false;
    }

    let placed = grid[(y as i64 + dy) as usize][(x as i64 + dx) as usize];
    if filled_type.is_some() {
        let filled = filled_type.unwrap();
        return placed != filled;
    } else {
        return placed == PlacedTile::None;
    }
}

// Copy-Pasta from fensed code
fn get_floor_styled_texture_coords(neighbour_mask: u8) -> (u32, u32) {
    let nth_bit = |n| -> bool { ((neighbour_mask >> n) & 0b1u8) == 0b1u8 };

    let left = nth_bit(0);
    let down_left = nth_bit(1);
    let down = nth_bit(2);
    let down_right = nth_bit(3);
    let right = nth_bit(4);
    let up_right = nth_bit(5);
    let up = nth_bit(6);
    let up_left = nth_bit(7);

    if !left && down && !down_right && right && !up {
        return (4, 2);
    }
    if left && !down_left && down && !down_right && right && !up {
        return (5, 2);
    }
    if left && !down_left && down && !right && !up {
        return (6, 2);
    }
    if left && !down_left && down && !right && up && !up_left {
        return (6, 3);
    }
    if left && !down && !right && up && !up_left {
        return (6, 4);
    }
    if left && !down && right && !up_right && up && !up_left {
        return (5, 4);
    }
    if !left && !down && right && !up_right && up {
        return (4, 4);
    }
    if !left && down && !down_right && right && !up_right && up {
        return (4, 3);
    }

    if !left && !down && !right && !up {
        return (7, 2);
    }

    if !left && down && !right && !up {
        return (3, 2);
    }
    if !left && down && !right && up {
        return (3, 3);
    }
    if !left && !down && !right && up {
        return (3, 4);
    }

    if !left && !down && right && !up {
        return (0, 5);
    }
    if left && !down && right && !up {
        return (1, 5);
    }
    if left && !down && !right && !up {
        return (2, 5);
    }

    if !left && down && right && !up {
        return (0, 2);
    }
    if left && down_left && down && down_right && right && !up {
        return (1, 2);
    }
    if left && down && !right && !up {
        return (2, 2);
    }

    if !left && !down && right && up {
        return (0, 4);
    }
    if left && !down && right && up_right && up && up_left {
        return (1, 4);
    }
    if left && !down && !right && up {
        return (2, 4);
    }

    if !left && down && down_right && right && up_right && up {
        return (0, 3);
    }
    if left && down_left && down && !right && up && up_left {
        return (2, 3);
    }

    if neighbour_mask == 0b01111111 {
        return (0, 0);
    }
    if neighbour_mask == 0b11011111 {
        return (1, 0);
    }
    if neighbour_mask == 0b11110111 {
        return (1, 1);
    }
    if neighbour_mask == 0b11111101 {
        return (0, 1);
    }

    if left && !down_left && down && down_right && right && !up {
        return (2, 0);
    }
    if left && down_left && down && !down_right && right && !up {
        return (3, 0);
    }
    if left && !down && right && up_right && up && !up_left {
        return (2, 1);
    }
    if left && !down && right && !up_right && up && up_left {
        return (3, 1);
    }

    if !left && down && !down_right && right && up_right && up {
        return (0, 6);
    }
    if left && !down_left && down && !right && up && up_left {
        return (1, 6);
    }
    if !left && down && down_right && right && !up_right && up {
        return (0, 7);
    }
    if left && down_left && down && !right && up && !up_left {
        return (1, 7);
    }

    if neighbour_mask == 0b01011111 {
        return (4, 0);
    }
    if neighbour_mask == 0b11110101 {
        return (4, 1);
    }
    if neighbour_mask == 0b01111101 {
        return (5, 0);
    }
    if neighbour_mask == 0b11010111 {
        return (5, 1);
    }

    if neighbour_mask == 0b01110111 {
        return (3, 5);
    }
    if neighbour_mask == 0b11011101 {
        return (4, 5);
    }

    if neighbour_mask == 0b01011101 {
        return (2, 6);
    }
    if neighbour_mask == 0b01010111 {
        return (3, 6);
    }
    if neighbour_mask == 0b11010101 {
        return (3, 7);
    }
    if neighbour_mask == 0b01110101 {
        return (2, 7);
    }

    if neighbour_mask == 0b11111111 {
        return (1, 3);
    }
    if neighbour_mask == 0b01010101 {
        return (5, 3);
    }

    return (1, 3);
}

impl GenSheet {
    pub fn new(kind: GenKind, biome: Biome) -> Self {
        match kind {
            GenKind::Floor => GenSheet::Floor(biome),
            GenKind::FloorStyled => GenSheet::FloorStyled(biome),
            GenKind::FloorAndFloorStyled => GenSheet::FloorAndFloorStyled(biome),
        }
    }

    pub fn base_tile<'a>(&self, image: &'a DynamicImage) -> SubImage<&'a DynamicImage> {
        match self {
            GenSheet::Floor(_) | GenSheet::FloorAndFloorStyled(_) => {
                image.view(0, 0, TILE_WIDTH, TILE_HEIGHT)
            }
            GenSheet::FloorStyled(_) => {
                image.view(1 * TILE_WIDTH, 3 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT)
            }
        }
    }

    fn place_floor_tiles(
        &self,
        base_image: &mut RgbaImage,
        sheets: &Sheets,
        biome: &Biome,
        config: &Spelunkicon,
        _rng: &mut StdRng,
    ) -> PlacedTileGrid {
        let sheet_image = sheets.sheet_floor_from_biome(biome).unwrap();

        let tile = self.base_tile(sheet_image);

        let mut placed_grid =
            vec![vec![PlacedTile::None; config.grid_width as usize]; config.grid_height as usize];

        for (row_idx, row) in config.grid.iter().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                if *col {
                    continue;
                }
                let x = col_idx as u32 * TILE_HEIGHT as u32;
                let y = row_idx as u32 * TILE_WIDTH as u32;

                // Place down base tile
                overlay(base_image, &tile, x, y);

                // Mark that we placed a tile here
                placed_grid[row_idx as usize][col_idx as usize] = PlacedTile::Floor;
            }
        }

        return placed_grid;
    }

    fn place_floor_decorations(
        &self,
        base_image: &mut RgbaImage,
        sheets: &Sheets,
        biome: &Biome,
        config: &Spelunkicon,
        rng: &mut StdRng,
        existing_grid: &PlacedTileGrid,
    ) {
        let sheet_image = sheets.sheet_floor_from_biome(biome).unwrap();

        let right_deco = vec![
            sheet_image.view(5 * TILE_WIDTH, 5 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
            sheet_image.view(6 * TILE_WIDTH, 5 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
        ];
        let right_up_deco =
            sheet_image.view(7 * TILE_WIDTH, 5 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT);

        let left_deco = vec![
            flip_horizontal(&right_deco[0]),
            flip_horizontal(&right_deco[1]),
        ];
        let left_up_deco = flip_horizontal(&right_up_deco);

        let up_deco = vec![
            sheet_image.view(5 * TILE_WIDTH, 6 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
            sheet_image.view(6 * TILE_WIDTH, 6 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
            sheet_image.view(7 * TILE_WIDTH, 6 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
        ];

        let spikes_deco = vec![
            sheet_image.view(5 * TILE_WIDTH, 8 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
            sheet_image.view(6 * TILE_WIDTH, 8 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
            sheet_image.view(7 * TILE_WIDTH, 8 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
        ];

        let down_deco = vec![
            sheet_image.view(5 * TILE_WIDTH, 7 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
            sheet_image.view(6 * TILE_WIDTH, 7 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
            sheet_image.view(7 * TILE_WIDTH, 7 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
        ];

        for (row_idx, row) in config.grid.iter().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                if *col {
                    continue;
                }

                if existing_grid[row_idx as usize][col_idx as usize] == PlacedTile::Floor {
                    let x = col_idx as u32 * TILE_HEIGHT as u32;
                    let y = row_idx as u32 * TILE_WIDTH as u32;

                    let pos = (col_idx, row_idx);
                    let get_neighbour_empty = |dir| -> bool {
                        neighbour_empty(config, &existing_grid, pos, dir, Some(PlacedTile::Floor))
                    };

                    let left = get_neighbour_empty(DIR_LEFT);
                    let right = get_neighbour_empty(DIR_RIGHT);
                    let up = get_neighbour_empty(DIR_UP);
                    let down = get_neighbour_empty(DIR_DOWN);

                    // Place generic deco
                    if left {
                        let x = x - (TILE_WIDTH / 2) + 5;
                        if up {
                            overlay(base_image, &left_up_deco, x, y);
                        } else {
                            overlay(base_image, left_deco.choose(rng).unwrap(), x, y);
                        }
                    }

                    if right {
                        let x = x + (TILE_WIDTH / 2) - 5;
                        if up {
                            overlay(base_image, &right_up_deco, x, y);
                        } else {
                            overlay(base_image, right_deco.choose(rng).unwrap(), x, y);
                        }
                    }

                    if down {
                        let y = y + (TILE_HEIGHT / 2) - 12;
                        overlay(base_image, down_deco.choose(rng).unwrap(), x, y);
                    }

                    // Place generic top-deco or spikes top-deco
                    if up {
                        let y = y - (TILE_HEIGHT / 2) + 4;
                        if rng.gen::<u32>() % 12 == 0 {
                            overlay(base_image, spikes_deco.choose(rng).unwrap(), x, y);
                        } else {
                            overlay(base_image, up_deco.choose(rng).unwrap(), x, y);
                        }
                    }
                }
            }
        }
    }

    fn place_floorstyled_tiles(
        &self,
        base_image: &mut RgbaImage,
        sheets: &Sheets,
        biome: &Biome,
        config: &Spelunkicon,
        _rng: &mut StdRng,
        existing_grid: Option<PlacedTileGrid>,
    ) -> PlacedTileGrid {
        let sheet_image = sheets.sheet_floorstyled_from_biome(biome).unwrap();

        let has_existing_grid = existing_grid.is_some();
        let mut placed_grid = existing_grid.unwrap_or_else(|| {
            vec![vec![PlacedTile::None; config.grid_width as usize]; config.grid_height as usize]
        });

        if has_existing_grid {
        } else {
            for (row_idx, row) in config.grid.iter().enumerate() {
                for (col_idx, col) in row.iter().enumerate() {
                    if *col {
                        continue;
                    }

                    // Just mark that we have a tile here, draw the actual tile later
                    placed_grid[row_idx as usize][col_idx as usize] = PlacedTile::FloorStyled;
                }
            }
        }

        for (row_idx, row) in config.grid.iter().enumerate() {
            for (col_idx, _) in row.iter().enumerate() {
                if placed_grid[row_idx as usize][col_idx as usize] == PlacedTile::FloorStyled {
                    let x = col_idx as u32 * TILE_HEIGHT as u32;
                    let y = row_idx as u32 * TILE_WIDTH as u32;

                    let pos = (col_idx, row_idx);
                    let get_neighbour_empty = |dir| -> bool {
                        neighbour_empty(
                            config,
                            &placed_grid,
                            pos,
                            dir,
                            Some(PlacedTile::FloorStyled),
                        )
                    };

                    let directions = [
                        get_neighbour_empty(DIR_LEFT),
                        get_neighbour_empty(DIR_DOWN_LEFT),
                        get_neighbour_empty(DIR_DOWN),
                        get_neighbour_empty(DIR_DOWN_RIGHT),
                        get_neighbour_empty(DIR_RIGHT),
                        get_neighbour_empty(DIR_UP_RIGHT),
                        get_neighbour_empty(DIR_UP),
                        get_neighbour_empty(DIR_UP_LEFT),
                    ];

                    let mut neighbour_mask: u8 = 0;
                    for (dir_idx, dir) in directions.iter().enumerate() {
                        if !*dir {
                            let neighbour_bit = 0b1u8 << dir_idx;
                            neighbour_mask |= neighbour_bit;
                        }
                    }

                    let (ix, iy) = get_floor_styled_texture_coords(neighbour_mask);
                    let tile = sheet_image.view(
                        ix * TILE_WIDTH,
                        iy * TILE_HEIGHT,
                        TILE_WIDTH,
                        TILE_HEIGHT,
                    );

                    // Place down tile tile
                    overlay(base_image, &tile, x, y);
                }
            }
        }
        return placed_grid;
    }

    fn place_floor_embeds(
        &self,
        base_image: &mut RgbaImage,
        sheets: &Sheets,
        config: &Spelunkicon,
        rng: &mut StdRng,
        existing_grid: &PlacedTileGrid,
    ) {
        let crust_gold = vec![
            sheets
                .items
                .view(TILE_WIDTH * 10, 0, TILE_WIDTH, TILE_HEIGHT),
            sheets
                .items
                .view(TILE_WIDTH * 11, 0, TILE_WIDTH, TILE_HEIGHT),
        ];

        for (row_idx, row) in config.grid.iter().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                if *col {
                    continue;
                }

                if existing_grid[row_idx as usize][col_idx as usize] == PlacedTile::Floor {
                    let x = col_idx as u32 * TILE_HEIGHT as u32;
                    let y = row_idx as u32 * TILE_WIDTH as u32;

                    // Place Gold
                    if rng.gen::<u32>() % 12 == 0 {
                        overlay(base_image, crust_gold.choose(rng).unwrap(), x, y);
                    }
                }
            }
        }
    }

    pub fn generate_image(
        &self,
        base_image: &mut RgbaImage,
        sheets: &Sheets,
        config: &Spelunkicon,
        rng: &mut StdRng,
    ) {
        match self {
            GenSheet::Floor(biome) => {
                let floor = self.place_floor_tiles(base_image, sheets, biome, config, rng);
                self.place_floor_decorations(base_image, sheets, biome, config, rng, &floor);
                self.place_floor_embeds(base_image, sheets, config, rng, &floor);
            }
            GenSheet::FloorStyled(biome) => {
                self.place_floorstyled_tiles(base_image, sheets, biome, config, rng, None);
            }
            GenSheet::FloorAndFloorStyled(biome) => {
                let floor = self.place_floor_tiles(base_image, sheets, biome, config, rng);
                let floor = self.place_floorstyled_tiles(
                    base_image,
                    sheets,
                    biome,
                    config,
                    rng,
                    Some(floor),
                );
                self.place_floor_decorations(base_image, sheets, biome, config, rng, &floor);
                self.place_floor_embeds(base_image, sheets, config, rng, &floor);
            }
        }
    }
}
