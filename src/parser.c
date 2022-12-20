#include "parser.h"
#include <string.h>

Parser parser_init(const char *src) {
  Parser p;
  memset(&p, 0, sizeof(p));

  p.src = src;
  p.len = strlen(src);

  return p;
}

char parser_next(Parser *p) {
  if (p->err && p->pos >= p->len) {
    return '\0';
  }

  return p->src[p->pos];
}
