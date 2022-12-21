#include "node.h"
#include <stdlib.h>
#include <string.h>

Node *node_init(void *data, usize n) {
  usize len = sizeof(Node) + n;
  Node *root = malloc(len);
  memset(root, 0, len);

  root->data = root + sizeof(Node);
  memcpy(root->data, data, n);

  return root;
}

void node_insert(Node *root, void *data, usize n) {
  Node *head = root;
  while (head->next) {
    head = head->next;
  }
  head->next = node_init(data, n);
}

void node_free(Node *root) {
  if (root->next) {
    node_free(root->next);
  }
  free(root);
}
