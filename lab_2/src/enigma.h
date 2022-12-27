#pragma once

#include <stddef.h>

unsigned char cipher(unsigned char value);
void cipher_arr(char* array, size_t size);
void cipher_file(int fd);
