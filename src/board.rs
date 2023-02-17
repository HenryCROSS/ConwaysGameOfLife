#[derive(Clone)]
pub struct Pos(pub usize, pub usize);

#[derive(Clone)]
pub enum State {
    ALIVE,
    DIE,
}

#[derive(Clone)]
pub enum Colour {
    BLACK,
}

#[derive(Clone)]
pub struct Block {
    state: State,
    colour: Colour,
    neighbors: usize,
}

impl Block {
    pub fn new() -> Block {
        Block {
            state: State::DIE,
            colour: Colour::BLACK,
            neighbors: 0,
        }
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }

    pub fn set_colour(&mut self, colour: Colour) {
        self.colour = colour;
    }

    pub fn get_colour(&self) -> &Colour {
        &self.colour
    }

    pub fn get_neighbors(&self) -> usize {
        self.neighbors
    }

    pub fn set_neighbors(&mut self, num: usize) {
        self.neighbors = num;
    }
}

pub struct Blocks {
    blocks: Vec<Vec<Block>>,
    weight: usize,
    height: usize,
}

impl Blocks {
    pub fn new(weight: usize, height: usize) -> Blocks {
        Blocks {
            blocks: vec![vec![Block::new(); weight]; height],
            weight,
            height,
        }
    }

    pub fn at(&self, x: usize, y: usize) -> Option<&Block> {
        if x > self.weight || y > self.height {
            return None;
        }

        Some(&self.blocks[y][y])
    }

    pub fn at_mut(&mut self, x: usize, y: usize) -> Option<&mut Block> {
        if x > self.weight || y > self.height {
            return None;
        }

        Some(&mut self.blocks[y][y])
    }
}

pub struct Grid {
    blocks: Blocks,
    alive_blocks_pos: Vec<Pos>,
}

impl Grid {
    pub fn new(weight: usize, height: usize) -> Grid {
        Grid {
            blocks: Blocks::new(weight, height),
            alive_blocks_pos: Vec::new(),
        }
    }

    pub fn get_block_ref_at(&self, x: usize, y: usize) -> Option<&Block> {
        self.blocks.at(x, y)
    }

    pub fn set_block_at(&mut self, x: usize, y: usize, block: Block) -> Result<(), &'static str> {
        if x >= self.get_weight() || y >= self.get_height() {
            return Err("x or y is out of index");
        }

        if let State::ALIVE = block.state {
            self.alive_blocks_pos.push(Pos(x, y));
        }

        self.alive_blocks_pos.push(Pos(x, y));

        self.blocks.at_mut(x, y).map(|b| {
            b.set_state(block.state);
            b.set_colour(block.colour);
        });
        Ok(())
    }

    pub fn get_grid(&self) -> &Blocks {
        &self.blocks
    }

    pub fn set_grid(&mut self, blocks: Box<Blocks>) {
        self.blocks = *blocks;
    }

    pub fn get_weight(&self) -> usize {
        self.blocks.weight
    }

    pub fn get_height(&self) -> usize {
        self.blocks.height
    }

    pub fn get_alive_pos(&self) -> &Vec<Pos> {
        &self.alive_blocks_pos
    }

    pub fn set_alive_pos(&mut self, pos_list: Box<Vec<Pos>>) -> Result<(), &'static str> {
        for pos in &*pos_list {
            if pos.0 >= self.get_weight() || pos.1 >= self.get_height() {
                return Err("It is out of edge");
            }
        }

        self.alive_blocks_pos = *pos_list;
        Ok(())
    }

    pub fn add_alive_pos(&mut self, pos: Pos) -> Result<(), &'static str> {
        if pos.0 >= self.get_weight() || pos.1 >= self.get_height() {
            return Err("It is out of edge");
        }

        self.alive_blocks_pos.push(pos);
        Ok(())
    }
}
