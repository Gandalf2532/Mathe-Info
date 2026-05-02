#include "randomizer.h"

int get_random_bytes(unsigned char *buf, size_t len) {

#if defined(_WIN32) || defined(_WIN64)

    NTSTATUS status = BCryptGenRandom(
        NULL,
        buf,
        (ULONG)len,
        BCRYPT_USE_SYSTEM_PREFERRED_RNG
    );

    return status == 0;

#elif defined(__linux__)

    int fd = open("/dev/urandom", O_RDONLY);
    if (fd < 0) return 0;

    ssize_t r = read(fd, buf, len);
    close(fd);

    return r == (ssize_t)len;

#else
    #error "Unsupported platform"
#endif
}