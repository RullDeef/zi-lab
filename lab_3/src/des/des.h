#pragma once

#include <cstdint>

uint64_t des_encrypt(uint64_t message, uint64_t key);
uint64_t des_decrypt(uint64_t message, uint64_t key);
