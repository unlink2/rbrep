#ifndef EXPR_H_
#define EXPR_H_ 

#include "error.h"
#include "parser.h"
#include "types.h"
#include <stdio.h>
#include "node.h"

typedef enum ExprKind {
  EXPR_BYTE,
  EXPR_ANY,
  EXPR_GROUP,
  EXPR_STRING,
} ExprKind;

typedef struct ByteExpr {
  u8 val;
} ByteExpr; 

typedef struct Expr {
  union {
    ByteExpr byte;
  };
     
  ExprKind kind;

  Error err;
} Expr;

Node *expr_from(const char *src);

Node *expr_parse(Parser *p);

Error expr_is_err(const Node *root);

void expr_free(Node *root);

Error expr_apply(const char *src, FILE *f);

#endif 
