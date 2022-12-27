#include <iostream>
#include <iomanip>
#include <bitset>
#include <bitset>
#include "des/des.h"
#include "des/bitstr.hpp"

int main(int argc, char* argv[])
{
    uint64_t key = 0x133457799BBCDFF1U;
    uint64_t msg = 0x0123456789ABCDEFU;

    uint64_t chipher = des_encrypt(key, msg);
    uint64_t msg_new = des_decrypt(key, chipher);

    std::cout << "original: " << std::hex << msg << std::endl;
    std::cout << "chipher:  " << std::hex << chipher << std::endl;
    std::cout << "dechiper: " << std::hex << msg_new << std::endl;

    return 0;
}
