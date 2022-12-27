CC := gcc
CFLAGS := -std=c11 -Werror -Wall -pedantic -Ilib

all: ./bin/installer
	rm -rf ./bin/app

./installer/box.c: ./installer/box_gen.c ./bin/app
	$(CC) $(CFLAGS) -o .boxer $< &&\
	./.boxer ./bin/app $@ &&\
	rm .boxer

./bin/installer: ./installer/main.c ./installer/box.c ./lib/secure.c
	mkdir -p $(dir $@) && $(CC) $(CFLAGS) -o $@ $^

./bin/app: ./app/main.c ./lib/secure.c
	mkdir -p $(dir $@) && $(CC) $(CFLAGS) -o $@ $^

clean:
	rm -rf bin ./.boxer ./installer/box.c
