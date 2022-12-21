#ifndef NODE_H_
#define NODE_H_ 

#include "types.h"

typedef struct Node {
  struct Node *next;
  void *data;
} Node;

Node *node_init(void *data, usize n);

void *node_get(const Node *root);
Node *node_next(const Node *root);

void node_insert(Node *root, void *data, usize n);
void node_free(Node *root);
void node_free_all(Node *root);

#endif 
