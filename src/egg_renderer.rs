use image::imageops::overlay;
use image::{GenericImageView, RgbaImage};
use rand::prelude::*;

use crate::constants::{TILE_HEIGHT, TILE_WIDTH};
use crate::grid_renderer::Sheets;
use crate::sheets::Biome;
use crate::spelunkicon::Spelunkicon;

pub struct PrideRenderer {}

impl PrideRenderer {
    fn render_nonbinary_flag(
        &self,
        base_image: &mut RgbaImage,
        sheets: &Sheets,
        config: &Spelunkicon,
        _rng: &mut StdRng,
    ) {
        let w = config.grid_width as u32;

        // Tiles
        {
            let gold_tile = sheets
                .sheet_floorstyled_from_biome(&Biome::CityOfGold)
                .unwrap()
                .view(1 * TILE_WIDTH, 4 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT);
            let y = 0 * TILE_HEIGHT;
            for i in 0..w {
                let x = i * TILE_WIDTH;
                overlay(base_image, &gold_tile, x, y);
            }
        }

        {
            let palace_tile = sheets
                .sheet_floorstyled_from_biome(&Biome::PalaceOfPleasure)
                .unwrap()
                .view(1 * TILE_WIDTH, 5 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT);
            let y = 1 * TILE_HEIGHT;
            for i in 0..w {
                let x = i * TILE_WIDTH;
                overlay(base_image, &palace_tile, x, y);
            }
        }

        {
            let babylon_tile = sheets
                .sheet_floorstyled_from_biome(&Biome::Babylon)
                .unwrap()
                .view(1 * TILE_WIDTH, 5 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT);
            let y = 2 * TILE_HEIGHT;
            for i in 0..w {
                let x = i * TILE_WIDTH;
                overlay(base_image, &babylon_tile, x, y);
            }
        }

        {
            let duat_tile = sheets
                .sheet_floorstyled_from_biome(&Biome::Duat)
                .unwrap()
                .view(1 * TILE_WIDTH, 2 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT);
            let y = 3 * TILE_HEIGHT;
            for i in 0..w {
                let x = i * TILE_WIDTH;
                overlay(base_image, &duat_tile, x, y);
            }
        }

        // No Decos
    }

    fn render_pansexual_flag(
        &self,
        base_image: &mut RgbaImage,
        sheets: &Sheets,
        config: &Spelunkicon,
        rng: &mut StdRng,
    ) {
        let w = config.grid_width as u32;

        // Tiles
        {
            let guts_sheet = sheets.sheet_floorstyled_from_biome(&Biome::Guts).unwrap();
            let guts_tile_top =
                guts_sheet.view(1 * TILE_WIDTH, 3 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT);
            let guts_tile_bot =
                guts_sheet.view(1 * TILE_WIDTH, 4 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT);
            let y = 0 * TILE_HEIGHT;
            for i in 0..w {
                let x = i * TILE_WIDTH;
                overlay(base_image, &guts_tile_top, x, y);
                overlay(base_image, &guts_tile_bot, x, y + TILE_HEIGHT);
            }
        }

        {
            let gold_sheet = sheets
                .sheet_floorstyled_from_biome(&Biome::CityOfGold)
                .unwrap();
            let gold_tile_top =
                gold_sheet.view(1 * TILE_WIDTH, 2 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT);
            let gold_tile_bot =
                gold_sheet.view(1 * TILE_WIDTH, 4 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT);
            let y = 2 * TILE_HEIGHT;
            for i in 0..w {
                let x = i * TILE_WIDTH;
                overlay(base_image, &gold_tile_top, x, y);
                overlay(base_image, &gold_tile_bot, x, y + TILE_HEIGHT);
            }
        }

        {
            let tidepool_tile = sheets
                .sheet_floor_from_biome(&Biome::TidePool)
                .unwrap()
                .view(0 * TILE_WIDTH, 0 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT);
            let y = 4 * TILE_HEIGHT;
            for i in 0..w {
                let x = i * TILE_WIDTH;
                overlay(base_image, &tidepool_tile, x, y);
                overlay(base_image, &tidepool_tile, x, y + TILE_HEIGHT);
            }
        }

        // Deco
        {
            let tide_pool = &sheets.sheet_floor_from_biome(&Biome::TidePool).unwrap();
            let up_deco = vec![
                tide_pool.view(5 * TILE_WIDTH, 6 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
                tide_pool.view(6 * TILE_WIDTH, 6 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
                tide_pool.view(7 * TILE_WIDTH, 6 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
            ];
            let y = 4 * TILE_HEIGHT;
            for i in 0..w {
                let x = i * TILE_WIDTH;
                overlay(
                    base_image,
                    up_deco.choose(rng).unwrap(),
                    x,
                    y - TILE_HEIGHT / 2,
                );
            }
        }
    }

    fn render_blm_pride_flag(
        &self,
        base_image: &mut RgbaImage,
        sheets: &Sheets,
        config: &Spelunkicon,
        rng: &mut StdRng,
    ) {
        let w = config.grid_width as u32;

        // Tiles
        {
            let duat_tile = sheets
                .sheet_floorstyled_from_biome(&Biome::Duat)
                .unwrap()
                .view(1 * TILE_WIDTH, 4 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT);
            let y = 0 * TILE_HEIGHT;
            for i in 0..w {
                let x = i * TILE_WIDTH;
                overlay(base_image, &duat_tile, x, y);
            }
        }

        {
            let wood_tile = sheets
                .sheet_floorstyled_from_biome(&Biome::Cave)
                .unwrap()
                .view(1 * TILE_WIDTH, 5 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT);
            let y = 1 * TILE_HEIGHT;
            for i in 0..w {
                let x = i * TILE_WIDTH;
                overlay(base_image, &wood_tile, x, y);
            }
        }

        {
            let vlad_tile = sheets
                .sheet_floorstyled_from_biome(&Biome::Vlad)
                .unwrap()
                .view(1 * TILE_WIDTH, 5 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT);
            let y = 2 * TILE_HEIGHT;
            for i in 0..w {
                let x = i * TILE_WIDTH;
                overlay(base_image, &vlad_tile, x, y);
            }
        }

        {
            let cave_tile = sheets.sheet_floor_from_biome(&Biome::Cave).unwrap().view(
                0 * TILE_WIDTH,
                0 * TILE_HEIGHT,
                TILE_WIDTH,
                TILE_HEIGHT,
            );
            let y = 3 * TILE_HEIGHT;
            for i in 0..w {
                let x = i * TILE_WIDTH;
                overlay(base_image, &cave_tile, x, y);
            }
        }

        {
            let gold_tile = sheets
                .sheet_floorstyled_from_biome(&Biome::CityOfGold)
                .unwrap()
                .view(1 * TILE_WIDTH, 5 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT);
            let y = 4 * TILE_HEIGHT;
            for i in 0..w {
                let x = i * TILE_WIDTH;
                overlay(base_image, &gold_tile, x, y);
            }
        }

        {
            let jungle_tile = sheets.sheet_floor_from_biome(&Biome::Jungle).unwrap().view(
                0 * TILE_WIDTH,
                0 * TILE_HEIGHT,
                TILE_WIDTH,
                TILE_HEIGHT,
            );
            let y = 5 * TILE_HEIGHT;
            for i in 0..w {
                let x = i * TILE_WIDTH;
                overlay(base_image, &jungle_tile, x, y);
            }
        }

        {
            let pagoda_tile = sheets
                .sheet_floorstyled_from_biome(&Biome::TidePool)
                .unwrap()
                .view(1 * TILE_WIDTH, 5 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT);
            let y = 6 * TILE_HEIGHT;
            for i in 0..w {
                let x = i * TILE_WIDTH;
                overlay(base_image, &pagoda_tile, x, y);
            }
        }

        {
            let eggplant_tile = sheets
                .sheet_floor_from_biome(&Biome::Eggplant)
                .unwrap()
                .view(0 * TILE_WIDTH, 0 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT);
            let y = 7 * TILE_HEIGHT;
            for i in 0..w {
                let x = i * TILE_WIDTH;
                overlay(base_image, &eggplant_tile, x, y);
            }
        }

        // Deco
        {
            let cave = &sheets.sheet_floor_from_biome(&Biome::Cave).unwrap();
            let up_deco = vec![
                cave.view(5 * TILE_WIDTH, 6 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
                cave.view(6 * TILE_WIDTH, 6 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
                cave.view(7 * TILE_WIDTH, 6 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
            ];
            let down_deco = vec![
                cave.view(5 * TILE_WIDTH, 7 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
                cave.view(6 * TILE_WIDTH, 7 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
                cave.view(7 * TILE_WIDTH, 7 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
            ];
            let y = 3 * TILE_HEIGHT;
            for i in 0..w {
                let x = i * TILE_WIDTH;
                overlay(
                    base_image,
                    up_deco.choose(rng).unwrap(),
                    x,
                    y - TILE_HEIGHT / 2,
                );
                overlay(
                    base_image,
                    down_deco.choose(rng).unwrap(),
                    x,
                    y + TILE_HEIGHT / 2,
                );
            }
        }

        {
            let jungle = &sheets.sheet_floor_from_biome(&Biome::Jungle).unwrap();
            let up_deco = vec![
                jungle.view(5 * TILE_WIDTH, 6 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
                jungle.view(6 * TILE_WIDTH, 6 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
                jungle.view(7 * TILE_WIDTH, 6 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
            ];
            let down_deco = vec![
                jungle.view(5 * TILE_WIDTH, 7 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
                jungle.view(6 * TILE_WIDTH, 7 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
                jungle.view(7 * TILE_WIDTH, 7 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
            ];
            let y = 5 * TILE_HEIGHT;
            for i in 0..w {
                let x = i * TILE_WIDTH;
                overlay(
                    base_image,
                    up_deco.choose(rng).unwrap(),
                    x,
                    y - TILE_HEIGHT / 2,
                );
                overlay(
                    base_image,
                    down_deco.choose(rng).unwrap(),
                    x,
                    y + TILE_HEIGHT / 2,
                );
            }
        }

        {
            let eggplant = &sheets.sheet_floor_from_biome(&Biome::Eggplant).unwrap();
            let up_deco = vec![
                eggplant.view(5 * TILE_WIDTH, 6 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
                eggplant.view(6 * TILE_WIDTH, 6 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
                eggplant.view(7 * TILE_WIDTH, 6 * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT),
            ];
            let y = 7 * TILE_HEIGHT;
            for i in 0..w {
                let x = i * TILE_WIDTH;
                overlay(
                    base_image,
                    up_deco.choose(rng).unwrap(),
                    x,
                    y - TILE_HEIGHT / 2,
                );
            }
        }
    }

    fn render_4_pride(
        &self,
        base_image: &mut RgbaImage,
        sheets: &Sheets,
        config: &Spelunkicon,
        rng: &mut StdRng,
    ) {
        self.render_nonbinary_flag(base_image, sheets, config, rng);
    }

    fn render_6_pride(
        &self,
        base_image: &mut RgbaImage,
        sheets: &Sheets,
        config: &Spelunkicon,
        rng: &mut StdRng,
    ) {
        self.render_pansexual_flag(base_image, sheets, config, rng);
    }

    fn render_8_pride(
        &self,
        base_image: &mut RgbaImage,
        sheets: &Sheets,
        config: &Spelunkicon,
        rng: &mut StdRng,
    ) {
        self.render_blm_pride_flag(base_image, sheets, config, rng);
    }

    pub fn render(
        &self,
        base_image: &mut RgbaImage,
        sheets: &Sheets,
        config: &Spelunkicon,
        rng: &mut StdRng,
    ) {
        match config.grid_height {
            4 => self.render_4_pride(base_image, sheets, config, rng),
            6 => self.render_6_pride(base_image, sheets, config, rng),
            8 => self.render_8_pride(base_image, sheets, config, rng),
            _ => {}
        }
    }
}
