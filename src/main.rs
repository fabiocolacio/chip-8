extern crate sdl2;
extern crate chip8;

use sdl2::keyboard::Scancode;

use chip8::Chip8;

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <path to rom file>", args.get(0).unwrap());
        return;
    }
    
    let sdl_ctx = sdl2::init().unwrap();
    
    let vid_ctx = sdl_ctx.video().unwrap();
    
    let window = vid_ctx.window("Chip8", (chip8::DISPLAY_WIDTH * 10) as u32, (chip8::DISPLAY_HEIGHT * 10) as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
        
    let mut canvas = window.into_canvas().build().unwrap();
    
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(
        sdl2::pixels::PixelFormatEnum::RGB24,
        chip8::DISPLAY_WIDTH as u32,
        chip8::DISPLAY_HEIGHT as u32).unwrap();
    
    let mut data = [0; 3 * chip8::DISPLAY_WIDTH * chip8::DISPLAY_HEIGHT];
    
    let mut event_pump = sdl_ctx.event_pump().unwrap();
    
    let mut chip = Chip8::from_rom_file(&args.get(1).unwrap()).unwrap();
    'mainloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'mainloop,
                _ => {}
            }
        }
        
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
        
        chip.tick();
        
        canvas.clear();
        
        for y in 0 .. chip8::DISPLAY_HEIGHT {
            for x in 0 .. chip8::DISPLAY_WIDTH {
                let state = chip.get_pixel(x, y);
                if state {
                    data[(x * 3) + (y * 3 * chip8::DISPLAY_WIDTH) + 0] = 0xff;
                    data[(x * 3) + (y * 3 * chip8::DISPLAY_WIDTH) + 1] = 0xff;
                    data[(x * 3) + (y * 3 * chip8::DISPLAY_WIDTH) + 2] = 0xff;
                } else {
                    data[(x * 3) + (y * 3 * chip8::DISPLAY_WIDTH) + 0] = 0x00;
                    data[(x * 3) + (y * 3 * chip8::DISPLAY_WIDTH) + 1] = 0x00;
                    data[(x * 3) + (y * 3 * chip8::DISPLAY_WIDTH) + 2] = 0x00;
                }
            }
        }
        
        texture.update(None, &data, chip8::DISPLAY_WIDTH * 3).unwrap();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
        
        for i in 0 .. chip8::DISPLAY_WIDTH * chip8::DISPLAY_HEIGHT * 3 {
            data[i] = 0;
        }
    }
}
