#include "buffer.h"
#include <stdlib.h>
#include <string.h>

Buffer buffer_init() {
  Buffer b;
  memset(&b, 0, sizeof(b));

  return b;
}

// read from buffer, or from file
int buffer_read_or(Buffer *b, FILE *or_file) { return 0; }

// reset read cursor by n chars
void buffer_back(Buffer *b, usize by) { b->len = b->len - by; }

void buffer_resize(Buffer *b) {}

void buffer_free(Buffer *b) { free(b->data); }
