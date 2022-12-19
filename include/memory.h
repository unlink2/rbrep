#ifndef MEMORY_H_ 
#define MEMORY_H_  
#include <stdlib.h> 
#include "types.h"

void *mp_malloc(usize len);

void mp_free(void *ptr);

#endif 