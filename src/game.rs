use crate::{board::{Grid, Pos, Blocks}, game_process::{MapProcesser, MapProcessOps}};

pub enum Backend_State {
    RUNNING,
    STOP,
}

pub trait I_Backend {
    fn backend_set_map_block_alive(&mut self, x: usize, y: usize) -> Result<(), &'static str>;
    fn backend_init_map(&mut self, weight: usize, height: usize);
    fn backend_get_map(&self) -> &Blocks;
    fn backend_set_state(&mut self, state: Backend_State);
    fn backend_get_state(&self) -> &Backend_State;

    fn get_game_option(&self);
    fn set_game_option(&mut self, options: Game_Options);
}


pub struct Game_Options;
pub struct Game_Controller {
    backend: Backend,
    options: Game_Options,
}

impl I_Backend for Game_Controller {
    fn backend_set_state(&mut self, state: Backend_State) {
        self.backend.set_state(state);
    }

    fn backend_get_state(&self) -> &Backend_State {
        self.backend.get_state()
    }

    fn backend_set_map_block_alive(&mut self, x: usize, y: usize) -> Result<(), &'static str> {
        self.backend.add_map_block_alive(x, y)
    }

    fn backend_get_map(&self) -> &Blocks {
        self.backend.get_map()
    }

    fn backend_init_map(&mut self, weight: usize, height: usize) {
        self.backend.generate_board(weight, height);
    }

    fn get_game_option(&self) {
        unimplemented!()
    }

    fn set_game_option(&mut self, options: Game_Options) {
        unimplemented!()
    }
}

pub struct Backend {
    state: Backend_State,
    game_board: Grid
}

impl Backend {
    pub fn set_state(&mut self, state: Backend_State){
        self.state = state;
    }

    pub fn get_state(&self) -> &Backend_State {
        &self.state
    }

    fn add_map_block_alive(&mut self, x: usize, y: usize) -> Result<(), &'static str> {
        self.game_board.add_alive_pos(Pos(x, y))
    }

    pub fn get_map(&self) -> &Blocks {
        self.game_board.get_grid()
    }

    pub fn generate_board(&mut self, weight: usize, height: usize) {
        self.game_board = Grid::new(weight, height);
    }

    pub fn process(&mut self) {
        if let Backend_State::RUNNING = self.state {
            MapProcesser::process_map(&mut self.game_board);
        }
    }
}

