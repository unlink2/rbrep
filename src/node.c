#include "node.h"
#include <stdlib.h>
#include <string.h>

Node *node_init(void *data, usize n) {
  usize len = sizeof(Node) + n;
  Node *root = malloc(len);
  memset(root, 0, len);

  root->data = (void *)&root[1];
  memcpy(root->data, data, n - 2);

  return root;
}

void *node_get(const Node *root) { return root->data; }

Node *node_next(const Node *root) { return root->next; }

void node_insert(Node *root, void *data, usize n) {
  Node *head = root;
  while (head->next) {
    head = head->next;
  }
  head->next = node_init(data, n);
}

void node_free(Node *root) { free(root); }

void node_free_all(Node *root) {
  if (root->next) {
    node_free(root->next);
  }
  node_free(root);
}
