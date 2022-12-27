#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>
#include "enigma.h"

int main(int argc, char* argv[])
{
    srand(0xA5A5A5A5);

    // filename as first arg
    if (argc != 2)
    {
        printf("usage: %s <filename>\n", argv[0]);
        return -1;
    }
    
    int fd = open(argv[1], O_RDWR);
    if (fd == -1)
    {
        perror(argv[1]);
        return -1;
    }

    cipher_file(fd);
    close(fd);
    return 0;
}
