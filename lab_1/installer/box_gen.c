#include <string.h>
#include <unistd.h>
#include <stdio.h>
#include <fcntl.h>
#include <sys/stat.h>

#define HEADER "#include <stddef.h>\nunsigned char* get_boxed_data(size_t *size) { static unsigned char data[] = {"
#define FOOTER "}; *size = sizeof(data); return data; }"

int main(int argc, char* argv[])
{
    // first arg is binary input file path
    // second arg is text output file path
    if (argc != 3)
        return -1;
    
    FILE* f_in = fopen(argv[1], "rb");
    if (f_in == NULL)
        return -2;
    
    FILE* f_out = fopen(argv[2], "w");
    if (f_out == NULL)
    {
        fclose(f_in);
        return -3;
    }

    fprintf(f_out, "%s\n", HEADER);
    size_t len;

    do
    {
        unsigned char buf[64];
        len = fread(buf, sizeof(unsigned char), 64, f_in);
        for (int i = 0; i < len; i++)
            fprintf(f_out, "%d,", buf[i]);
        fprintf(f_out, "\n");
    } while (len == 64);

    fprintf(f_out, "%s\n", FOOTER);

    fclose(f_out);
    fclose(f_in);
    return 0;
}
