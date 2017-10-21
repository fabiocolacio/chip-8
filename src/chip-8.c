#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <sys/time.h>

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
    
    time_t t;
    srand((unsigned) time(&t));
}

int chip_8_key_down(const chip_8 *chip, uint8_t key) {
    return (chip->input >> key) & 0x1;
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
            size_t size = CHIP_8_DISPLAY_WIDTH * CHIP_8_DISPLAY_HEIGHT / 8;
            memset(chip->display, 0, size);
        } else if (opcode == 0x00ee) {
            if (chip->sp <= 0) {
                printf("ERROR: Stack Underflow\n");
                return;
            }
            chip->pc = chip->stack[(chip->sp)--];
        } else {
            chip->pc = GET_NNN(opcode);
        }
    } else if (prefix == 0x1) {
        chip->pc = GET_NNN(opcode);
    } else if (prefix == 0x2) {
        if (chip->sp + 1 > 15) {
            printf("ERROR: Stack Overflow\n");
            return;
        }
        chip->stack[++(chip->sp)] = chip->pc;
        chip->pc = GET_NNN(opcode);
    } else if (prefix == 0x3) {
        if (chip->v[GET_X(opcode)] == GET_NN(opcode)) {
            chip->pc += 2;
        }
    } else if (prefix == 0x4) {
        if (chip->v[GET_X(opcode)] != GET_NN(opcode)) {
            chip->pc += 2;
        }
    } else if (prefix == 0x5) {
        if (chip->v[GET_X(opcode)] == chip->v[GET_Y(opcode)]) {
            chip->pc += 2;
        }
    } else if (prefix == 0x6) {
        chip->v[GET_X(opcode)] = GET_NN(opcode);
    } else if (prefix == 0x7) {
        chip->v[GET_X(opcode)] += GET_NN(opcode);
    } else if (prefix == 0x8) {
        if (GET_N(opcode) == 0x0) {
            chip->v[GET_X(opcode)] = chip->v[GET_Y(opcode)];
        } else if (GET_N(opcode) == 0x1) {
            chip->v[GET_X(opcode)] |= chip->v[GET_Y(opcode)];
        } else if (GET_N(opcode) == 0x2) {
            chip->v[GET_X(opcode)] &= chip->v[GET_Y(opcode)];
        } else if (GET_N(opcode) == 0x3) {
            chip->v[GET_X(opcode)] ^= chip->v[GET_Y(opcode)];
        } else if (GET_N(opcode) == 0x4) {
            uint8_t x = chip->v[GET_X(opcode)];
            uint8_t y = chip->v[GET_Y(opcode)];
            if (x + y > 255) {
                chip->v[0xf] = 1;
            } else {
                chip->v[0xf] = 0;
            }
            chip->v[GET_X(opcode)] = x + y;
        } else if (GET_N(opcode) == 0x5) {
            uint8_t x = chip->v[GET_X(opcode)];
            uint8_t y = chip->v[GET_Y(opcode)];
            if (x > y) {
                chip->v[0xf] = 1;
            } else {
                chip->v[0xf] = 0;
            }
            chip->v[GET_X(opcode)] = x - y;
        } else if (GET_N(opcode) == 0x6) {
            uint8_t x = chip->v[GET_X(opcode)];
            chip->v[0xf] = x & 0x1;
            chip->v[GET_X(opcode)] >>= 1;
        } else if (GET_N(opcode) == 0x7) {
            uint8_t x = chip->v[GET_X(opcode)];
            uint8_t y = chip->v[GET_Y(opcode)];
            if (y > x) {
                chip->v[0xf] = 1;
            } else {
                chip->v[0xf] = 0;
            }
            chip->v[GET_X(opcode)] = y - x;
        } else if (GET_N(opcode) == 0xe) {
            uint8_t x = chip->v[GET_X(opcode)];
            chip->v[0xf] = (x >> 7) & 0x1;
            chip->v[GET_X(opcode)] <<= 1;
        }
    } else if (prefix == 0x9) {
        if (chip->v[GET_X(opcode)] != chip->v[GET_Y(opcode)]) {
            chip->pc += 2;
        }
    } else if (prefix == 0xa) {
        chip->i = GET_NNN(opcode);
    } else if (prefix == 0xb) {
        chip->pc = GET_NNN(opcode) + chip->v[0];
    } else if (prefix == 0xc) {
        chip->v[GET_X(opcode)] = (rand() % 256) & GET_NN(opcode);
    } else if (prefix == 0xd) {
        uint8_t width = CHIP_8_DISPLAY_WIDTH / 8;
        uint8_t height = CHIP_8_DISPLAY_HEIGHT / 8;
        uint8_t x = GET_X(opcode);
        uint8_t y = GET_Y(opcode);
        uint8_t n = GET_N(opcode);
        uint16_t i = chip->i;
        size_t size = CHIP_8_DISPLAY_WIDTH * CHIP_8_DISPLAY_HEIGHT / 8;
        
        for (uint8_t count = 0; count < n; count++) {
            chip->display[x + (width * y++) % size] ^= chip->mem[i + count];
        }
    } else if (prefix == 0xe) {
        if (GET_NN(opcode) == 0x9e) {
            if (chip_8_key_down(chip, chip->v[GET_X(opcode)])) {
                chip->pc += 2;
            }
        } else if (GET_NN(opcode) == 0xa1) {
            if (!chip_8_key_down(chip, chip->v[GET_X(opcode)])) {
                chip->pc += 2;
            }
        }
    } else if (prefix == 0xf) {
        if (GET_NN(opcode) == 0x07) {
            chip->v[GET_X(opcode)] = chip->delay_timer;
        } else if (GET_NN(opcode) == 0x0a) {
            int keypress = 0;
            for (uint8_t key = 0; key < 0x10; key++) {
                if (chip_8_key_down(chip, key)) {
                    chip->v[GET_X(opcode)] = 0xff;
                    keypress = 1;
                }
            }
            if (!keypress) {
                chip->pc -= 2;
                return;
            }
        } else if (GET_NN(opcode) == 0x15) {
            chip->delay_timer = chip->v[GET_X(opcode)];
        } else if (GET_NN(opcode) == 0x18) {
            chip->sound_timer = chip->v[GET_X(opcode)];
        } else if (GET_NN(opcode) == 0x1e) {
            chip->i = chip->v[GET_X(opcode)] + chip->i;
        } else if (GET_NN(opcode) == 0x29) {
            chip->i = chip->v[GET_X(opcode)] * 5;
        } else if (GET_NN(opcode) == 0x33) {
            uint8_t x = GET_X(opcode);
			chip->mem[chip->i]	= chip->v[x] / 100;
			chip->mem[chip->i + 1] = (chip->v[x] / 10) % 10;
			chip->mem[chip->i + 2] = chip->v[x] % 10;
        } else if (GET_NN(opcode) == 0x55) {
            uint8_t x = GET_X(opcode);
            for (int i = 0; i <= x; i++) {
                chip->mem[chip->i + i] = chip->v[i];
            }
        } else if (GET_NN(opcode) == 0x65) {
            uint8_t x = GET_X(opcode);
            for (int i = 0; i <= x; i++) {
                chip->v[i] = chip->mem[chip->i + i];
            }
        }
    }
    
    static struct timeval lastFireTime = {.tv_sec = 0, .tv_usec = 0};
	struct timeval currentTime;
	struct timeval timeDiff;
	
	gettimeofday(&currentTime, NULL);
	timersub(&currentTime, &lastFireTime, &timeDiff);
	double totalTime = (timeDiff.tv_sec * 1000000.0 + timeDiff.tv_usec) / 1000000.0;
	
	if (totalTime >= 1.0/60.0f) {
		
		lastFireTime = currentTime;
		
		if (chip->delay_timer > 0) {
			chip->delay_timer--;
		}
		
		if (chip->sound_timer > 0) {
			// TODO: play sound here
			chip->sound_timer--;
		}
	}
}
