#include <string.h>
#include <stdio.h>

#include "chip-8.h"

const uint8_t chip_8_sprites[] = {
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
};

// Attempt to load rom into chip-8 memory
// returns 0 on success
// return -1 on error
int chip_8_load_rom(chip_8 *chip, const char *rom_location) {
    if (chip == NULL || rom_location == NULL) return -1;
    
    FILE *rom_file = NULL;
    rom_file = fopen(rom_location, "r");
    if (rom_file != NULL) {
        int res = fread(&chip->mem[0x200], 1, 3584, rom_file);
        fclose(rom_file);
        if (res >= 0) {
            return 0;
        }
    }
    return -1;
}

void chip_8_init(chip_8 *chip) {
    if (chip == NULL) return;
    
    memset(chip, 0, sizeof(chip_8));
    
    // programs are stored in memory from 0x200 - 0xfff
    chip->pc = 0x200;
    
    // install fonts into memory
    for (int addr = 0; addr < 80; addr++) {
        chip->mem[addr] = chip_8_sprites[addr];
    }
}

#define GET_PREFIX(opcode) ((opcode & 0xf000) >> 16)
#define GET_X(opcode) ((opcode & 0x0f00) >> 8)
#define GET_Y(opcode) ((opcode & 0x00f0) >> 4)
#define GET_N(opcode) (opcode & 0x000f)
#define GET_NN(opcode) (opcode & 0x00ff)
#define GET_NNN(opcode) (opcode & 0x0fff)

void chip_8_clock_tick(chip_8 *chip) {
    if (chip == NULL || chip->pc >= 0x1000) return;
    
    uint16_t opcode = (chip->mem[chip->pc++] << 8) | (chip->mem[chip->pc++]);
    uint8_t prefix = GET_PREFIX(opcode);
    
    if (prefix == 0x0) {
        if (opcode == 0x00e0) {
            for (int r = 0; r < CHIP_8_DISPLAY_HEIGHT; r++) {
                for (int c = 0; c < CHIP_8_DISPLAY_WIDTH; c++) {
                    chip->display[r][c] = 0;
                }
            }
        } else if (opcode == 0x00ee) {
            if (chip->sp <= 0) {
                printf("ERROR: Stack Underflow\n");
                return;
            }
            chip->pc = chip->stack[--(chip->sp)];
        } else {
            chip->pc = GET_NNN(opcode);
        }
    }
}
