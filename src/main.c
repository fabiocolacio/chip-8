#include <stdio.h>

#include "chip-8.h"

int main(int argc, char *argv[]) {
    if (argc < 2) {
        printf("Usage: %s <romfile>\n", argv[0]);
    }
        
    chip_8 chip;
    chip_8_init(&chip);
    chip_8_load_rom(&chip, argv[1]);
    
    while (1) {
        chip_8_clock_tick(&chip);
    }
    
    return 0;
}