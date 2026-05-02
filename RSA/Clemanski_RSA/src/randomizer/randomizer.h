#if defined(_WIN32) || defined(_WIN64)
    #include <windows.h>
    #include <bcrypt.h>
    #pragma comment(lib, "bcrypt.lib")
#elif defined(__linux__)
    #include <unistd.h>
    #include <fcntl.h>
#endif

int get_random_bytes(unsigned char *buf, size_t len);