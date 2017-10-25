extern crate sdl2;
extern crate chip8;

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
    
        canvas.clear();
        
        chip.tick();
        
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
        
        texture.update(None, &data, chip8::DISPLAY_WIDTH * 3);
        canvas.copy(&texture, None, None);
        canvas.present();
        
        for i in 0 .. chip8::DISPLAY_WIDTH * chip8::DISPLAY_HEIGHT * 3 {
            data[i] = 0;
        }
    }
}
