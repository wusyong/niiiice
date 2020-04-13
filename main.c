#include "niiiice.h"
#include <stdio.h>

// cargo build
// gcc main.c -L. -l:target/debug/libniiiice.so
int main(int argc, char *argv[]) {
    Kerl_t *sponge = sponge_new();
    char trits[243] = {0};

    const char *hash = sponge_hash_trytes(sponge, trits, 243);
    for (int i = 0; i < 243; i ++) {
        printf("%d", hash[i]);
    }

    sponge_free(sponge);
}
