#![windows_subsystem = "windows"]

use chaikin::*;

use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::font::Font;
use speedy2d::font::TextLayout;
use speedy2d::font::TextOptions;
use speedy2d::window::{MouseButton, VirtualKeyCode};
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use speedy2d::Window;

use std::time::Duration;

pub struct MyWindowHandler {
    pub mouse_pos: MousePosition,
    pub clicked_points: ClickedPoints,
    pub smoothed_points: ClickedPoints,
    pub iterations: u8,
    pub launched: bool,
    pub font: Font,
}

pub struct MousePosition {
    pub x: f32,
    pub y: f32,
}

impl MousePosition {
    pub fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    fn set(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::WHITE);

        helper.set_title("Chaikin's curves");

        let block = self.font.layout_text(
            "Press ENTER to start the loop\nPress C to clear the screen\n\nDraw at least 3 points for animation",
            20.0,
            TextOptions::new(),
        );

        graphics.draw_text((10.0, 10.0), Color::BLACK, &block);

        for point in &self.clicked_points.points {
            graphics.draw_circle(Vec2::new(point.0, point.1), 3.0, Color::GRAY)
        }

        if self.clicked_points.points.len() > 1 {
            if self.launched {
                if self.clicked_points.points.len() == 2 {
                    graphics.draw_line(
                        self.clicked_points.points[0],
                        self.clicked_points.points[1],
                        1.0,
                        Color::BLUE,
                    )
                } else {
                    if self.iterations == 0 {
                        self.smoothed_points = self.clicked_points.clone();
                    } else {
                        self.smoothed_points = chaikins(self.smoothed_points.clone());
                    }

                    for i in 0..self.smoothed_points.points.len() - 1 {
                        let p1 = &self.smoothed_points.points[i];
                        let p2 = &self.smoothed_points.points[i + 1];
                        graphics.draw_line(
                            Vec2::new(p1.0, p1.1),
                            Vec2::new(p2.0, p2.1),
                            1.0,
                            Color::BLUE,
                        );
                    }

                    if self.iterations < 7 {
                        self.iterations += 1;

                        let dur = Duration::from_millis(500);

                        spin_sleep::sleep(dur);
                    } else {
                        self.iterations = 0;
                        self.smoothed_points = chaikins(self.clicked_points.clone());
                    }
                }
            }
        }
        // Request that we draw another frame once this one has finished
        helper.request_redraw();
    }

    fn on_mouse_move(&mut self, helper: &mut WindowHelper<()>, position: speedy2d::dimen::Vec2) {
        self.mouse_pos.set(position.x, position.y);

        helper.request_redraw();
    }

    fn on_mouse_button_down(
        &mut self,
        helper: &mut WindowHelper<()>,
        button: speedy2d::window::MouseButton,
    ) {
        if let MouseButton::Left = button {
            if !self.launched {
                self.clicked_points.add(self.mouse_pos.x, self.mouse_pos.y);
            }
        };

        helper.request_redraw();
    }

    fn on_key_down(
        &mut self,
        helper: &mut WindowHelper<()>,
        key: Option<speedy2d::window::VirtualKeyCode>,
        _scancode: speedy2d::window::KeyScancode,
    ) {
        if key == Some(VirtualKeyCode::Return) {
            self.launched = true;
        }
        if key == Some(VirtualKeyCode::Escape) {
            helper.terminate_loop();
        }
        if key == Some(VirtualKeyCode::C) {
            self.launched = false;
            self.clicked_points = ClickedPoints::new();
            self.smoothed_points = ClickedPoints::new();
            self.iterations = 0;
        }

        helper.request_redraw();
    }

    // If desired, on_mouse_move(), on_key_down(), etc...
}

fn main() {
    let window = Window::new_centered("Title", (640, 480)).unwrap();

    let bytes = include_bytes!("../assets/fonts/NotoSans-Regular.ttf");
    let font = Font::new(bytes).unwrap();

    window.run_loop(MyWindowHandler {
        mouse_pos: MousePosition::new(),
        clicked_points: ClickedPoints::new(),
        smoothed_points: ClickedPoints::new(),
        iterations: 0,
        launched: false,
        font,
    });
}
