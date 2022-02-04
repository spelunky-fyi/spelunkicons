use rand::prelude::*;
use std::collections::HashMap;

use crate::sheets::Biome;
use crate::spelunkicon::Spelunkicon;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PlacedTile {
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
pub type PlacedTileGrid = Vec<Vec<PlacedTile>>;

pub static DIR_NONE: (i64, i64) = (0, 0);
pub static DIR_LEFT: (i64, i64) = (-1, 0);
pub static DIR_UP_LEFT: (i64, i64) = (-1, -1);
pub static DIR_UP: (i64, i64) = (0, -1);
pub static DIR_UP_RIGHT: (i64, i64) = (1, -1);
pub static DIR_RIGHT: (i64, i64) = (1, 0);
pub static DIR_DOWN_RIGHT: (i64, i64) = (1, 1);
pub static DIR_DOWN: (i64, i64) = (0, 1);
pub static DIR_DOWN_LEFT: (i64, i64) = (-1, 1);
pub fn neighbour_empty(
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

pub struct GridGenerator {}

impl GridGenerator {
    pub fn place_floor_tiles(
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

    pub fn place_floorstyled_tiles(
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

    fn place_floormisc_cave(
        &self,
        _config: &Spelunkicon,
        _rng: &mut StdRng,
        grid: &mut PlacedTileGrid,
        _biome: &Biome,
        pos: (usize, usize),
        directions: &HashMap<(i64, i64), bool>,
    ) {
        let (x, y) = pos;

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

                grid[y][x] = misc_tile;
                if y > 0 {
                    grid[y - 1][x] = misc_tile;
                }
            }
        } else {
            let left = directions[&DIR_LEFT];
            let right = directions[&DIR_RIGHT];
            if left != right {
                grid[y][x] = PlacedTile::ArrowTrap;
            }
        }
    }

    fn place_floormisc_jungle(
        &self,
        config: &Spelunkicon,
        rng: &mut StdRng,
        grid: &mut PlacedTileGrid,
        biome: &Biome,
        pos: (usize, usize),
        directions: &HashMap<(i64, i64), bool>,
    ) {
        let (x, y) = pos;

        let this = neighbour_empty(config, &grid, pos, DIR_NONE, Some(PlacedTile::Floor));
        if !this {
            let left = directions[&DIR_LEFT];
            let right = directions[&DIR_RIGHT];
            let up = directions[&DIR_UP];
            let down = directions[&DIR_DOWN];
            if left || right || up || down {
                grid[y][x] = if rng.gen_bool(0.5) {
                    PlacedTile::SpearTrap
                } else {
                    PlacedTile::BushBlock
                };
            }
        } else {
            let this = neighbour_empty(config, &grid, pos, DIR_NONE, None);
            if this {
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
                            grid[y][x] = PlacedTile::HoneyUp;
                        } else {
                            let down_bee = neighbour_empty(
                                config,
                                &grid,
                                pos,
                                DIR_DOWN,
                                Some(PlacedTile::FloorStyled),
                            );
                            if !down_bee {
                                grid[y][x] = PlacedTile::HoneyDown;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn place_floormisc_volcana(
        &self,
        config: &Spelunkicon,
        rng: &mut StdRng,
        grid: &mut PlacedTileGrid,
        _biome: &Biome,
        pos: (usize, usize),
        directions: &HashMap<(i64, i64), bool>,
    ) {
        let (x, y) = pos;

        let this = directions[&DIR_NONE];
        if this {
            let down = directions[&DIR_DOWN];
            let up = directions[&DIR_UP];
            if !down && rng.gen_bool(0.01) {
                grid[y - 1][x] = PlacedTile::UdjatSocketTop;
                grid[y][x] = PlacedTile::UdjatSocketBot;
            } else if down && !up {
                grid[y - 1][x] = PlacedTile::ChainTop;
                for i in 0..4 {
                    if y + i as usize == config.grid_height as usize - 1
                        || i == 3
                        || grid[y + i][x] != PlacedTile::None
                    {
                        grid[y + i][x] = PlacedTile::ChainBot;
                        break;
                    } else {
                        grid[y + i][x] = PlacedTile::ChainMid;
                    }
                }
            }
        } else {
            let up = directions[&DIR_UP];
            let down = neighbour_empty(config, &grid, pos, DIR_DOWN, Some(PlacedTile::Floor));
            if up && !down {
                let flip = rng.gen_bool(0.5);
                let tile = if flip {
                    PlacedTile::ConveyorLeft
                } else {
                    PlacedTile::ConveyorRight
                };

                grid[y][x] = tile;

                let left = directions[&DIR_LEFT];
                let up_left = directions[&DIR_UP_LEFT];
                let down_left =
                    neighbour_empty(config, &grid, pos, DIR_DOWN_LEFT, Some(PlacedTile::Floor));
                if left && up_left && !down_left {
                    grid[y][x - 1] = tile;
                }

                let right = directions[&DIR_RIGHT];
                let up_right = directions[&DIR_UP_RIGHT];
                let down_right =
                    neighbour_empty(config, &grid, pos, DIR_DOWN_RIGHT, Some(PlacedTile::Floor));
                if right && up_right && !down_right {
                    grid[y][x + 1] = tile;
                }
            }
        }
    }

    fn place_floormisc_tidepool(
        &self,
        _config: &Spelunkicon,
        _rng: &mut StdRng,
        grid: &mut PlacedTileGrid,
        _biome: &Biome,
        pos: (usize, usize),
        directions: &HashMap<(i64, i64), bool>,
    ) {
        let (x, y) = pos;

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
                    grid[y][x] = PlacedTile::LionTrap;
                    if y > 0 {
                        grid[y - 1][x] = PlacedTile::LionTrap;
                    }
                };
            }
        }
    }

    fn place_floormisc_temple(
        &self,
        config: &Spelunkicon,
        rng: &mut StdRng,
        grid: &mut PlacedTileGrid,
        _biome: &Biome,
        pos: (usize, usize),
        directions: &HashMap<(i64, i64), bool>,
    ) {
        let (x, y) = pos;

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
                grid[y][x] = PlacedTile::LargeCrushTrapTopLeft;
                grid[y][x + 1] = PlacedTile::LargeCrushTrapTopRight;
                grid[y + 1][x] = PlacedTile::LargeCrushTrapBotLeft;
                grid[y + 1][x + 1] = PlacedTile::LargeCrushTrapBotRight;
            } else {
                let left = directions[&DIR_LEFT];
                let up = directions[&DIR_UP];
                if left || right || up || down {
                    grid[y][x] = PlacedTile::CrushTrap;
                }
            }
        }
    }

    pub fn place_floormisc_tiles(
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
                Biome::Cave => self.place_floormisc_cave(
                    config,
                    rng,
                    &mut grid,
                    biome,
                    (col_idx, row_idx),
                    &directions,
                ),
                Biome::Jungle | Biome::Beehive => self.place_floormisc_jungle(
                    config,
                    rng,
                    &mut grid,
                    biome,
                    (col_idx, row_idx),
                    &directions,
                ),
                Biome::Volcana => self.place_floormisc_volcana(
                    config,
                    rng,
                    &mut grid,
                    biome,
                    (col_idx, row_idx),
                    &directions,
                ),
                Biome::TidePool => self.place_floormisc_tidepool(
                    config,
                    rng,
                    &mut grid,
                    biome,
                    (col_idx, row_idx),
                    &directions,
                ),
                Biome::Temple | Biome::CityOfGold => self.place_floormisc_temple(
                    config,
                    rng,
                    &mut grid,
                    biome,
                    (col_idx, row_idx),
                    &directions,
                ),
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
}
