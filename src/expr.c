#include <string.h>
#include "expr.h"
#include <ctype.h>

Expr expr_init() {
  Expr e;
  memset(&e, 0, sizeof(e));

  return e;
}

Node *expr_from(const char *src) {
  Parser p = parser_init(src);
  Node *root = expr_parse(&p);
  return root;
}

void expr_parse_byte(Parser *p, Node *root, char first) {
  if (p->err) {
    return;
  }
}

Node *expr_parse(Parser *p) {
  if (p->err) {
    return NULL;
  }

  Expr e = expr_init();
  Node *root = node_init(&e, sizeof(e));

  char first = parser_next(p);
  if (isxdigit(first)) {
    expr_parse_byte(p, root, first);
  } else {
    Expr *self = node_get(root);
    err(ERR_BAD_SYNTAX, "Synatax error at '%c' : %ld\n", first, p->pos);
    self->err = ERR_BAD_SYNTAX;
    p->err = ERR_BAD_SYNTAX;
  }

  return root;
}

Error expr_is_err(const Node *root) {
  const Node *head = root;
  while (head) {
    Expr *e = node_get(head);
    if (e->err) {
      return e->err;
    }
    head = head->next;
  }
  return OK;
}

void expr_free(Node *root) {
  // TODO do special frees for certain types
  node_free(root);
}

Error expr_apply(const char *src, FILE *f) { return OK; }