all: cip

cip: src/main.c src/enigma.c
	gcc -std=c2x -Wall -pedantic -o $@ $^

clean:
	rm -f cip
