extern crate sdl2;
use sdl2::keyboard::Scancode;
use sdl2::event::Event;
use sdl2::pixels::Color;

extern crate chip8;
use chip8::{ Chip8, DISPLAY_WIDTH, DISPLAY_HEIGHT };

mod screen;
use screen::Screen;

mod buzzer;
use buzzer::Buzzer;

use std::time::{Duration, Instant};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <path to rom file>", args.get(0).unwrap());
        return;
    }
    
    // setup SDL2 subsystems
    let sdl_ctx = sdl2::init().unwrap();
    let vid_ctx = sdl_ctx.video().unwrap();
    let audio_ctx = sdl_ctx.audio().unwrap();
    let mut event_pump = sdl_ctx.event_pump().unwrap();
    
    // clock speed for the execution of chip-8 commands in MHz
    let clock_speed = 1;
    let clock_speed = (1 / (clock_speed)) * 1000000;
    let clock_speed = Duration::new(0, clock_speed);
    let mut last_cycle = Instant::now();
    
    // colors for pixels that are 'on' and 'off'
    let on_color = Color::RGB(0xff, 0xff, 0xff);
    let off_color = Color::RGB(0x00, 0x00, 0x00);
    
    // setup window to render graphics into
    let mut window = Screen::new(&vid_ctx);
    window.set_scale(10, 10);
    
    // setup buzzer to play sounds
    let buzzer = Buzzer::new(&audio_ctx);
    
    // setup chip-8 emulator structure
    let mut chip = Chip8::from_rom_file(&args.get(1).unwrap()).unwrap();
    
    'mainloop: loop {
        let dt = last_cycle.elapsed();
    
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{ .. } => break 'mainloop,
                _ => (),
            }
        }
        
        // run the emulator at at the given clock speed
        if dt >= clock_speed {
            last_cycle = Instant::now();
            
            // update the state of each key of the emulator's keyboard
            update_keypad(&mut chip, &event_pump);
            
            // run one clock cycle
            chip.tick();  
            
            // update the host's buzzer with the state of the chip's sound timer
            buzzer.set(chip.sound_status());
            
            // update host's window with chip's graphics
            for y in 0 .. DISPLAY_HEIGHT {
                for x in 0 .. DISPLAY_WIDTH {
                    let color =  if chip.get_pixel(x, y) {
                        on_color
                    } else {
                        off_color
                    };
                    window.set_pixel(color, x as i32, y as i32);
                }
            }
            window.update();
        }
    }
}

fn update_keypad(chip: &mut Chip8, event_pump: &sdl2::EventPump) {
    let keyboard_state = event_pump.keyboard_state();
    chip.set_input(0x1, keyboard_state.is_scancode_pressed(Scancode::Num1));
    chip.set_input(0x2, keyboard_state.is_scancode_pressed(Scancode::Num2));
    chip.set_input(0x3, keyboard_state.is_scancode_pressed(Scancode::Num3));
    chip.set_input(0xC, keyboard_state.is_scancode_pressed(Scancode::Num4));
    chip.set_input(0x4, keyboard_state.is_scancode_pressed(Scancode::Q));
    chip.set_input(0x5, keyboard_state.is_scancode_pressed(Scancode::W));
    chip.set_input(0x6, keyboard_state.is_scancode_pressed(Scancode::E));
    chip.set_input(0xD, keyboard_state.is_scancode_pressed(Scancode::R));
    chip.set_input(0x7, keyboard_state.is_scancode_pressed(Scancode::A));
    chip.set_input(0x8, keyboard_state.is_scancode_pressed(Scancode::S));
    chip.set_input(0x9, keyboard_state.is_scancode_pressed(Scancode::D));
    chip.set_input(0xE, keyboard_state.is_scancode_pressed(Scancode::F));
    chip.set_input(0xA, keyboard_state.is_scancode_pressed(Scancode::Z));
    chip.set_input(0x0, keyboard_state.is_scancode_pressed(Scancode::X));
    chip.set_input(0xB, keyboard_state.is_scancode_pressed(Scancode::C));
    chip.set_input(0xF, keyboard_state.is_scancode_pressed(Scancode::V));
}
