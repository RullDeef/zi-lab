#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <sys/stat.h>
#include "secure.h"

#define MAX_PATH_LEN 512
#define APP_POSTFIX "/best-app"

unsigned char* get_boxed_data(size_t *size);
void apply_hw_key(unsigned char* data, size_t size);
int install(const char* path);

int main(void)
{
    int status = EXIT_SUCCESS;
    char install_path[MAX_PATH_LEN + sizeof(APP_POSTFIX)];

    printf("destination folder (empty for current dir): ");

    if (fgets(install_path, MAX_PATH_LEN, stdin) == NULL)
        status = EXIT_FAILURE;
    else
    {
        while (strlen(install_path) > 0 && isspace(install_path[strlen(install_path) - 1]))
            install_path[strlen(install_path) - 1] = '\0';
        
        if (strlen(install_path) == 0)
            strcpy(install_path, ".");
        
        strcat(install_path, APP_POSTFIX);
        status = install(install_path);
    }

    return status;
}

int install(const char* path)
{
    FILE *f = fopen(path, "wb");
    if (f == NULL)
        return EXIT_FAILURE;

    int status = EXIT_SUCCESS;
    size_t size;
    unsigned char* data = get_boxed_data(&size);
    apply_hw_key(data, size);

    if (fwrite(data, sizeof(unsigned char), size, f) == -1)
        status = EXIT_FAILURE;

    fclose(f);

    if (status == EXIT_SUCCESS)
        chmod(path, S_IRWXU | S_IRWXG | S_IROTH | S_IXOTH);
    
    return status;
}

void apply_hw_key(unsigned char* data, size_t size)
{
    for (size_t i = 0; i + 16 < size; i++)
    {
        int match = 1;
        for (size_t j = i; match && j < i + 16; j++)
            if (data[j] != 0xAA)
                match = 0;
        if (match)
        {
            __int128_t key = get_hw_id();
            memcpy(data + i, &key, 16);
        }
    }
}
