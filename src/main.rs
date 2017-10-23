extern crate sdl2;
extern crate chip8;

use chip8::Chip8;

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <path to rom file>", args.get(0).unwrap());
        return;
    }
    
    let mut chip = Chip8::from_rom_file(&args.get(1).unwrap()).unwrap();
    loop {
        chip.tick();
    }
}