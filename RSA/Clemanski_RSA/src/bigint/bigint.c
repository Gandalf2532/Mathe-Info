#include "bigint.h"

int bigintIsEven(unsigned char* n, size_t len) {
    return (n[len-1] & 0x01) == 0;
}

void bigint_sub1(unsigned char *n, size_t len) {
    for (int i = len - 1; i >= 0; i--) {
        if (n[i] > 0) {
            n[i]--;
            break;
        } else {
            n[i] = 0xFF;
        }
    }
}

void bigint_div2(unsigned char *n, size_t len) {
    unsigned char carry = 0;
    
    for (size_t i = 0; i < len; i++) {
        unsigned char new_carry = n[i] & 1;
        n[i] = (n[i] >> 1) | (carry << 7);
        carry = new_carry;
    }
}
