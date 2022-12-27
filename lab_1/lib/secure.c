#include <fcntl.h>
#include <unistd.h>
#include "secure.h"

static union {
    __uint128_t key;
    struct {
        unsigned long low;
        unsigned long high;
    } s;
} hw_key = {
    .s.low = 0xAAAAAAAAAAAAAAAA,
    .s.high = 0xAAAAAAAAAAAAAAAA
};

__uint128_t get_hw_id(void)
{
    int fd = open("/etc/machine-id", O_RDONLY);
    if (fd == -1)
        return -1;

    char buf[32];
    read(fd, &buf, 32 * sizeof(char));
    close(fd);

    __uint128_t id = 0;
    for (int i = 0; i < 32; i++)
        id += (id << 4) + (id >> 124) + buf[i] - (buf[i] > '9' ? 'a' + 10 : '0');

    return id;
}

bool auth_by_hw_id(void)
{
    return hw_key.key == get_hw_id();
}
