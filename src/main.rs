extern crate sdl2;
extern crate chip8;

mod buzzer;

use std::time::{Duration, Instant};
use sdl2::keyboard::Scancode;
use sdl2::event::Event;
use sdl2::pixels::PixelFormatEnum;

use chip8::{ Chip8, DISPLAY_WIDTH, DISPLAY_HEIGHT, DISPLAY_SIZE };
use buzzer::Buzzer;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <path to rom file>", args.get(0).unwrap());
        return;
    }
    
    let sdl_ctx = sdl2::init().unwrap();
    let vid_ctx = sdl_ctx.video().unwrap();
    let audio_ctx = sdl_ctx.audio().unwrap();
    let mut event_pump = sdl_ctx.event_pump().unwrap();
    
    let buzzer = Buzzer::new(&audio_ctx);
    
    let window = vid_ctx
        .window("Chip8", (DISPLAY_WIDTH * 10) as u32, (DISPLAY_HEIGHT * 10) as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
        
    let mut canvas = window.into_canvas().build().unwrap();
    
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(
        PixelFormatEnum::RGB24,
        DISPLAY_WIDTH as u32,
        DISPLAY_HEIGHT as u32).unwrap();
    
    let mut dt = Instant::now();
    
    let mut chip = Chip8::from_rom_file(&args.get(1).unwrap()).unwrap();
    
    chip.set_audio_callback(|state| buzzer.set(state));
    
    'mainloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'mainloop,
                _ => {}
            }
        }
        
        // Update the state of each key of the emulator's keyboard
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
        
        // Run the emulator at 1MHz
        if dt.elapsed() >= Duration::from_millis(1) {
            dt = Instant::now();
            chip.tick();
        }
        
        let mut data = [0; 3 * DISPLAY_SIZE];
        
        for y in 0 .. DISPLAY_HEIGHT {
            for x in 0 .. DISPLAY_WIDTH {
                let state =  if chip.get_pixel(x, y) { 0xff } else { 0x00 };
                for z in 0 .. 3 {
                    data[(x * 3) + (y * 3 * DISPLAY_WIDTH) + z] = state;
                }
            }
        }
        
        //buzzer.toggle(chip.sound_status());
        
        texture.update(None, &data, DISPLAY_WIDTH * 3).unwrap();
        canvas.clear();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
    }
}
