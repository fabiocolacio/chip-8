extern crate sdl2;
extern crate chip8;

use sdl2::VideoSubsystem;
use sdl2::video::Window;
use sdl2::render::{ Canvas };
use sdl2::pixels::{ Color };

use chip8::{ DISPLAY_WIDTH, DISPLAY_HEIGHT };

pub struct Screen {
    canvas: Canvas<Window>,
}

impl Screen {
    pub fn new(video_subsystem: &VideoSubsystem) -> Screen {
        let window = video_subsystem
            .window("Chip8", (DISPLAY_WIDTH) as u32, (DISPLAY_HEIGHT) as u32)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        
        let canvas = window.into_canvas().build().unwrap();
        
        Screen {
            canvas,
        }
    }
    
    pub fn set_scale(&mut self, x_scale: u32, y_scale: u32) {
        let window_size = (DISPLAY_WIDTH as u32 * x_scale, DISPLAY_HEIGHT as u32 * y_scale);
        self.canvas.set_scale(x_scale as f32, y_scale as f32);    
        self.canvas.window_mut().set_size(window_size.0, window_size.1).unwrap();
    }
    
    pub fn set_pixel(&mut self, color: Color, x: i32, y: i32) {
        self.canvas.set_draw_color(color);
        self.canvas.draw_point((x, y)).unwrap();
    }
    
    pub fn update(&mut self) {
        self.canvas.present()
    }
}
