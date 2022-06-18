extern crate piston_window;
extern crate rand;

use piston_window::*;
use piston_window::types::Color;

const BACK_COLOR: Color = [0.204, 0.286, 0.369, 1.0];
fn main() {
    let (width, height) = (400, 400);

    // Prepare window settings
    let mut window_settings = WindowSettings::new(
        "Rust Snake",
        [ width as f64, height as f64]
    ).exit_on_esc(true);

    // Fix vsync extension error for linux
    window_settings.set_vsync(true); 

    // Create a window
    let mut window: PistonWindow = window_settings.build().unwrap();

    // Event loop
    while let Some(event) = window.next() {

        // Catch the events of the keyboard
        if let Some(Button::Keyboard(key)) = event.press_args() {
            // Key press event handler
        }

        // Draw all of them
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            // Draw event handler
        });

        // Update the state of the game
        event.update(|arg| {
            // Update event handler
        });
    }
}
