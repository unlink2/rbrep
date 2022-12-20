#include "str.h"
#include <stdlib.h>
#include <string.h>
#include "memory.h"
#include "macros.h"
#include "debug.h"

Str str_init(const char *s, usize len) {
  Str str = {len, s};

  return str;
}

Str str_init_owned(const char *s, usize len) {
  char *dst = mp_malloc(len);
  Str str = {len, strncpy(dst, s, len)};
  return str;
}

void str_free(Str *s) { mp_free((void *)s->raw); }

bool str_eq(const Str *l, const Str *r) {
  return l->len == r->len && strncmp(l->raw, r->raw, l->len) == 0;
}

bool str_eq_raw(const Str *l, const char *r) {
  const Str r_str = str_init(r, strlen(r));
  return str_eq(l, &r_str);
}

Str array_to_str(Array *a) { return str_init((const char *)a->data, a->len); }

bool str_starts_with(const Str *l, const Str *r) {
  if (l->len < r->len) {
    return FALSE;
  }

  return strncmp(l->raw, r->raw, r->len) == 0;
}

bool str_starts_with_raw(const Str *l, const char *r) {
  const Str r_str = str_init(r, strlen(r));
  return str_starts_with(l, &r_str);
}

Str str_slice(Str *s, usize start, usize end) {
  end = MIN(end, s->len);
  start = MIN(start, s->len);

  debug_assert(start <= end);

  Str result = *s;
  result.raw = result.raw + start;
  result.len = end - start;

  return result;
}
