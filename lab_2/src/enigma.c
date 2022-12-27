#include "enigma.h"
#include <unistd.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

#define BYTES           256
#define ROTORS_COUNT    3
#define TICKS_PER_ROTOR 256
#define BUFF_LEN        8192

typedef struct {
    unsigned char fwd;
    unsigned char bwd;
} pair_t;

static void init_rotor(pair_t rotor[TICKS_PER_ROTOR][BYTES])
{
    for (int tick = 0; tick < TICKS_PER_ROTOR; tick++)
    {
        for (int byte = 1; byte < BYTES; byte++)
        {
            int pos = rand() % BYTES;
            while (rotor[tick][pos].fwd != 0)
                pos = (pos + 1) % BYTES;
            rotor[tick][pos].fwd = byte;
            rotor[tick][byte].bwd = pos;
        }
        // last zero byte
        for (int pos = 0; pos < BYTES; pos++)
        {
            if (rotor[tick][pos].fwd == 0)
            {
                rotor[tick][0].bwd = pos;
                break;
            }
        }
    }
}

static void init_reflector(unsigned char reflector[BYTES])
{
    int zero_pos = rand() % BYTES;
    reflector[0] = zero_pos;

    int i = 0;
    while (1)
    {
        while (i < BYTES && reflector[i] != 0)
            i++;

        if (i >= BYTES)
            break;
        
        if (i == zero_pos)
        {
            i++;
            continue;
        }

        int pos = rand() % BYTES;
        while (reflector[pos] != 0 || pos == zero_pos)
            pos = (pos + 1) % BYTES;
        reflector[pos] = i;
        reflector[i] = pos;
    }
}

unsigned char cipher(unsigned char value)
{
    static pair_t rotors[ROTORS_COUNT][TICKS_PER_ROTOR][BYTES];
    static unsigned char reflector[BYTES];
    static long counter = -1;

    if (counter == -1)
    {
        init_reflector(reflector);
        for (int i = 0; i < ROTORS_COUNT; i++)
            init_rotor(rotors[i]);
    }

    counter = (counter + 1) % (ROTORS_COUNT * TICKS_PER_ROTOR);
    unsigned long rem = counter;
    unsigned long rems[ROTORS_COUNT];

    for (int i = 0; i < ROTORS_COUNT; i++)
    {
        rems[i] = rem % TICKS_PER_ROTOR;
        rem /= TICKS_PER_ROTOR;
        value = rotors[i][rems[i]][value].fwd;
    }

    value = reflector[value];

    for (int i = ROTORS_COUNT - 1; i >=0; i--)
        value = rotors[i][rems[i]][value].bwd;

    return value;
}

void cipher_arr(char* array, size_t size)
{
    for (size_t i = 0; i < size; i++)
        array[i] = cipher(array[i]);
}

void cipher_file(int fd)
{
    char buffer[BUFF_LEN];
    ssize_t size;

    while ((size = read(fd, buffer, BUFF_LEN)) > 0)
    {
        cipher_arr(buffer, size);
        lseek(fd, -size, SEEK_CUR);
        if (write(fd, buffer, size) == -1)
            return;
    }
}
