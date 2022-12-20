#ifndef STR_H_
#define STR_H_ 

#include "array.h"
#include "types.h"

#define str_fmt "%.*s"
#define str_out(str) (int)(str).len, (str).raw

typedef struct Str {
  usize len;
  const char *raw;
  
  Error err;
} Str;

Str str_init(const char *s, usize len);

Str str_init_owned(const char *s, usize len);

// frees an owned str 
void str_free(Str *s);

bool str_eq_raw(const Str *l, const char *r);
bool str_eq(const Str *l, const Str *r);

bool str_starts_with(const Str *l, const Str *r);
bool str_starts_with_raw(const Str *l, const char *r);

// converts an array to an owned string 
Str array_to_str(Array *a);

Str str_slice(Str *s, usize start, usize end);

#endif 
