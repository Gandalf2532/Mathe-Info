#include <stdio.h>
#include <stdint.h>
#include "randomizer/randomizer.h"
#include "bigint/bigint.h"

unsigned char* generatePrimeNumber(size_t len /*In Bytes*/) {
    unsigned char n[len];

    if (get_random_bytes(n, len)) {
        n[len-1] |= 0x01; //ungerade machen

        unsigned char d[len];
        for(size_t i = 0; i < len; ++i) {
            d[i] = n[i];
        }

        bigint_sub1(d, len);
        int s = 0;
        while(bigintIsEven(d, len)) {
            bigint_div2(d, len);
            s++;
        }     
    }

    return n;
}


int main() {
    unsigned char* p = generatePrimeNumber(128);
    unsigned char* q = generatePrimeNumber(128);
}
