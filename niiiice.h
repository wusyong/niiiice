#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Kerl Kerl_t;

void sponge_free(Kerl_t *ptr);

const char *sponge_hash_trytes(Kerl_t *ptr, const char *trits, int length);

Kerl_t *sponge_new(void);
