#ifndef ARRAY_H_ 
#define ARRAY_H_ 

#include "types.h"
#include "error.h"

// An array is a generic resizeable container 
// for any data type 
// the size of the type is contained in the stride lenght  
typedef struct Array {
  usize len;
  usize max_len;
  usize stride;
  u8 *data;
  
  bool allow_resize;
  Error err; 
} Array; 

Array array_init(usize stride);

// returns a clone of the data contained at index into *data 
// if data is NULL nothing is returned 
Error array_get(Array *a, usize index, void *data);

// returns a pointer to the data contained at index 
// if data is NULL nothing is returned
Error array_get_ptr(Array *a, usize index, void **data);

// insert and replace an item in the array 
Error array_insert(Array *a, usize index, void *data);

// remove an item from the array 
// the array will not actually resize during this call,
// but it will move the data to fill the gap if required
Error array_remove(Array *a, usize index, void *data);

// if array's len is greater than max_len 
// the array will reallocated 
// and copy its data to a new location
Error array_resize(Array *a);

// push a new item to the end of the array 
// this call will resize if requried
Error array_push(Array *a, void *data);

// pop the top item from the array 
// and copy it into the location of *data 
// if *data is NULL the top item is simply discarded 
Error array_pop(Array *a, void *data);

void array_free(Array *a); 

#endif 
