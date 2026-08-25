use bevy::{
    platform::collections::{HashMap, HashSet},
    prelude::*,
};

use crate::world_grid::{cell::Cell, coord::Coord, grid_transform::GridTransform};

pub mod cell;
pub mod coord;
pub mod grid_transform;

pub const CELL_SIZE: f32 = 16.0;

pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_grid);
        app.add_systems(Update, update_transforms);
    }
}

#[derive(Resource)]
pub struct WorldGrid {
    bottom_left: Coord,
    grid_size: UVec2,
    cells: HashMap<Coord, Cell>,
}

impl WorldGrid {
    pub fn new(grid_size: UVec2) -> Self {
        Self {
            bottom_left: Coord(IVec2::ZERO),
            grid_size,
            cells: HashMap::new(),
        }
    }

    pub fn _set_offset(&mut self, bottom_left: Coord) {
        self.bottom_left = bottom_left;
    }

    pub fn grid_size(&self) -> UVec2 {
        self.grid_size
    }

    /// Resizes the grid and returns a hash set of all the entities that moved out of the grid diring the resize
    pub fn resize(&mut self, new_size: UVec2) -> HashSet<Entity> {
        let mut new_cells = HashMap::new();

        for x in 0..new_size.x as i32 {
            for y in 0..new_size.y as i32 {
                let x = x + self.bottom_left.0.x;
                let y = y + self.bottom_left.0.y;

                let coord = Coord(IVec2 { x, y });
                if let Some(cell) = self.get_cell(coord) {
                    new_cells.insert(coord, cell);
                }
            }
        }

        let mut moved_out_of_grid = HashSet::new();
        for (coord, cell) in &self.cells {
            if new_cells.get_key_value(coord).is_none() {
                match cell {
                    Cell::Empty => (),
                    Cell::Occupied { entity } => {
                        moved_out_of_grid.insert(*entity);
                    }
                }
            }
        }

        self.cells = new_cells;
        self.grid_size = new_size;

        moved_out_of_grid
    }

    pub fn center(&self) -> Vec2 {
        let x = (self.grid_size.x as f32 * CELL_SIZE) / 2.0 + self.bottom_left.to_world().x;
        let y = (self.grid_size.y as f32 * CELL_SIZE) / 2.0 + self.bottom_left.to_world().y;

        Vec2 { x, y }
    }

    pub fn set_cell(&mut self, coord: Coord, cell: Cell) {
        self.cells.insert(coord, cell);
    }

    pub fn set_all_cells(&mut self, coords: &[Coord], cell: Cell) {
        for coord in coords {
            self.set_cell(*coord, cell);
        }
    }

    pub fn get_cell(&self, coord: Coord) -> Option<Cell> {
        self.cells.get(&coord).copied()
    }

    pub fn cell_empty(&self, coord: Coord) -> bool {
        if let Some(cell) = self.cells.get(&coord) {
            return *cell == Cell::Empty;
        }

        true
    }

    pub fn all_cells_empty(&self, coords: &[Coord]) -> bool {
        for coord in coords {
            if !self.cell_empty(*coord) {
                return false;
            }
        }

        true
    }

    pub fn coord_in_grid(&self, coord: Coord) -> bool {
        let coord = Coord(self.bottom_left.0 + coord.0);

        coord.0.x >= 0
            && coord.0.y >= 0
            && coord.0.x < self.grid_size.x as i32
            && coord.0.y < self.grid_size.y as i32
    }

    pub fn transform_in_grid(&self, transform: GridTransform) -> bool {
        transform
            .get_coords()
            .iter()
            .all(|coord| self.coord_in_grid(*coord))
    }

    fn _all_coords(&self) -> Vec<Coord> {
        let mut coords = Vec::new();

        for x in 0..self.grid_size.x as i32 {
            for y in 0..self.grid_size.y as i32 {
                let x = x + self.bottom_left.0.x;
                let y = y + self.bottom_left.0.y;

                let coord = Coord(IVec2 { x, y });
                coords.push(coord);
            }
        }

        coords
    }
}

fn draw_grid(mut gizmos: Gizmos, world_grid: If<Res<WorldGrid>>) {
    gizmos
        .grid_2d(
            Isometry2d::new(world_grid.center(), Rot2::default()),
            world_grid.grid_size,
            Vec2::splat(CELL_SIZE),
            Color::srgba(1.0, 1.0, 1.0, 0.1),
        )
        .outer_edges();
}

fn update_transforms(
    mut transforms: Query<(&mut Transform, &GridTransform), Changed<GridTransform>>,
) {
    for (mut transform, grid_transform) in &mut transforms {
        let world = grid_transform.center_in_world();
        transform.translation = Vec3 {
            x: world.x,
            y: world.y,
            z: 0.0,
        };
    }
}
