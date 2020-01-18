use piston_window::*;
use crate::constants::BACK_COLOR;

mod constants;
mod position;
mod draw;
mod life_cycle;
mod size;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Tetris", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            //todo key press handle
        }

        window.draw_2d(&event, |context, g, _| {
            clear(BACK_COLOR, g);
            //todo
        });

        event.update(|arg| {
            //todo
        });
    }
}
