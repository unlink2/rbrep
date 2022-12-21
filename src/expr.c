#include <string.h>
#include "expr.h"

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

Node *expr_parse(Parser *p) {
  Expr e = expr_init();

  Node *root = node_init(&e, sizeof(e));

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