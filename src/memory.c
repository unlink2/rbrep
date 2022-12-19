#include "memory.h"

void *mp_malloc(usize len) { return malloc(len); }

void mp_free(void *ptr) { free(ptr); }
