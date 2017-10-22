use std::fs::File;
use std::io::Read;
use std::io;

/// The width of the Chip8 display
pub const DISPLAY_WIDTH: usize = 64;
/// The height of the Chip8 display
pub const DISPLAY_HEIGHT: usize = 32;
const DISPLAY_BUFFER_SIZE: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT / 8;

/// The default fontset for the Chip8 contains sprites for each
/// hexadecimal digit (0 - F).
///
/// Each byte represents a single row of 8 pixels across the screen
/// horizontally. Each sprite is 5 bytes long (8x5 pixel sprites).
pub const FONT: [u8; 80] = [
    0xf0, 0x90, 0x90, 0x90, 0xf0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xf0, 0x10, 0xf0, 0x80, 0xf0, // 2
    0xf0, 0x10, 0xf0, 0x10, 0xf0, // 3
    0x90, 0x90, 0xf0, 0x10, 0x10, // 4
    0xf0, 0x80, 0xf0, 0x10, 0xf0, // 5
    0xf0, 0x80, 0xf0, 0x90, 0xf0, // 6
    0xf0, 0x10, 0x20, 0x40, 0x40, // 7
    0xf0, 0x90, 0xf0, 0x90, 0xf0, // 8
    0xf0, 0x90, 0xf0, 0x10, 0xf0, // 9
    0xf0, 0x90, 0xf0, 0x90, 0x90, // A
    0xe0, 0x90, 0xe0, 0x90, 0xe0, // B
    0xf0, 0x80, 0x80, 0x80, 0xf0, // C
    0xe0, 0x90, 0x90, 0x90, 0xe0, // D
    0xf0, 0x80, 0xf0, 0x80, 0xf0, // E
    0xf0, 0x80, 0xf0, 0x80, 0x80  // F
];

/// A Structure that emulates the architecture of the Chip8 computer.
pub struct Chip8 {
    /// The Chip8 has 4KB of RAM.
    ///
    /// The original Interpreter and fonts takes up the first
    /// 512 bytes of ram, so program roms can use the space
    /// from 0x200 - 0xfff
    pub mem: [u8; 0x1000],
    
    /// The Chip8 has 16 8-bit registers ranging from v0 to vf
    pub v: [u8; 0x10],
    
    /// Whenever the delay timer is active as long as it is non-zero.
    /// The timer decrements its value at a rate of 60Hz until it reaches
    /// zero, and de-activates itself.
    pub delay_timer: u8,
    
    /// The Chip8's buzzer sounds as long as the sound timer contains a
    /// non-zero value. It decrements itself at a rate of 60Hz until it
    /// reaches zero and de-activates itself.
    pub sound_timer: u8,
    
    /// The stack pointer always points to the top of the stack.
    pub stack_pointer: u8,
    
    /// The stack stores 16-bit vales and has max nesting of 16
    pub stack: [u16; 0x10],
    
    /// The I register is a special 16-bit register used for storing
    /// memory addresses.
    pub i: u16,
    
    /// The program counter keeps track of which command is to
    /// be executed next.
    pub program_counter: u16,
    
    /// Chip8 computers have a 16-key hexadecimal keypad with keys 0 - F.
    pub input: u8,
    
    /// Chip8 computers have a 64 x 32 pixel display.
    pub display: [u8; DISPLAY_BUFFER_SIZE],
}

impl Chip8 {
    /// Create a Chip8 device and load the given ROM file into it.
    pub fn from_rom_file(rom_file: &str) -> io::Result<Chip8> {
        let mut ram: [u8; 0x1000] = [0; 0x1000];
        
        // load rom data into memory
        let mut rom_data: [u8; 0xe00] = [0; 0xe00];
        let mut file = File::open(rom_file)?;
        file.read(&mut rom_data)?;
        for i in 0 .. 0xe00 {
            ram[i + 0x200] = rom_data[i];
        }
        
        // load fonts into memory
        for i in 0 .. 80 {
            ram[i] = FONT[i];
        }
        
        Ok(Chip8 {
            mem: ram,
            v: [0; 0x10],
            delay_timer: 0,
            sound_timer: 0,
            stack_pointer: 0,
            stack: [0; 0x10],
            i: 0,
            program_counter: 0x200,
            input: 0,
            display: [0; DISPLAY_BUFFER_SIZE],
        })
    }
    
    /// Create a Chip8 device and load the given ROM data into it.
    pub fn with_rom_data(rom_data: [u8; 0xe00]) -> Chip8 {
        let mut ram: [u8; 0x1000] = [0; 0x1000];
        
        // load rom data into memory
        for i in 0 .. 0xe00 {
            ram[i + 0x200] = rom_data[i];
        }
        
        // load fonts into memory
        for i in 0 .. 80 {
            ram[i] = FONT[i];
        }
        
        Chip8 {
            mem: ram,
            v: [0; 0x10],
            delay_timer: 0,
            sound_timer: 0,
            stack_pointer: 0,
            stack: [0; 0x10],
            i: 0,
            program_counter: 0x200,
            input: 0,
            display: [0; DISPLAY_BUFFER_SIZE],
        }
    }
}
