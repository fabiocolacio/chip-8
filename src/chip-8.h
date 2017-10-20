// CHIP-8 specs here:
// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM

#ifndef __CHIP_8__
#define __CHIP_8__

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>

#define CHIP_8_DISPLAY_WIDTH 64
#define CHIP_8_DISPLAY_HEIGHT 32

typedef struct chip_8 {
    // Memory & Registers //
    uint8_t mem[0x1000]; // 4KB RAM
    uint8_t v[16]; // general purpose registers
    uint8_t sound_timer;
    uint8_t delay_timer;
    uint8_t sp; // stack pointer
    uint16_t i; // stores memory address
    uint16_t pc; // program counter
    uint16_t stack[16];
    
    // Display //
    uint8_t display[64][32];
} chip_8;

void chip_8_init(chip_8 *chip);

int chip_8_load_rom(chip_8 *chip, const char *rom_location);

void chip_8_clock_tick(chip_8 *chip);

#ifdef __cplusplus
}
#endif

#endif