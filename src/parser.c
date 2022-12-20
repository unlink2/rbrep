#include "parser.h"
#include <string.h>

Expr expr_init() {
  Expr e;
  memset(&e, 0, sizeof(e));

  return e;
}

Expr expr_from(const char *src) {
  Expr e = expr_init();
  return e;
}

bool expr_is_err(const Expr *expr) { return FALSE; }

void expr_free(Expr *expr) {}

Error expr_apply(const char *src, FILE *f) { return OK; }
