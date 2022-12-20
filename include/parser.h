#ifndef PARSER_H_
#define PARSER_H_ 

#include "error.h"
#include "types.h"

typedef struct Parser {
  const char *src;
  usize len;
  usize pos;

  Error err;
} Parser;

Parser parser_init(const char *src);

char parser_next(Parser *p);

#endif 
