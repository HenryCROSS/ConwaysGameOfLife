use iced::Theme;
use iced::alignment;
use iced::executor;
use iced::theme;
use iced::widget::{
    checkbox, column, container, horizontal_space, image, radio, row, scrollable, slider, text,
    text_input, toggler, vertical_space,
};
use iced::widget::{Button, Column, Container, Slider};
use iced::Application;
use iced::Command;
use iced::{Color, Element, Length, Renderer, Sandbox, Settings};

pub fn main() -> iced::Result {
    Game::run(Settings::default())
}

// pub struct Game {
//     select: i32,
//     debug: bool,
// }
struct Game;

impl Application for Game {
    type Executor = executor::Default;
    type Flags = ();
    type Message = ();
    type Theme = Theme;
    fn new(flags: Self::Flags) -> (Game, iced::Command<Self::Message>) {
        (Game, Command::none())
    }

    fn title(&self) -> String {
        String::from("My Conway's Game of Life")
    }


    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        "Hello, world".into()
    }
}
