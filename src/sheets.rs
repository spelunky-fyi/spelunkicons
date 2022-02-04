use image::imageops::{flip_horizontal, overlay, FilterType};
use image::ImageFormat::Png;
use image::{load_from_memory_with_format, DynamicImage, SubImage};
use image::{GenericImageView, RgbaImage};
use rand::prelude::*;
use std::collections::HashMap;

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

    floormisc: DynamicImage,

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

            floormisc: load_from_memory_with_format(pngs::FLOORMISC, Png).unwrap(),

            items: load_from_memory_with_format(pngs::ITEMS, Png).unwrap(),
        }
    }

    fn sheet_floor_from_biome(&self, biome: &Biome) -> Option<&DynamicImage> {
        match biome {
            Biome::Cave => Some(&self.floor_cave),
            Biome::Jungle | Biome::Beehive => Some(&self.floor_jungle),
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
            Biome::Olmec => Some(&self.floorstyled_stone),

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
    Olmec,
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

    // FloorMisc
    AltarLeft,
    AltarRight,
    IdolAltarLeft,
    IdolAltarRight,
    EggplantAltarLeft,
    EggplantAltarRight,
    ArrowTrap,
    LaserTrap,
    TotemTrap,
    LionTrap,
    SpearTrap,
    FrogTrapLeft,
    FrogTrapRight,
    CrushTrap,
    LargeCrushTrapTopLeft,
    LargeCrushTrapTopRight,
    LargeCrushTrapBotLeft,
    LargeCrushTrapBotRight,
    BushBlock,
    BoneBlock,
    IceBlock,
    ChainTop,
    ChainMid,
    ChainBot,
    Platform,
    UdjatSocketTop,
    UdjatSocketBot,
    ConveyorLeft,
    ConveyorRight,
    PushBlock,
    PowderKeg,
    HoneyUp,
    HoneyDown,
}
type PlacedTileGrid = Vec<Vec<PlacedTile>>;

static DIR_NONE: (i64, i64) = (0, 0);
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

// Copy-Pasta from fenesd code
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

    pub fn base_tiles<'a>(&self, image: &'a DynamicImage) -> Vec<SubImage<&'a DynamicImage>> {
        match self {
            GenSheet::Floor(_) | GenSheet::FloorAndFloorStyled(_) => {
                vec![
                    image.view(0, 0, TILE_WIDTH, TILE_HEIGHT),
                    image.view(TILE_WIDTH, 0, TILE_WIDTH, TILE_HEIGHT),
                    image.view(0, TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
                    image.view(TILE_WIDTH, TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
                ]
            }
            _ => vec![],
        }
    }

    fn place_floor_tiles(
        &self,
        _biome: &Biome,
        config: &Spelunkicon,
        _rng: &mut StdRng,
    ) -> PlacedTileGrid {
        let mut grid =
            vec![vec![PlacedTile::None; config.grid_width as usize]; config.grid_height as usize];

        for (row_idx, row) in config.grid.iter().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                if *col {
                    continue;
                }

                // Mark that we placed a tile here
                grid[row_idx as usize][col_idx as usize] = PlacedTile::Floor;
            }
        }

        return grid;
    }

    fn place_floorstyled_tiles(
        &self,
        _biome: &Biome,
        config: &Spelunkicon,
        rng: &mut StdRng,
        existing_grid: Option<PlacedTileGrid>,
    ) -> PlacedTileGrid {
        let has_grid = existing_grid.is_some();
        let mut grid = existing_grid.unwrap_or_else(|| {
            vec![vec![PlacedTile::None; config.grid_width as usize]; config.grid_height as usize]
        });

        if has_grid {
            // Find a couple seeds for floorstyled, then do a small flood-fill
            for _ in 0..2 {
                let col_idx = rng.gen::<u32>() % config.grid_height as u32;
                let row_idx = rng.gen::<u32>() % config.grid_width as u32;

                if grid[row_idx as usize][col_idx as usize] == PlacedTile::Floor {
                    fn flood_fill(
                        x: usize,
                        y: usize,
                        depth: usize,
                        config: &Spelunkicon,
                        grid: &mut PlacedTileGrid,
                    ) {
                        if grid[y][x] == PlacedTile::Floor {
                            grid[y][x] = PlacedTile::FloorStyled;
                            if depth == 0 {
                                return;
                            }

                            if x > 0 {
                                flood_fill(x - 1, y, depth - 1, &config, grid);
                            }
                            if y > 0 {
                                flood_fill(x, y - 1, depth - 1, &config, grid);
                            }
                            if x < config.grid_width as usize - 1 {
                                flood_fill(x + 1, y, depth - 1, &config, grid);
                            }
                            if y < config.grid_height as usize - 1 {
                                flood_fill(x, y + 1, depth - 1, &config, grid);
                            }
                        }
                    }
                    flood_fill(col_idx as usize, row_idx as usize, 3, &config, &mut grid);
                }
            }
        } else {
            for (row_idx, row) in config.grid.iter().enumerate() {
                for (col_idx, col) in row.iter().enumerate() {
                    if *col {
                        continue;
                    }

                    // Just mark that we have a tile here, draw the actual tile later
                    grid[row_idx as usize][col_idx as usize] = PlacedTile::FloorStyled;
                }
            }
        }

        return grid;
    }

    fn place_floormisc_tiles(
        &self,
        biome: &Biome,
        config: &Spelunkicon,
        rng: &mut StdRng,
        grid: PlacedTileGrid,
    ) -> PlacedTileGrid {
        let mut grid = grid;

        // Don't wanna place multiple altars
        let mut placed_altar = false;

        // Try to place a few floormisc stuff
        // We are trying more than we want because most of those will be invalid anyways
        for _ in 0..config.max_misc {
            // Don't place these things right on the border, makes some stuff easier
            let col_idx = rng.gen::<usize>() % (config.grid_height as usize - 2) + 1;
            let row_idx = rng.gen::<usize>() % (config.grid_width as usize - 2) + 1;

            let pos = (col_idx, row_idx);
            let get_neighbour_empty = |dir, tile: Option<PlacedTile>| -> bool {
                neighbour_empty(config, &grid, pos, dir, tile)
            };
            let directions = HashMap::from([
                (DIR_NONE, get_neighbour_empty(DIR_NONE, None)),
                (DIR_LEFT, get_neighbour_empty(DIR_LEFT, None)),
                (DIR_DOWN_LEFT, get_neighbour_empty(DIR_DOWN_LEFT, None)),
                (DIR_DOWN, get_neighbour_empty(DIR_DOWN, None)),
                (DIR_DOWN_RIGHT, get_neighbour_empty(DIR_DOWN_RIGHT, None)),
                (DIR_RIGHT, get_neighbour_empty(DIR_RIGHT, None)),
                (DIR_UP_RIGHT, get_neighbour_empty(DIR_UP_RIGHT, None)),
                (DIR_UP, get_neighbour_empty(DIR_UP, None)),
                (DIR_UP_LEFT, get_neighbour_empty(DIR_UP_LEFT, None)),
            ]);

            let before = grid[row_idx][col_idx];

            match biome {
                Biome::Cave => {
                    let this = directions[&DIR_NONE];
                    if this {
                        let up = directions[&DIR_UP];
                        let down = directions[&DIR_DOWN];
                        if up && !down {
                            let left = directions[&DIR_LEFT];
                            let up_left = directions[&DIR_UP_LEFT];
                            let right = directions[&DIR_RIGHT];
                            let up_right = directions[&DIR_UP_RIGHT];

                            let misc_tile = if left && up_left && right && up_right {
                                PlacedTile::TotemTrap
                            } else {
                                PlacedTile::BoneBlock
                            };

                            grid[row_idx][col_idx] = misc_tile;
                            if row_idx > 0 {
                                grid[row_idx - 1][col_idx] = misc_tile;
                            }
                        }
                    } else {
                        let left = directions[&DIR_LEFT];
                        let right = directions[&DIR_RIGHT];
                        if left != right {
                            grid[row_idx][col_idx] = PlacedTile::ArrowTrap;
                        }
                    }
                }
                Biome::Jungle | Biome::Beehive => {
                    let this =
                        neighbour_empty(config, &grid, pos, DIR_NONE, Some(PlacedTile::Floor));
                    if !this {
                        let left = directions[&DIR_LEFT];
                        let right = directions[&DIR_RIGHT];
                        let up = directions[&DIR_UP];
                        let down = directions[&DIR_DOWN];
                        if left || right || up || down {
                            grid[row_idx][col_idx] = if rng.gen_bool(0.5) {
                                PlacedTile::SpearTrap
                            } else {
                                PlacedTile::BushBlock
                            };
                        }
                    } else {
                        match biome {
                            Biome::Beehive => {
                                let up_bee = neighbour_empty(
                                    config,
                                    &grid,
                                    pos,
                                    DIR_UP,
                                    Some(PlacedTile::FloorStyled),
                                );
                                if !up_bee {
                                    grid[row_idx][col_idx] = PlacedTile::HoneyUp;
                                } else {
                                    let down_bee = neighbour_empty(
                                        config,
                                        &grid,
                                        pos,
                                        DIR_DOWN,
                                        Some(PlacedTile::FloorStyled),
                                    );
                                    if !down_bee {
                                        grid[row_idx][col_idx] = PlacedTile::HoneyDown;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Biome::Volcana => {
                    let this = directions[&DIR_NONE];
                    if this {
                        let down = directions[&DIR_DOWN];
                        let up = directions[&DIR_UP];
                        if !down && rng.gen_bool(0.01) {
                            grid[row_idx - 1][col_idx] = PlacedTile::UdjatSocketTop;
                            grid[row_idx][col_idx] = PlacedTile::UdjatSocketBot;
                        } else if down && !up {
                            grid[row_idx - 1][col_idx] = PlacedTile::ChainTop;
                            for i in 0..4 {
                                if row_idx + i as usize == config.grid_height as usize - 1
                                    || i == 3
                                    || grid[row_idx + i][col_idx] != PlacedTile::None
                                {
                                    grid[row_idx + i][col_idx] = PlacedTile::ChainBot;
                                    break;
                                } else {
                                    grid[row_idx + i][col_idx] = PlacedTile::ChainMid;
                                }
                            }
                        }
                    } else {
                        let up = directions[&DIR_UP];
                        let down =
                            neighbour_empty(config, &grid, pos, DIR_DOWN, Some(PlacedTile::Floor));
                        if up && !down {
                            let flip = rng.gen_bool(0.5);
                            let tile = if flip {
                                PlacedTile::ConveyorLeft
                            } else {
                                PlacedTile::ConveyorRight
                            };

                            grid[row_idx][col_idx] = tile;

                            let left = directions[&DIR_LEFT];
                            let up_left = directions[&DIR_UP_LEFT];
                            let down_left = neighbour_empty(
                                config,
                                &grid,
                                pos,
                                DIR_DOWN_LEFT,
                                Some(PlacedTile::Floor),
                            );
                            if left && up_left && !down_left {
                                grid[row_idx][col_idx - 1] = tile;
                            }

                            let right = directions[&DIR_RIGHT];
                            let up_right = directions[&DIR_UP_RIGHT];
                            let down_right = neighbour_empty(
                                config,
                                &grid,
                                pos,
                                DIR_DOWN_RIGHT,
                                Some(PlacedTile::Floor),
                            );
                            if right && up_right && !down_right {
                                grid[row_idx][col_idx + 1] = tile;
                            }
                        }
                    }
                }
                Biome::TidePool => {
                    let this = directions[&DIR_NONE];
                    if this {
                        let up = directions[&DIR_UP];
                        let down = directions[&DIR_DOWN];
                        if up && !down {
                            let left = directions[&DIR_LEFT];
                            let up_left = directions[&DIR_UP_LEFT];
                            let right = directions[&DIR_RIGHT];
                            let up_right = directions[&DIR_UP_RIGHT];

                            if left && up_left && right && up_right {
                                grid[row_idx][col_idx] = PlacedTile::LionTrap;
                                if row_idx > 0 {
                                    grid[row_idx - 1][col_idx] = PlacedTile::LionTrap;
                                }
                            };
                        }
                    }
                }
                Biome::Temple | Biome::CityOfGold => {
                    let floor = Some(PlacedTile::Floor);
                    let floor_styled = Some(PlacedTile::FloorStyled);
                    let this = neighbour_empty(config, &grid, pos, DIR_NONE, floor)
                        && neighbour_empty(config, &grid, pos, DIR_NONE, floor_styled);
                    if !this {
                        let right = neighbour_empty(config, &grid, pos, DIR_RIGHT, floor)
                            && neighbour_empty(config, &grid, pos, DIR_RIGHT, floor_styled);
                        let down = neighbour_empty(config, &grid, pos, DIR_DOWN, floor)
                            && neighbour_empty(config, &grid, pos, DIR_DOWN, floor_styled);
                        let down_right = neighbour_empty(config, &grid, pos, DIR_DOWN_RIGHT, floor)
                            && neighbour_empty(config, &grid, pos, DIR_DOWN_RIGHT, floor_styled);
                        if !right && !down && !down_right && rng.gen_bool(0.5) {
                            grid[row_idx][col_idx] = PlacedTile::LargeCrushTrapTopLeft;
                            grid[row_idx][col_idx + 1] = PlacedTile::LargeCrushTrapTopRight;
                            grid[row_idx + 1][col_idx] = PlacedTile::LargeCrushTrapBotLeft;
                            grid[row_idx + 1][col_idx + 1] = PlacedTile::LargeCrushTrapBotRight;
                        } else {
                            let left = directions[&DIR_LEFT];
                            let up = directions[&DIR_UP];
                            if left || right || up || down {
                                grid[row_idx][col_idx] = PlacedTile::CrushTrap;
                            }
                        }
                    }
                }
                Biome::Ice => {
                    let this = directions[&DIR_NONE];
                    if !this {
                        grid[row_idx][col_idx] = PlacedTile::IceBlock;
                    }
                }
                Biome::Babylon => {
                    let this = directions[&DIR_NONE];
                    if !this {
                        let left = directions[&DIR_LEFT];
                        let right = directions[&DIR_RIGHT];
                        if left != right {
                            grid[row_idx][col_idx] = PlacedTile::LaserTrap;
                        }
                    }
                }
                Biome::Sunken => {
                    let this = directions[&DIR_NONE];
                    let left = directions[&DIR_LEFT];
                    let right = directions[&DIR_RIGHT];
                    if !this && (!left && right) {
                        grid[row_idx][col_idx - 1] = PlacedTile::FrogTrapLeft;
                        grid[row_idx][col_idx] = PlacedTile::FrogTrapRight;
                    } else if !this && (left && !right) {
                        grid[row_idx][col_idx] = PlacedTile::FrogTrapLeft;
                        grid[row_idx][col_idx + 1] = PlacedTile::FrogTrapRight;
                    }
                }
                _ => {}
            }

            // Do some generic stuff
            let this = directions[&DIR_NONE];
            if grid[row_idx][col_idx] == before && this {
                let left = directions[&DIR_LEFT];
                let right = directions[&DIR_RIGHT];
                let down = directions[&DIR_DOWN];
                let down_left = directions[&DIR_DOWN_LEFT];
                let down_right = directions[&DIR_DOWN_RIGHT];
                let up = directions[&DIR_UP];
                let up_left = directions[&DIR_UP_LEFT];
                let up_right = directions[&DIR_UP_RIGHT];

                if !down {
                    if !placed_altar
                        && rng.gen_bool(0.2)
                        && (this && up)
                        && ((left && !down_left && up_left) || (right && !down_right && up_right))
                    {
                        let (left_type, right_type) = match biome {
                            Biome::Cave | Biome::Volcana | Biome::TidePool => {
                                if rng.gen_bool(0.5) {
                                    (PlacedTile::AltarLeft, PlacedTile::AltarRight)
                                } else {
                                    (PlacedTile::IdolAltarLeft, PlacedTile::IdolAltarRight)
                                }
                            }
                            Biome::Ice => {
                                if rng.gen_bool(0.3333) {
                                    (PlacedTile::AltarLeft, PlacedTile::AltarRight)
                                } else if rng.gen_bool(0.5) {
                                    (
                                        PlacedTile::EggplantAltarLeft,
                                        PlacedTile::EggplantAltarRight,
                                    )
                                } else {
                                    (PlacedTile::IdolAltarLeft, PlacedTile::IdolAltarRight)
                                }
                            }
                            _ => (PlacedTile::AltarLeft, PlacedTile::AltarRight),
                        };

                        if left && !down_left && up_left {
                            grid[row_idx][col_idx - 1] = left_type;
                            grid[row_idx][col_idx] = right_type;
                        } else {
                            grid[row_idx][col_idx] = left_type;
                            grid[row_idx][col_idx + 1] = right_type;
                        }

                        placed_altar = true;
                    } else if this && (left || right) {
                        grid[row_idx][col_idx] = if rng.gen_bool(0.05) {
                            PlacedTile::PowderKeg
                        } else {
                            PlacedTile::PushBlock
                        }
                    }
                } else if up {
                    match biome {
                        Biome::Cave
                        | Biome::TidePool
                        | Biome::Surface
                        | Biome::PalaceOfPleasure
                        | Biome::Ice
                        | Biome::Volcana => {
                            grid[row_idx][col_idx] = PlacedTile::Platform;
                        }
                        _ => {}
                    }
                }
            }
        }

        return grid;
    }

    fn render_floor_tiles(
        &self,
        base_image: &mut RgbaImage,
        sheets: &Sheets,
        biome: &Biome,
        _config: &Spelunkicon,
        rng: &mut StdRng,
        grid: &PlacedTileGrid,
    ) {
        let sheet_image = sheets.sheet_floor_from_biome(biome).unwrap();

        let tile_images = self.base_tiles(sheet_image);

        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, tile) in row.iter().enumerate() {
                if *tile == PlacedTile::Floor {
                    let x = col_idx as u32 * TILE_HEIGHT as u32;
                    let y = row_idx as u32 * TILE_WIDTH as u32;

                    // Place down base tile
                    overlay(base_image, tile_images.choose(rng).unwrap(), x, y);
                }
            }
        }
    }

    fn render_floorstyled_tiles(
        &self,
        base_image: &mut RgbaImage,
        sheets: &Sheets,
        biome: &Biome,
        config: &Spelunkicon,
        _rng: &mut StdRng,
        grid: &PlacedTileGrid,
    ) {
        let sheet_image = sheets.sheet_floorstyled_from_biome(biome).unwrap();

        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, tile) in row.iter().enumerate() {
                if *tile == PlacedTile::FloorStyled {
                    let x = col_idx as u32 * TILE_HEIGHT as u32;
                    let y = row_idx as u32 * TILE_WIDTH as u32;

                    let pos = (col_idx, row_idx);
                    let get_neighbour_empty = |dir| -> bool {
                        neighbour_empty(config, &grid, pos, dir, Some(PlacedTile::FloorStyled))
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
    }

    fn render_floormisc_tiles(
        &self,
        base_image: &mut RgbaImage,
        sheets: &Sheets,
        biome: &Biome,
        config: &Spelunkicon,
        rng: &mut StdRng,
        grid: &PlacedTileGrid,
    ) {
        let floormisc = &sheets.floormisc;
        let biome_sheet = sheets
            .sheet_floor_from_biome(biome)
            .unwrap_or(&sheets.floor_cave);
        let floorstyled_biome_sheet = sheets
            .sheet_floorstyled_from_biome(biome)
            .unwrap_or(&sheets.floorstyled_stone);

        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, tile) in row.iter().enumerate() {
                let x = col_idx as u32 * TILE_HEIGHT as u32;
                let y = row_idx as u32 * TILE_WIDTH as u32;

                let pos = (col_idx, row_idx);
                let get_neighbour_empty =
                    |dir| -> bool { neighbour_empty(config, &grid, pos, dir, None) };

                let mut place_tile = |sheet: &DynamicImage, ix, iy| {
                    let tile_image =
                        sheet.view(ix * TILE_WIDTH, iy * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT);
                    overlay(base_image, &tile_image, x, y);
                };

                match tile {
                    PlacedTile::AltarLeft => {
                        place_tile(floormisc, 2, 0);
                    }
                    PlacedTile::AltarRight => {
                        place_tile(floormisc, 3, 0);
                    }
                    PlacedTile::IdolAltarLeft => {
                        place_tile(biome_sheet, 10, 0);
                    }
                    PlacedTile::IdolAltarRight => {
                        place_tile(biome_sheet, 11, 0);

                        let tile_image = sheets.items.view(
                            15 * TILE_WIDTH,
                            1 * TILE_HEIGHT,
                            TILE_WIDTH,
                            TILE_HEIGHT,
                        );
                        overlay(
                            base_image,
                            &tile_image,
                            x - TILE_WIDTH / 2,
                            y - TILE_HEIGHT + 18,
                        );
                    }
                    PlacedTile::EggplantAltarLeft => {
                        place_tile(biome_sheet, 11, 2);
                    }
                    PlacedTile::EggplantAltarRight => {
                        place_tile(biome_sheet, 11, 2);
                    }
                    PlacedTile::ArrowTrap | PlacedTile::LaserTrap => {
                        let (ix, iy) = match biome {
                            Biome::Sunken => (6, 0),
                            Biome::Babylon => (5, 4),
                            _ => (1, 0),
                        };

                        let tile_image = floormisc.view(
                            ix * TILE_WIDTH,
                            iy * TILE_HEIGHT,
                            TILE_WIDTH,
                            TILE_HEIGHT,
                        );

                        let left = get_neighbour_empty(DIR_LEFT);
                        if left {
                            let tile_image = flip_horizontal(&tile_image);
                            overlay(base_image, &tile_image, x, y);
                        } else {
                            overlay(base_image, &tile_image, x, y);
                        }
                    }
                    PlacedTile::TotemTrap | PlacedTile::LionTrap => {
                        let (ix, iy) = match biome {
                            Biome::TidePool => (5, 1),
                            _ => (4, 1),
                        };

                        let down = grid[row_idx - 1][col_idx] == *tile;
                        if !down {
                            place_tile(floormisc, ix, iy - 1);
                        } else {
                            place_tile(floormisc, ix, iy);
                        }
                    }
                    PlacedTile::SpearTrap => {
                        place_tile(floormisc, 5, 3);
                    }
                    PlacedTile::FrogTrapLeft => {
                        place_tile(biome_sheet, 8, 9);
                    }
                    PlacedTile::FrogTrapRight => {
                        place_tile(biome_sheet, 9, 9);
                    }
                    PlacedTile::CrushTrap => match biome {
                        Biome::CityOfGold => place_tile(floorstyled_biome_sheet, 9, 0),
                        _ => place_tile(floormisc, 0, 6),
                    },
                    PlacedTile::LargeCrushTrapTopLeft => match biome {
                        Biome::CityOfGold => place_tile(floorstyled_biome_sheet, 6, 0),
                        _ => place_tile(floormisc, 0, 4),
                    },
                    PlacedTile::LargeCrushTrapTopRight => match biome {
                        Biome::CityOfGold => place_tile(floorstyled_biome_sheet, 7, 0),
                        _ => place_tile(floormisc, 1, 4),
                    },
                    PlacedTile::LargeCrushTrapBotLeft => match biome {
                        Biome::CityOfGold => place_tile(floorstyled_biome_sheet, 6, 1),
                        _ => place_tile(floormisc, 0, 5),
                    },
                    PlacedTile::LargeCrushTrapBotRight => match biome {
                        Biome::CityOfGold => place_tile(floorstyled_biome_sheet, 7, 1),
                        _ => place_tile(floormisc, 1, 5),
                    },
                    PlacedTile::BushBlock => {
                        place_tile(biome_sheet, 10, 2);
                    }
                    PlacedTile::BoneBlock => {
                        place_tile(biome_sheet, 10, 2);
                    }
                    PlacedTile::IceBlock => {
                        let tile_image = biome_sheet.view(
                            7 * TILE_WIDTH,
                            1 * TILE_HEIGHT,
                            TILE_WIDTH,
                            TILE_HEIGHT,
                        );
                        let tile_image = DynamicImage::ImageRgba8(tile_image.to_image());

                        let overlap = 8;
                        let tile_image = tile_image.resize(
                            TILE_WIDTH + overlap,
                            TILE_HEIGHT + overlap,
                            FilterType::CatmullRom,
                        );
                        overlay(base_image, &tile_image, x - overlap / 2, y - overlap / 2);
                    }
                    PlacedTile::ChainTop => {
                        place_tile(biome_sheet, 4, 0);
                        place_tile(biome_sheet, 7, 1);
                    }
                    PlacedTile::ChainMid => {
                        place_tile(biome_sheet, 4, 1);
                    }
                    PlacedTile::ChainBot => {
                        place_tile(biome_sheet, 4, 2);
                        place_tile(biome_sheet, 7, 3);
                    }
                    PlacedTile::Platform => match biome {
                        Biome::Cave
                        | Biome::TidePool
                        | Biome::Surface
                        | Biome::PalaceOfPleasure => {
                            let (ix, iy) = match biome {
                                Biome::TidePool => (7, 3),
                                Biome::PalaceOfPleasure => (9, 2),
                                _ => (1, 1),
                            };
                            let sheet = match biome {
                                Biome::PalaceOfPleasure => &floorstyled_biome_sheet,
                                _ => &floormisc,
                            };

                            if grid[row_idx + 1][col_idx] != PlacedTile::None {
                                place_tile(sheet, ix - 1, iy);
                            } else {
                                place_tile(sheet, ix, iy);

                                let iy = iy + 1;
                                for i in 1..config.grid_height as u32 {
                                    let y = y + i * TILE_HEIGHT;
                                    let next_row_idx = row_idx + i as usize + 1;
                                    if next_row_idx == config.grid_height as usize
                                        || grid[next_row_idx as usize][col_idx] != PlacedTile::None
                                    {
                                        let iy = iy + 1;
                                        let tile_image = sheet.view(
                                            ix * TILE_WIDTH,
                                            iy * TILE_HEIGHT,
                                            TILE_WIDTH,
                                            TILE_HEIGHT,
                                        );
                                        overlay(base_image, &tile_image, x, y);

                                        break;
                                    } else {
                                        let tile_image = sheet.view(
                                            ix * TILE_WIDTH,
                                            iy * TILE_HEIGHT,
                                            TILE_WIDTH,
                                            TILE_HEIGHT,
                                        );
                                        overlay(base_image, &tile_image, x, y);
                                    }
                                }
                            }
                        }
                        Biome::Ice | Biome::Volcana => {
                            place_tile(biome_sheet, 4, 5);
                        }
                        _ => {}
                    },
                    PlacedTile::UdjatSocketTop => {
                        if rng.gen_bool(0.5) {
                            place_tile(floormisc, 5, 5);
                        } else {
                            place_tile(floormisc, 4, 5);
                        }
                    }
                    PlacedTile::UdjatSocketBot => {
                        place_tile(&sheets.floorstyled_babylon, 7, 2);
                    }
                    PlacedTile::ConveyorLeft => {
                        place_tile(biome_sheet, 11, 11);
                    }
                    PlacedTile::ConveyorRight => {
                        place_tile(biome_sheet, 11, 10);
                    }
                    PlacedTile::PushBlock => {
                        let sheet = match biome {
                            Biome::CityOfGold | Biome::Duat => floorstyled_biome_sheet,
                            Biome::Surface => &sheets.floor_cave,
                            _ => biome_sheet,
                        };
                        let (ix, iy) = match biome {
                            Biome::CityOfGold | Biome::Duat => (9, 0),
                            _ => (7, 0),
                        };
                        place_tile(sheet, ix, iy);
                    }
                    PlacedTile::PowderKeg => {
                        place_tile(floormisc, 2, 2);
                    }
                    PlacedTile::HoneyUp => {
                        let tile_image = sheets.items.view(
                            14 * TILE_WIDTH,
                            14 * TILE_HEIGHT,
                            TILE_WIDTH,
                            TILE_HEIGHT,
                        );
                        overlay(base_image, &tile_image, x, y - 22);
                    }
                    PlacedTile::HoneyDown => {
                        let tile_image = sheets.items.view(
                            13 * TILE_WIDTH,
                            14 * TILE_HEIGHT,
                            TILE_WIDTH,
                            TILE_HEIGHT,
                        );
                        overlay(base_image, &tile_image, x, y + 22);
                    }
                    _ => {}
                }
            }
        }
    }

    fn render_floor_decorations(
        &self,
        base_image: &mut RgbaImage,
        sheets: &Sheets,
        biome: &Biome,
        config: &Spelunkicon,
        rng: &mut StdRng,
        grid: &PlacedTileGrid,
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

        let has_spikes = match biome {
            Biome::Volcana
            | Biome::TidePool
            | Biome::Sunken
            | Biome::Jungle
            | Biome::Ice
            | Biome::Eggplant
            | Biome::Cave => true,
            _ => false,
        };
        let spikes = vec![
            sheet_image.view(5 * TILE_WIDTH, 9 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
            sheet_image.view(6 * TILE_WIDTH, 9 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
            sheet_image.view(7 * TILE_WIDTH, 9 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
        ];
        let spikes_deco = vec![
            sheet_image.view(5 * TILE_WIDTH, 8 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
            sheet_image.view(6 * TILE_WIDTH, 8 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
            sheet_image.view(7 * TILE_WIDTH, 8 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
        ];
        let spikes_blood = vec![
            sheet_image.view(5 * TILE_WIDTH, 10 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
            sheet_image.view(6 * TILE_WIDTH, 10 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
            sheet_image.view(7 * TILE_WIDTH, 10 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
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

                if grid[row_idx as usize][col_idx as usize] == PlacedTile::Floor {
                    let x = col_idx as u32 * TILE_HEIGHT as u32;
                    let y = row_idx as u32 * TILE_WIDTH as u32;

                    let pos = (col_idx, row_idx);
                    let get_neighbour_empty = |dir| -> bool {
                        neighbour_empty(config, &grid, pos, dir, Some(PlacedTile::Floor))
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
                        let y_deco = y - (TILE_HEIGHT / 2) + 4;
                        if has_spikes
                            && rng.gen::<u32>() % 12 == 0
                            && neighbour_empty(config, &grid, pos, DIR_UP, None)
                        {
                            let y = y - TILE_HEIGHT;
                            let spikes_choice = rng.gen_range(0..spikes.len());
                            overlay(base_image, &spikes[spikes_choice], x, y);
                            overlay(base_image, &spikes_deco[spikes_choice], x, y_deco);
                            if rng.gen_bool(0.1) {
                                overlay(base_image, &spikes_blood[spikes_choice], x, y);
                            }
                        } else {
                            overlay(base_image, up_deco.choose(rng).unwrap(), x, y_deco);
                        }
                    }
                }
            }
        }
    }

    fn render_floor_embeds(
        &self,
        base_image: &mut RgbaImage,
        sheets: &Sheets,
        config: &Spelunkicon,
        rng: &mut StdRng,
        grid: &PlacedTileGrid,
    ) {
        let crust_gold = vec![
            sheets
                .items
                .view(TILE_WIDTH * 10, 0, TILE_WIDTH, TILE_HEIGHT),
            sheets
                .items
                .view(TILE_WIDTH * 11, 0, TILE_WIDTH, TILE_HEIGHT),
        ];
        let crust_jewels = vec![
            sheets
                .items
                .view(TILE_WIDTH * 3, 0, TILE_WIDTH, TILE_HEIGHT),
            sheets
                .items
                .view(TILE_WIDTH * 4, 0, TILE_WIDTH, TILE_HEIGHT),
            sheets
                .items
                .view(TILE_WIDTH * 5, 0, TILE_WIDTH, TILE_HEIGHT),
        ];
        let crust_jetpack =
            sheets
                .items
                .view(TILE_WIDTH * 9, TILE_HEIGHT * 2, TILE_WIDTH, TILE_HEIGHT);

        for (row_idx, row) in config.grid.iter().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                if *col {
                    continue;
                }

                if grid[row_idx as usize][col_idx as usize] == PlacedTile::Floor {
                    let x = col_idx as u32 * TILE_HEIGHT as u32;
                    let y = row_idx as u32 * TILE_WIDTH as u32;

                    // Place Gold
                    if rng.gen::<u32>() % 12 == 0 {
                        overlay(base_image, crust_gold.choose(rng).unwrap(), x, y);
                    } else if rng.gen::<u32>() % 24 == 0 {
                        overlay(base_image, crust_jewels.choose(rng).unwrap(), x, y);
                    } else if rng.gen::<u32>() % 62000 == 0 {
                        overlay(base_image, &crust_jetpack, x, y);
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
                let floor = self.place_floor_tiles(biome, config, rng);
                let floor = self.place_floormisc_tiles(biome, config, rng, floor);

                self.render_floor_tiles(base_image, sheets, biome, config, rng, &floor);
                self.render_floormisc_tiles(base_image, sheets, biome, config, rng, &floor);
                self.render_floor_decorations(base_image, sheets, biome, config, rng, &floor);
                self.render_floor_embeds(base_image, sheets, config, rng, &floor);
            }
            GenSheet::FloorStyled(biome) => {
                let floor = self.place_floorstyled_tiles(biome, config, rng, None);
                let floor = self.place_floormisc_tiles(biome, config, rng, floor);

                self.render_floorstyled_tiles(base_image, sheets, biome, config, rng, &floor);
                self.render_floormisc_tiles(base_image, sheets, biome, config, rng, &floor);
            }
            GenSheet::FloorAndFloorStyled(biome) => {
                let floor = self.place_floor_tiles(biome, config, rng);
                let floor = self.place_floorstyled_tiles(biome, config, rng, Some(floor));
                let floor = self.place_floormisc_tiles(biome, config, rng, floor);

                self.render_floor_tiles(base_image, sheets, biome, config, rng, &floor);
                self.render_floorstyled_tiles(base_image, sheets, biome, config, rng, &floor);
                self.render_floormisc_tiles(base_image, sheets, biome, config, rng, &floor);
                self.render_floor_decorations(base_image, sheets, biome, config, rng, &floor);
                self.render_floor_embeds(base_image, sheets, config, rng, &floor);
            }
        }
    }
}
