extern crate rand;

use std::fs::File;
use std::io::Read;
use std::time::{Duration, Instant};
use rand::Rng;

/// The width of the Chip8 display
pub const DISPLAY_WIDTH: usize = 64;

/// The height of the Chip8 display
pub const DISPLAY_HEIGHT: usize = 32;

/// The total number of pixels in the Chip8 display buffer
pub const DISPLAY_SIZE: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;

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
    mem: [u8; 0x1000],
    
    /// The Chip8 has 16 8-bit registers ranging from v0 to vf
    v: [u8; 0x10],
    
    /// Whenever the delay timer is active as long as it is non-zero.
    /// The timer decrements its value at a rate of 60Hz until it reaches
    /// zero, and de-activates itself.
    dt: u8,
    
    /// The Chip8's buzzer sounds as long as the sound timer contains a
    /// non-zero value. It decrements itself at a rate of 60Hz until it
    /// reaches zero and de-activates itself.
    st: u8,
    
    /// The stack pointer always points to the top of the stack.
    sp: u8,
    
    /// The stack is used primarily for handling calls to subroutines
    stack: [u16; 0x10],
    
    /// The 16-bit Index register stores memory addresses.
    i: u16,
    
    /// The program counter keeps track of which command to execute next.
    pc: u16,
    
    /// Chip8 computers have a 16-key hexadecimal keypad with keys 0 - F.
    input: [bool; 0x10],
    
    /// Chip8 computers have a 64 x 32 pixel display.
    display: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],

    /// This flag is enabled when the chip executes the draw command
    render_flag: bool,

    /// Used to keep the timers ticking down at 60Hz
    last_cycle: Instant,
}

/// Print a warning that the given opcode was unsupported
fn unsupported_opcode(opcode: u16, pc: u16) {
    println!("[WARNING] opcode 0x{:X} from pc 0x{:X} is unsupported", opcode, pc);
}

impl Chip8 {
    /// Create a Chip8 device and load the specified ROM file into it.
    pub fn from_rom_file(rom_file: &str) -> std::io::Result<Chip8> {
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
            dt: 0,
            st: 0,
            sp: 0,
            stack: [0; 0x10],
            i: 0,
            pc: 0x200,
            input: [false; 0x10],
            display: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            render_flag: false,
            last_cycle: Instant::now(),
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
            dt: 0,
            st: 0,
            sp: 0,
            stack: [0; 0x10],
            i: 0,
            pc: 0x200,
            input: [false; 0x10],
            display: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            render_flag: false,
            last_cycle: Instant::now(),
        }
    }
    
    /// Get the state of the render flag
    pub fn get_render_flag(&self) -> bool {
        self.render_flag
    }

    /// Performs a single Chip8 operation, and updates timers
    pub fn tick(&mut self) {
        let opcode: u16 = (self.mem[self.pc as usize] as u16) << 8; self.pc += 1;
        let opcode: u16 = opcode | (self.mem[self.pc as usize] as u16); self.pc += 1;
     
        self.render_flag = false;

        // Execute the instruction at PC
        self.execute_opcode(opcode);
        
        // Timers decrement themselves at a rate of 60Hz
        if self.last_cycle.elapsed() >= Duration::from_millis(17) {
            self.last_cycle = Instant::now();
            if self.dt > 0 { self.dt -= 1; }
            if self.st > 0 { self.st -= 1; }
        }
    }
    
    /// Executes the given opcode
    fn execute_opcode(&mut self, opcode: u16) {
        let prefix = ((opcode & 0xf000) >> 12) as u8;
        let x = ((opcode & 0x0f00) >> 8) as usize;
        let y = ((opcode & 0x00f0) >> 4) as usize;
        let n = (opcode & 0x000f) as u8;
        let nn = (opcode & 0x00ff) as u8;
        let nnn = (opcode & 0x0fff) as u16;
        
        match prefix {
            0x0 => {
                match nn {
                    // 00e0 clears the display
                    0xe0 => {
                        for y in 0 .. DISPLAY_HEIGHT {
                            for x in 0 .. DISPLAY_WIDTH {
                                self.display[y][x] = false;
                            }
                        }
                    },
                    
                    // 00ee returns from a subroutine
                    0xee => {
                        self.sp -= 1;
                        self.pc = self.stack[self.sp as usize];
                    },
                    
                    _ => {
                        unsupported_opcode(opcode, self.pc);
                        return;
                    },
                }
            },
            
            // 1nnn jumps to location nnn
            0x1 => self.pc = nnn,
            
            // 2nnn calls the subroutine at nnn
            0x2 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            },
            
            // 3xkk skips the next instruction if Vx == kk
            0x3 => if self.v[x] == nn { self.pc += 2 },
            
            // 4xkk skips next instruction if Vx != kk
            0x4 => if self.v[x] != nn { self.pc += 2 },
            
            // 5xy0 skips next instruction if Vx == Vy
            0x5 => if self.v[x] == self.v[y] { self.pc += 2 },
            
            // 6xkk sets Vx = kk
            0x6 => self.v[x] = nn,
            
            // 7xkk sets Vx = Vx + kk
            0x7 => self.v[x] = self.v[x].wrapping_add(nn),
            
            0x8 => {
                match n {
                    // 8xy0 sets Vx = Vy
                    0x0 => self.v[x] = self.v[y],
                    
                    // 8xy1 sets Vx = Vx OR Vy
                    0x1 => self.v[x] |= self.v[y],
                    
                    // 8xy2 sets Vx = Vx AND Vy
                    0x2 => self.v[x] &= self.v[y],
                    
                    // 8xy3 sets Vx = Vx XOR Vy
                    0x3 => self.v[x] ^= self.v[y],
                    
                    // 8xy4 sets Vx = Vx + Vy, sets Vf = carry
                    0x4 => {
                        self.v[0xf] = (self.v[y] > (255 - self.v[x])) as u8;
                        self.v[x] = self.v[x].wrapping_add(self.v[y]);
                    },
                    
                    // 8xy5 sets Vx = Vx - Vy, sets Vf = not borrow
                    0x5 => {
                        self.v[0xf] =  (self.v[x] >= self.v[y]) as u8;
                        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
                    },
                    
                    // 8xy6 sets Vx = Vx >> 1, and stores least significant bit in Vf
                    0x6 => {
                        self.v[0xf] = self.v[x] & 0x1;
                        self.v[x] = self.v[x].wrapping_shr(1);
                    },
                    
                    // 8xy7 sets Vx = Vy - Vx, and sets Vf = not borrow
                    0x7 => {
                        self.v[0xf] = (self.v[y] >= self.v[x]) as u8;
                        self.v[x] = self.v[y].wrapping_sub(self.v[x]);
                    },
                    
                    // 8xye sets Vx = Vx << 1, and stores most significant bit in Vf
                    0xe => {
                        self.v[0xf] = (self.v[x] >> 7) & 0x1;
                        self.v[x] = self.v[x].wrapping_shl(1);
                    },
                    
                    _ => {
                        unsupported_opcode(opcode, self.pc);
                        return;
                    },
                }
            },
            
            // 9xy0 skips the next instruction of Vx != Vy
            0x9 => if self.v[x] != self.v[y] { self.pc += 2 },
            
            // annn sets i to the address at nnn
            0xa => self.i = nnn,
            
            // bnnn jumps to address nnn + v0
            0xb => self.pc = nnn.wrapping_add(self.v[0] as u16),
            
            // cxkk sets Vx to NN ANDed with a random byte
            0xc => self.v[x] = nn & rand::thread_rng().gen_range(0x0, 0xff),
            
            // dxyn draws a sprite at location (Vx, Vy) of height N.
            // The sprite is taken from memory address stored in register i
            0xd => {
                self.render_flag = true;
                for index in 0 .. n as usize {
                    let sprite: u8 = self.mem[self.i as usize + index];
                    
                    let x = self.v[x] as usize;
                    let y = self.v[y] as usize + index;
                    
                    let mut collision = false;

                    for pixel_index in 0 .. 8 {
                        let x = x + pixel_index;
                        let pixel_index = 7 - pixel_index;
                        let pixel = ((sprite >> pixel_index) & 0x1) == 0x1;
                        if self.display[y % DISPLAY_HEIGHT][x % DISPLAY_WIDTH] && pixel {
                            collision = true;
                        }
                        self.display[y % DISPLAY_HEIGHT][x % DISPLAY_WIDTH] ^= pixel;
                    }
                    
                    self.v[0xf] = collision as u8;
                }
            },
            
            0xe => {
                match nn {
                    // ex9e skips the next instruction if the key of index Vx is pressed
                    0x9e => if self.input[self.v[x] as usize] { self.pc += 2 },
                    
                    // exa1 skips the next instruction if the key of index Vx is not pressed
                    0xa1 => if !self.input[self.v[x] as usize] { self.pc += 2 },
                    
                    _ => {
                        unsupported_opcode(opcode, self.pc);;
                        return;
                    },
                }
            },
            
            0xf => {
                match nn {
                    // fx07 sets Vx to the value of the delay timer
                    0x07 => self.v[x] = self.dt,
                    
                    // fx0a waits for a key to be pressed, and store its index in Vx
                    0x0a => {
                        let mut pressed = false;
                        for index in 0 .. 0x10 {
                            if self.input[index] {
                                pressed = true;
                                self.v[x] = index as u8;
                            }
                        }
                        if !pressed {
                            self.pc -= 2;
                        }
                    },
                    
                    // fx15 sets the delay timer to Vx
                    0x15 => self.dt = self.v[x],
                    
                    // fx18 sets the sound timer to Vx
                    0x18 => self.st = self.v[x],
                    
                    // fx1e adds Vx to the address in register i
                    0x1e => self.i = self.i.wrapping_add(self.v[x] as u16),
                    
                    // fx29 ets the register i to the address of sprite Vx
                    0x29 => self.i = 5 * self.v[x] as u16,
                    
                    // fx33 stores the binary-coded decimal representation of Vx.
                    // Most significant 3 digits are stored at i.
                    // Middle digits is stored at i + 1.
                    // Least significant digit is stored at i + 2.
                    0x33 => {
                        let vx = self.v[x];
                        let i = self.i as usize;
                        self.mem[i] = vx / 100;
                        self.mem[i + 1] = (vx / 10) % 10;
                        self.mem[i + 2] = vx % 10;
                    },
                    
                    // fx55 stores registers V0 - Vx into ram starting at location i.
                    0x55 => {
                        for index in 0 .. x + 1 {
                            self.mem[index + self.i as usize] = self.v[index];
                        }
                    },
                    
                    // fx66 fills registers V0 - Vx with data in ram at location i.
                    0x65 => {
                        for index in 0 .. x + 1 {
                            self.v[index] = self.mem[index + self.i as usize];
                        }
                    }
                    
                    _ => {
                        unsupported_opcode(opcode, self.pc);
                        return;
                    },
                }
            },
            
            _  => {
                unsupported_opcode(opcode, self.pc);;
                return;
            },
        }
    }
    
    /// Get the value of the given register
    pub fn get_v(&self, register: usize) -> u8 {
        self.v[register]
    }
    
    /// Get the value of the sound timer
    pub fn get_st(&self) -> u8 {
        self.st
    }
    
    /// Get the value of the delay timer
    pub fn get_dt(&self) -> u8 {
        self.dt
    }
    
    /// Check if the the pixel at the given (x, y) location is on or off
    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.display[y][x]
    }
    
    /// Check if the button of the given hex value is on or off
    pub fn get_input(&self, key: usize) -> bool {
        self.input[key]
    }
    
    /// Set the value of a key on the keypad
    pub fn set_input(&mut self, key: usize, value: bool) {
      self.input[key] = value;
    }
    
    /// Returns true if the chip should be playing a sound
    /// 
    /// If you would like to handle sound yourself, and manually poll
    /// the emulator to determine when to do so, use this function.
    /// Optionally, you can use ``set_audio_callback()`` to define a function
    /// for the emulator to call to play the sounds automatically.
    pub fn sound_status(&self) -> bool {
        self.st > 0
    }
}

impl std::fmt::Display for Chip8 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,
            "REGISTERS \n\
             ========= \n\
             V0 = {} \n\
             V1 = {} \n\
             V2 = {} \n\
             V3 = {} \n\
             V4 = {} \n\
             V5 = {} \n\
             V6 = {} \n\
             V7 = {} \n\
             V8 = {} \n\
             V9 = {} \n\
             VA = {} \n\
             VB = {} \n\
             VC = {} \n\
             VD = {} \n\
             VE = {} \n\
             VF = {} \n\
             \n\
             TIMERS \n\
             ====== \n\
             Delay = {} \n\
             Sound = {} \n\
             \n\
             STACK \n\
             ===== \n\
             Stack Pointer = {} \n\
             Stack[0] = {} \n\
             Stack[1] = {} \n\
             Stack[2] = {} \n\
             Stack[3] = {} \n\
             Stack[4] = {} \n\
             Stack[5] = {} \n\
             Stack[6] = {} \n\
             Stack[7] = {} \n\
             Stack[8] = {} \n\
             Stack[9] = {} \n\
             Stack[A] = {} \n\
             Stack[B] = {} \n\
             Stack[C] = {} \n\
             Stack[D] = {} \n\
             Stack[E] = {} \n\
             Stack[F] = {} \n\
             \n\
             Index \n\
             ===== \n\
             I-Register = {} \n\
             \n\
             Program Counter \n\
             =============== \n\
             PC = {} \n",
             self.v[0],
             self.v[1],
             self.v[2],
             self.v[3],
             self.v[4],
             self.v[5],
             self.v[6],
             self.v[7],
             self.v[8],
             self.v[9],
             self.v[10],
             self.v[11],
             self.v[12],
             self.v[13],
             self.v[14],
             self.v[15],
             self.dt,
             self.st,
             self.sp,
             self.stack[0],
             self.stack[1],
             self.stack[2],
             self.stack[3],
             self.stack[4],
             self.stack[5],
             self.stack[6],
             self.stack[7],
             self.stack[8],
             self.stack[9],
             self.stack[10],
             self.stack[11],
             self.stack[12],
             self.stack[13],
             self.stack[14],
             self.stack[15],
             self.i,
             self.pc)
    }
}
