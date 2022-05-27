use image::imageops::{flip_horizontal, overlay, FilterType};
use image::ImageFormat::Png;
use image::{load_from_memory_with_format, DynamicImage, SubImage};
use image::{GenericImageView, RgbaImage};
use rand::prelude::*;

use crate::constants::{TILE_HEIGHT, TILE_WIDTH};
use crate::grid_generator::*;
use crate::pngs::*;
use crate::sheets::Biome;
use crate::spelunkicon::Spelunkicon;

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

pub struct Sheets {
    floor_cave: [DynamicImage; 2],
    floor_jungle: [DynamicImage; 2],
    floor_babylon: [DynamicImage; 2],
    floor_eggplant: [DynamicImage; 2],
    floor_ice: [DynamicImage; 2],
    floor_sunken: [DynamicImage; 2],
    floor_surface: [DynamicImage; 2],
    floor_temple: [DynamicImage; 2],
    floor_tidepool: [DynamicImage; 2],
    floor_volcano: [DynamicImage; 2],

    floorstyled_vlad: [DynamicImage; 2],
    floorstyled_wood: [DynamicImage; 2],
    floorstyled_babylon: [DynamicImage; 2],
    floorstyled_beehive: [DynamicImage; 2],
    floorstyled_duat: [DynamicImage; 2],
    floorstyled_gold: [DynamicImage; 2],
    floorstyled_guts: [DynamicImage; 2],
    floorstyled_mothership: [DynamicImage; 2],
    floorstyled_pagoda: [DynamicImage; 2],
    floorstyled_palace: [DynamicImage; 2],
    floorstyled_stone: [DynamicImage; 2],
    floorstyled_sunken: [DynamicImage; 2],
    floorstyled_temple: [DynamicImage; 2],

    floormisc: [DynamicImage; 2],
    basecamp_deco: [DynamicImage; 2],

    items: [DynamicImage; 2],
}

fn load_images(spel2_data: &[u8], classic_data: &[u8]) -> [DynamicImage; 2] {
    [
        load_from_memory_with_format(spel2_data, Png).unwrap(),
        load_from_memory_with_format(classic_data, Png).unwrap(),
    ]
}

impl Sheets {
    pub fn new() -> Self {
        Self {
            floor_cave: load_images(FLOOR_CAVE, CLASSIC_FLOOR_CAVE),
            floor_jungle: load_images(FLOOR_JUNGLE, CLASSIC_FLOOR_JUNGLE),
            floor_babylon: load_images(FLOOR_BABYLON, CLASSIC_FLOOR_BABYLON),
            floor_eggplant: load_images(FLOOR_EGGPLANT, CLASSIC_FLOOR_EGGPLANT),
            floor_ice: load_images(FLOOR_ICE, CLASSIC_FLOOR_ICE),
            floor_sunken: load_images(FLOOR_SUNKEN, CLASSIC_FLOOR_SUNKEN),
            floor_surface: load_images(FLOOR_SURFACE, CLASSIC_FLOOR_SURFACE),
            floor_temple: load_images(FLOOR_TEMPLE, CLASSIC_FLOOR_TEMPLE),
            floor_tidepool: load_images(FLOOR_TIDEPOOL, CLASSIC_FLOOR_TIDEPOOL),
            floor_volcano: load_images(FLOOR_VOLCANO, CLASSIC_FLOOR_VOLCANO),

            floorstyled_vlad: load_images(FLOORSTYLED_VLAD, CLASSIC_FLOORSTYLED_VLAD),
            floorstyled_wood: load_images(FLOORSTYLED_WOOD, CLASSIC_FLOORSTYLED_WOOD),
            floorstyled_babylon: load_images(FLOORSTYLED_BABYLON, CLASSIC_FLOORSTYLED_BABYLON),
            floorstyled_beehive: load_images(FLOORSTYLED_BEEHIVE, CLASSIC_FLOORSTYLED_BEEHIVE),
            floorstyled_duat: load_images(FLOORSTYLED_DUAT, CLASSIC_FLOORSTYLED_DUAT),
            floorstyled_gold: load_images(FLOORSTYLED_GOLD, CLASSIC_FLOORSTYLED_GOLD),
            floorstyled_guts: load_images(FLOORSTYLED_GUTS, CLASSIC_FLOORSTYLED_GUTS),
            floorstyled_mothership: load_images(
                FLOORSTYLED_MOTHERSHIP,
                CLASSIC_FLOORSTYLED_MOTHERSHIP,
            ),
            floorstyled_pagoda: load_images(FLOORSTYLED_PAGODA, CLASSIC_FLOORSTYLED_PAGODA),
            floorstyled_palace: load_images(FLOORSTYLED_PALACE, CLASSIC_FLOORSTYLED_PALACE),
            floorstyled_stone: load_images(FLOORSTYLED_STONE, CLASSIC_FLOORSTYLED_STONE),
            floorstyled_sunken: load_images(FLOORSTYLED_SUNKEN, CLASSIC_FLOORSTYLED_SUNKEN),
            floorstyled_temple: load_images(FLOORSTYLED_TEMPLE, CLASSIC_FLOORSTYLED_TEMPLE),
            floormisc: load_images(FLOORMISC, CLASSIC_FLOORMISC),
            basecamp_deco: load_images(BASECAMP_DECO, CLASSIC_BASECAMP_DECO),

            items: load_images(ITEMS, CLASSIC_ITEMS),
        }
    }

    pub fn sheet_floor_from_biome(&self, biome: &Biome, classic: bool) -> Option<&DynamicImage> {
        let idx = if classic { 1 } else { 0 };
        match biome {
            Biome::Cave => Some(&self.floor_cave[idx]),
            Biome::Jungle | Biome::Beehive => Some(&self.floor_jungle[idx]),
            Biome::Babylon => Some(&self.floor_babylon[idx]),
            Biome::Eggplant => Some(&self.floor_eggplant[idx]),
            Biome::Ice => Some(&self.floor_ice[idx]),
            Biome::Sunken => Some(&self.floor_sunken[idx]),
            Biome::Surface => Some(&self.floor_surface[idx]),
            Biome::Temple => Some(&self.floor_temple[idx]),
            Biome::TidePool => Some(&self.floor_tidepool[idx]),
            Biome::Volcana => Some(&self.floor_volcano[idx]),

            _ => None,
        }
    }

    pub fn sheet_floorstyled_from_biome(
        &self,
        biome: &Biome,
        classic: bool,
    ) -> Option<&DynamicImage> {
        let idx = if classic { 1 } else { 0 };
        match biome {
            Biome::Cave => Some(&self.floorstyled_wood[idx]),
            Biome::Jungle => Some(&self.floorstyled_stone[idx]),
            Biome::Babylon => Some(&self.floorstyled_babylon[idx]),
            Biome::Sunken => Some(&self.floorstyled_sunken[idx]),
            Biome::Temple => Some(&self.floorstyled_temple[idx]),
            Biome::TidePool => Some(&self.floorstyled_pagoda[idx]),

            Biome::Beehive => Some(&self.floorstyled_beehive[idx]),
            Biome::Vlad => Some(&self.floorstyled_vlad[idx]),
            Biome::CityOfGold => Some(&self.floorstyled_gold[idx]),
            Biome::Duat => Some(&self.floorstyled_duat[idx]),
            Biome::Mothership => Some(&self.floorstyled_mothership[idx]),
            Biome::PalaceOfPleasure => Some(&self.floorstyled_palace[idx]),
            Biome::Guts => Some(&self.floorstyled_guts[idx]),
            Biome::Olmec => Some(&self.floorstyled_stone[idx]),

            _ => None,
        }
    }

    pub fn get_floormisc(&self, classic: bool) -> &DynamicImage {
        let idx = if classic { 1 } else { 0 };
        return &self.floormisc[idx];
    }

    pub fn get_basecamp_deco(&self, classic: bool) -> &DynamicImage {
        let idx = if classic { 1 } else { 0 };
        return &self.basecamp_deco[idx];
    }

    pub fn get_items(&self, classic: bool) -> &DynamicImage {
        let idx = if classic { 1 } else { 0 };
        return &self.items[idx];
    }
}

pub struct GridRenderer {
    classic_mode: bool,
}

impl GridRenderer {
    pub fn new(classic_mode: bool) -> Self {
        return Self { classic_mode };
    }

    fn base_tiles<'a>(&self, image: &'a DynamicImage) -> Vec<SubImage<&'a DynamicImage>> {
        return vec![
            image.view(0, 0, TILE_WIDTH, TILE_HEIGHT),
            image.view(TILE_WIDTH, 0, TILE_WIDTH, TILE_HEIGHT),
            image.view(0, TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
            image.view(TILE_WIDTH, TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
        ];
    }

    pub fn render_floor_tiles(
        &self,
        base_image: &mut RgbaImage,
        sheets: &Sheets,
        biome: &Biome,
        _config: &Spelunkicon,
        rng: &mut StdRng,
        grid: &PlacedTileGrid,
    ) {
        let sheet_image = sheets
            .sheet_floor_from_biome(biome, self.classic_mode)
            .unwrap();

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

    pub fn render_floorstyled_tiles(
        &self,
        base_image: &mut RgbaImage,
        sheets: &Sheets,
        biome: &Biome,
        config: &Spelunkicon,
        _rng: &mut StdRng,
        grid: &PlacedTileGrid,
    ) {
        let sheet_image = sheets
            .sheet_floorstyled_from_biome(biome, self.classic_mode)
            .unwrap();

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

    pub fn render_floormisc_tiles(
        &self,
        base_image: &mut RgbaImage,
        sheets: &Sheets,
        biome: &Biome,
        config: &Spelunkicon,
        rng: &mut StdRng,
        grid: &PlacedTileGrid,
    ) {
        let floormisc = sheets.get_floormisc(self.classic_mode);
        let biome_sheet = sheets
            .sheet_floor_from_biome(biome, self.classic_mode)
            .unwrap_or(
                sheets
                    .sheet_floor_from_biome(&Biome::Cave, self.classic_mode)
                    .unwrap(),
            );
        let floorstyled_biome_sheet = sheets
            .sheet_floorstyled_from_biome(biome, self.classic_mode)
            .unwrap_or(
                sheets
                    .sheet_floorstyled_from_biome(&Biome::Olmec, self.classic_mode)
                    .unwrap(),
            );

        let mut placed_egg = false;

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

                        let tile_image = sheets.get_items(self.classic_mode).view(
                            15 * TILE_WIDTH,
                            1 * TILE_HEIGHT,
                            TILE_WIDTH,
                            TILE_HEIGHT,
                        );
                        if *biome == Biome::Volcana && !placed_egg && rng.gen_bool(0.3) {
                            let precious_image = &load_images(CHAR_PRECIOUS, CLASSIC_CHAR_PRECIOUS)
                                [if self.classic_mode { 1 } else { 0 }];

                            overlay(
                                base_image,
                                precious_image,
                                x - TILE_WIDTH + 8,
                                y - TILE_HEIGHT + 8,
                            );

                            overlay(
                                base_image,
                                &tile_image,
                                x - TILE_WIDTH / 2,
                                y - TILE_HEIGHT + 13,
                            );

                            placed_egg = true;
                        } else {
                            overlay(
                                base_image,
                                &tile_image,
                                x - TILE_WIDTH / 2,
                                y - TILE_HEIGHT + 18,
                            );
                        }
                    }
                    PlacedTile::EggplantAltarLeft => {
                        place_tile(biome_sheet, 10, 2);
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

                        // Avoid subtracting from index if it's 0
                        let down = match row_idx > 0 {
                            false => false,
                            true => grid[row_idx - 1][col_idx] == *tile,
                        };
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
                        if !placed_egg && rng.gen_bool(0.2) {
                            let beg_image = &load_images(CHAR_BEG, CLASSIC_CHAR_BEG)
                                [if self.classic_mode { 1 } else { 0 }];

                            overlay(base_image, beg_image, x, y);

                            placed_egg = true;
                        }

                        let tile_image = biome_sheet.view(
                            7 * TILE_WIDTH,
                            1 * TILE_HEIGHT,
                            TILE_WIDTH,
                            TILE_HEIGHT,
                        );
                        let tile_image = DynamicImage::ImageRgba8(tile_image.to_image());

                        let overlap = 8;
                        let mut tile_image = tile_image.resize(
                            TILE_WIDTH + overlap,
                            TILE_HEIGHT + overlap,
                            FilterType::CatmullRom,
                        );
                        tile_image
                            .as_mut_rgba8()
                            .unwrap()
                            .pixels_mut()
                            .for_each(|p| p[3] = p[2]);
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
                        place_tile(
                            &sheets
                                .sheet_floorstyled_from_biome(&Biome::Babylon, self.classic_mode)
                                .unwrap(),
                            7,
                            2,
                        );
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
                            Biome::Surface => sheets
                                .sheet_floor_from_biome(&Biome::Cave, self.classic_mode)
                                .unwrap(),
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
                        let tile_image = sheets.get_items(self.classic_mode).view(
                            14 * TILE_WIDTH,
                            14 * TILE_HEIGHT,
                            TILE_WIDTH,
                            TILE_HEIGHT,
                        );
                        overlay(base_image, &tile_image, x, y - 22);
                    }
                    PlacedTile::HoneyDown => {
                        let tile_image = sheets.get_items(self.classic_mode).view(
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

    pub fn render_floor_decorations(
        &self,
        base_image: &mut RgbaImage,
        sheets: &Sheets,
        biome: &Biome,
        config: &Spelunkicon,
        rng: &mut StdRng,
        grid: &PlacedTileGrid,
    ) {
        let sheet_image = sheets
            .sheet_floor_from_biome(biome, self.classic_mode)
            .unwrap();

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

        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, tile) in row.iter().enumerate() {
                let x = col_idx as u32 * TILE_HEIGHT as u32;
                let y = row_idx as u32 * TILE_WIDTH as u32;

                let pos = (col_idx, row_idx);
                let get_neighbour_empty = |dir| -> bool {
                    neighbour_empty(config, &grid, pos, dir, Some(PlacedTile::Floor))
                };

                match tile {
                    PlacedTile::Floor => {
                        let left = get_neighbour_empty(DIR_LEFT);
                        let right = get_neighbour_empty(DIR_RIGHT);
                        let up = get_neighbour_empty(DIR_UP);
                        let down = get_neighbour_empty(DIR_DOWN);

                        // Place generic deco
                        if left {
                            let x = x - (TILE_WIDTH / 2);
                            if up {
                                overlay(base_image, &left_up_deco, x, y);
                            } else {
                                overlay(base_image, left_deco.choose(rng).unwrap(), x, y);
                            }
                        }

                        if right {
                            let x = x + (TILE_WIDTH / 2);
                            if up {
                                overlay(base_image, &right_up_deco, x, y);
                            } else {
                                overlay(base_image, right_deco.choose(rng).unwrap(), x, y);
                            }
                        }

                        if down {
                            let y = y + (TILE_HEIGHT / 2);
                            overlay(base_image, down_deco.choose(rng).unwrap(), x, y);
                        }

                        // Place generic top-deco or spikes top-deco
                        if up {
                            let y_deco = y - (TILE_HEIGHT / 2);
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
                    PlacedTile::BoneBlock => {
                        let floor_cave = sheets
                            .sheet_floor_from_biome(&Biome::Cave, self.classic_mode)
                            .unwrap();
                        let left_deco = floor_cave.view(
                            10 * TILE_WIDTH,
                            3 * TILE_HEIGHT,
                            TILE_WIDTH,
                            TILE_HEIGHT,
                        );
                        let right_deco = floor_cave.view(
                            11 * TILE_WIDTH,
                            3 * TILE_HEIGHT,
                            TILE_WIDTH,
                            TILE_HEIGHT,
                        );

                        overlay(base_image, &left_deco, x - (TILE_WIDTH / 2) + 16, y);
                        overlay(base_image, &right_deco, x + (TILE_WIDTH / 2), y);

                        let up_empty = neighbour_empty(config, &grid, pos, DIR_UP, None);
                        let up_bone = !neighbour_empty(
                            config,
                            &grid,
                            pos,
                            DIR_UP,
                            Some(PlacedTile::BoneBlock),
                        );
                        if up_empty || up_bone {
                            let up_deco = floor_cave.view(
                                11 * TILE_WIDTH,
                                2 * TILE_HEIGHT,
                                TILE_WIDTH,
                                TILE_HEIGHT,
                            );
                            overlay(base_image, &up_deco, x, y - (TILE_HEIGHT / 2));
                        }
                        if up_empty && rng.gen_bool(0.5) {
                            let items = sheets.get_items(self.classic_mode);
                            let ribcage = items.view(
                                14 * TILE_WIDTH,
                                3 * TILE_HEIGHT,
                                TILE_WIDTH,
                                TILE_HEIGHT,
                            );
                            let skull = items.view(
                                15 * TILE_WIDTH,
                                3 * TILE_HEIGHT,
                                TILE_WIDTH,
                                TILE_HEIGHT,
                            );
                            overlay(base_image, &ribcage, x - 16, y - (TILE_HEIGHT * 3 / 4) + 6);
                            overlay(base_image, &skull, x + 16, y - (TILE_HEIGHT * 3 / 4) + 6);
                        }
                    }
                    PlacedTile::BushBlock => {
                        let floor_jungle = sheets
                            .sheet_floor_from_biome(&Biome::Jungle, self.classic_mode)
                            .unwrap();
                        let left_deco = floor_jungle.view(
                            10 * TILE_WIDTH,
                            3 * TILE_HEIGHT,
                            TILE_WIDTH,
                            TILE_HEIGHT,
                        );
                        let right_deco = floor_jungle.view(
                            11 * TILE_WIDTH,
                            3 * TILE_HEIGHT,
                            TILE_WIDTH,
                            TILE_HEIGHT,
                        );
                        let down_deco = floor_jungle.view(
                            10 * TILE_WIDTH,
                            4 * TILE_HEIGHT,
                            TILE_WIDTH,
                            TILE_HEIGHT,
                        );

                        overlay(base_image, &left_deco, x - (TILE_WIDTH / 2), y);
                        overlay(base_image, &right_deco, x + (TILE_WIDTH / 2), y);
                        overlay(base_image, &down_deco, x, y + (TILE_HEIGHT / 2));

                        let up = neighbour_empty(config, &grid, pos, DIR_UP, None)
                            || !neighbour_empty(
                                config,
                                &grid,
                                pos,
                                DIR_UP,
                                Some(PlacedTile::BushBlock),
                            );
                        if up {
                            let up_deco = floor_jungle.view(
                                11 * TILE_WIDTH,
                                2 * TILE_HEIGHT,
                                TILE_WIDTH,
                                TILE_HEIGHT,
                            );
                            overlay(base_image, &up_deco, x, y - (TILE_HEIGHT / 2));
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn render_floor_embeds(
        &self,
        base_image: &mut RgbaImage,
        sheets: &Sheets,
        config: &Spelunkicon,
        rng: &mut StdRng,
        grid: &PlacedTileGrid,
    ) {
        let items = sheets.get_items(self.classic_mode);
        let crust_gold = vec![
            items.view(TILE_WIDTH * 10, 0, TILE_WIDTH, TILE_HEIGHT),
            items.view(TILE_WIDTH * 11, 0, TILE_WIDTH, TILE_HEIGHT),
        ];
        let crust_jewels = vec![
            items.view(TILE_WIDTH * 3, 0, TILE_WIDTH, TILE_HEIGHT),
            items.view(TILE_WIDTH * 4, 0, TILE_WIDTH, TILE_HEIGHT),
            items.view(TILE_WIDTH * 5, 0, TILE_WIDTH, TILE_HEIGHT),
        ];
        let crust_jetpack = items.view(TILE_WIDTH * 9, TILE_HEIGHT * 2, TILE_WIDTH, TILE_HEIGHT);

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
                    } else if rng.gen::<u32>() % 5000 == 0 {
                        overlay(base_image, &crust_jetpack, x, y);
                    }
                }
            }
        }
    }
}
