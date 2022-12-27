#include <stdio.h>
#include "secure.h"

int main(void)
{
    if (!auth_by_hw_id())
    {
        printf("Я не буду здесь работать!\n");
        return -1;
    }

    printf("Привет, пользователь!\n");
    return 0;
}
