use piston_window::*;

pub fn create_window() -> PistonWindow {
    WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap()
}

pub fn draw_screen(window: &mut PistonWindow, event: &Event, gfx: &[u8; 64 * 32]) {
    window.draw_2d(event, |context, graphics, _device| {
        clear([1.0; 4], graphics);
        rectangle([1.0, 0.0, 0.0, 1.0], // red
                  [0.0, 0.0, 100.0, 100.0],
                  context.transform,
                  graphics);
    });
}