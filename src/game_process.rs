use crate::board::{Grid, Block, State, Pos, Blocks};

pub trait MapProcessOps {
    fn process_map(grid: &mut Grid);
}

pub struct MapProcesser;

impl MapProcessOps for MapProcesser {
    fn process_map(grid: &mut Grid) {
        let weight = grid.get_weight();
        let height = grid.get_height();
        let mut new_map = Box::new(Blocks::new(weight, height));
        let mut new_alive_list = grid.get_alive_pos().to_vec();

        // check neighbors and state
        let _ = grid.get_alive_pos().iter().map(|Pos(x, y)| {
            let x = *x;
            let y = *y;

            if let Some(block) = new_map.at_mut(x, y) {
                block.set_state(State::ALIVE);
            }

            for i in y-1..y+1 {
                for j in x-1..x+1 {
                    if i == x && j == y {
                        continue;
                    }

                    if let Some(block) = new_map.at_mut(i, j) {
                        block.set_neighbors(block.get_neighbors() + 1);
                    }
                }
            }
        });

        // generate new map
        let _ = grid.get_alive_pos().iter().map(|Pos(x, y)| {
            let x = *x;
            let y = *y;

            for i in y-1..y+1 {
                for j in x-1..x+1 {
                    let block: &mut Block;
                    if let Some(b) = new_map.at_mut(i, j) {
                        block = b;
                    } else {
                        continue;
                    }

                    let neighbors = block.get_neighbors();
                    if let State::ALIVE = block.get_state() {
                        if neighbors > 3 || neighbors < 2 {
                            block.set_state(State::DIE);
                            new_alive_list.retain(|Pos(tx, ty)|{
                                *tx != x || *ty != y
                            });
                        }
                    } else {
                        if neighbors >= 3 {
                            block.set_state(State::ALIVE);
                            new_alive_list.push(Pos(x, y));
                        }
                    }
                }
            }
        });

        grid.set_alive_pos(Box::new(new_alive_list));
        grid.set_grid(new_map);
    }
}
