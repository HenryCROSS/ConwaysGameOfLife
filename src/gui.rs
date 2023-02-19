use iced::{Point, Color, Vector};
use iced::widget::canvas::event::{self, Event};
use iced::widget::canvas::{Cache, Canvas, Cursor, Frame, Geometry, Path, Text};
use iced::{mouse, widget::canvas, Theme};

pub enum Interaction {
    None,
    Drawing,
    Erasing,
}

impl Default for Interaction {
    fn default() -> Self {
        Self::None
    }
}

pub enum Message {}

pub struct GUI_Grid {
    scaling: f32,
    life_cache: Cache
}

impl canvas::Program<Message> for GUI_Grid {
    type State = Interaction;
    fn draw(
        &self,
        state: &Self::State,
        theme: &Theme,
        bounds: iced::Rectangle,
        cursor: canvas::Cursor,
    ) -> Vec<canvas::Geometry> {
        let center = Vector::new(bounds.width / 2.0, bounds.height / 2.0);
        let lift = self.life_cache.draw(bounds.size(), |frame| {
            let background = Path::rectangle(Point::ORIGIN, frame.size());
            frame.fill(&background, Color::WHITE);

            frame.with_save(|frame| {
                frame.translate(center);
                frame.scale(self.scaling);
            })
        });
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: Event,
        bounds: iced::Rectangle,
        cursor: canvas::Cursor,
    ) -> (event::Status, Option<Message>) {
        (event::Status::Ignored, None)
        // match event {
        //     canvas::Event::Touch() => {
        //     },
        //     canvas::Event::Mouse() => _,
        //     canvas::Event::Keyboard() => _,
        // }
    }

    fn mouse_interaction(
        &self,
        state: &Self::State,
        bounds: iced::Rectangle,
        cursor: canvas::Cursor,
    ) -> mouse::Interaction {
        match state {
            Interaction::Drawing => mouse::Interaction::Crosshair,
            Interaction::Erasing => mouse::Interaction::Crosshair,
            Interaction::None if cursor.is_over(&bounds) => mouse::Interaction::Crosshair,
            _ => mouse::Interaction::default(),
        }
    }
}
