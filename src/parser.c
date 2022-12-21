#include "parser.h"
#include <string.h>
#include <ctype.h>

Parser parser_init(const char *src) {
  Parser p;
  memset(&p, 0, sizeof(p));

  p.src = src;
  p.len = strlen(src);

  return p;
}

bool parser_end(Parser *p) { return p->pos >= p->len; }

char parser_peek(Parser *p) { return p->src[p->pos]; }

usize parser_trim(Parser *p) {
  usize start = p->pos;
  while (!parser_end(p) && isspace(parser_peek(p))) {
    p->pos++;
  }
  return p->pos - start;
}

char parser_next(Parser *p) {
  if (p->err || parser_end(p)) {
    return '\0';
  }
  parser_trim(p);

  char c = parser_peek(p);
  p->pos++;
  return c;
}
