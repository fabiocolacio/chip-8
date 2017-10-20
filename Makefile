build: clean
	mkdir build
	gcc src/*.c -o build/chip8

clean:
	rm -rf build
