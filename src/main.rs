mod board;
mod game;
mod game_process;
mod gui;

use std::fmt::format;

use iced::widget::Button;
use iced::widget::Column;
use iced::widget::Container;
use iced::widget::Text;
use iced::{Element, Sandbox, Settings};

pub fn main() -> Result<(), iced::Error> {
    Game_UI::run(Settings::default())
}

enum Game_Stage {
    SETTINGS,
    GAMMING,
    MENU,
    EXIT,
}
struct Game_UI {
    stage: Game_Stage,
    debug: bool,
    value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum MenuMessage {
    Exit,
    Play,
    Setting,
    Menu,
}

#[derive(Debug, Clone, Copy)]
pub enum CounterMessage {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Game_UI {
    type Message = MenuMessage;

    fn new() -> Self {
        Game_UI {
            stage: Game_Stage::MENU,
            value: 0,
            debug: false,
        }
    }

    fn title(&self) -> String {
        String::from("My Conway's Game of Life")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            MenuMessage::Exit => self.stage = Game_Stage::EXIT,
            MenuMessage::Play => self.stage = Game_Stage::GAMMING,
            MenuMessage::Setting => self.stage = Game_Stage::SETTINGS,
            MenuMessage::Menu => self.stage = Game_Stage::MENU,
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let label = Text::new(format!("Count: {}", self.value));
        let incr = Button::new("Play").on_press(MenuMessage::Play);
        let decr = Button::new("Settings").on_press(MenuMessage::Setting);

        let col = Column::new().push(incr).push(label).push(decr);
        Container::new(col)
            .center_x()
            .center_y()
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}
