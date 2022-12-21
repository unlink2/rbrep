#ifndef EXPR_H_
#define EXPR_H_ 

#include "error.h"
#include "parser.h"
#include "types.h"
#include <stdio.h>

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

Expr expr_from(const char *src);

Expr expr_parse(Parser *p);

bool expr_is_err(const Expr *expr);

void expr_free(Expr *expr);

Error expr_apply(const char *src, FILE *f);

#endif 
