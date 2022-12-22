#ifndef BUFFER_H_
#define BUFFER_H_

#include "types.h"
#include <stdio.h>

typedef struct Buffer {
  usize len; // current write position 
  usize pos;  // current read cursor 

  char *data;
  usize max_len; 
} Buffer;

#define BUFFER_DEFAULT_LEN 128 

Buffer buffer_init();

// read from buffer, or from file 
int buffer_read_or(Buffer *b, FILE *or_file);

// reset read cursor by n chars 
void buffer_back(Buffer *b, usize by);

void buffer_resize(Buffer *b);

void buffer_free(Buffer *b);


#endif 
