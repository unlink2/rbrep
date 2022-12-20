#ifndef PARSER_H_
#define PARSER_H_ 

#include "error.h"
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
  struct Expr *next;
  
  Error err;
} Expr;

Expr expr_from(const char *src);

void expr_free(Expr *expr);

Error expr_apply(const char *src, FILE *f);

#endif 
