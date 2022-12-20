#include "array.h"
#include "memory.h"
#include <stdio.h>
#include <string.h>
#include "debug.h"

#define ARRAY_DEFAULT_LEN 64

#define array_calc_index(a, i) ((a)->stride * (i))
#define array_in_bounds(a, i) ((i) < (a)->len)

Array array_init(usize stride) {
  Array a;
  memset(&a, 0, sizeof(a));
  a.stride = stride;
  a.len = 0;
  a.max_len = ARRAY_DEFAULT_LEN;
  a.data = mp_malloc(stride * a.max_len);
  a.allow_resize = TRUE;
  return a;
}

Error array_get(Array *a, usize index, void *data) {
  // bail early if the data pointer is NULL
  if (!data) {
    return OK;
  }

  if (!array_in_bounds(a, index)) {
    return ERR_ARRAY_OUT_OF_BOUNDS;
  }
  memcpy(data, a->data + array_calc_index(a, index), a->stride);
  return OK;
}

Error array_get_ptr(Array *a, usize index, void **data) {
  // bail early if the data pointer is NULL
  if (!data) {
    return OK;
  }

  if (!array_in_bounds(a, index)) {
    return ERR_ARRAY_OUT_OF_BOUNDS;
  }
  *data = a->data + array_calc_index(a, index);
  return OK;
}

Error array_insert(Array *a, usize index, void *data) {
  if (!array_in_bounds(a, index)) {
    return ERR_ARRAY_OUT_OF_BOUNDS;
  }
  memcpy(a->data + array_calc_index(a, index), data, a->stride);
  return OK;
}

Error array_remove(Array *a, usize index, void *data) {
  if (!array_in_bounds(a, index)) {
    return ERR_ARRAY_OUT_OF_BOUNDS;
  }

  // if it is the last index, simply pop
  if (index == a->len - 1) {
    return array_pop(a, data);
  }
  // otherwise

  // get pointer of the object to be removed
  // and move everything after it
  // by one stride
  Error err = OK;

  u8 *to_remove = NULL;
  err = array_get(a, index, data);
  if (err) {
    return err;
  }
  err = array_get_ptr(a, index, (void **)&to_remove);
  if (err) {
    return err;
  }

  // pointer after the item to be removed
  u8 *after_remove = NULL;
  err = array_get_ptr(a, index + 1, (void **)&after_remove);
  if (err) {
    return err;
  }

  // pointer to last item
  u8 *last_item = NULL;
  err = array_get_ptr(a, a->len - 1, (void **)&last_item);
  if (err) {
    return err;
  }

  // move
  memcpy(to_remove, after_remove, last_item - after_remove);

  // lastly modify length
  a->len--;

  return err;
}

Error array_resize(Array *a) {
  // if required, resize
  if (a->len <= a->max_len) {
    return OK;
  } else if (!a->allow_resize) {
    // if resize is not allowed, error out
    return ERR_ARRAY_RESIZE_NOT_ALLOWED;
  }
  // store old location
  u8 *old_data = a->data;
  usize old_max_len = a->max_len;

  // do a resize. simply double the current amount!
  a->max_len *= 2;
  a->data = mp_malloc(a->max_len * a->stride);

  // copy old data to new location
  memcpy(a->data, old_data, old_max_len * a->stride);

  // free old data
  mp_free(old_data);

  return OK;
}

Error array_push(Array *a, void *data) {
  a->len++;
  // make sure to resize on push
  // if we are out of memory
  Error err = array_resize(a);
  if (err) {
    a->len--;
    return err;
  }

  return array_insert(a, a->len - 1, data);
}

Error array_pop(Array *a, void *data) {
  if (a->len <= 0) {
    return ERR_ARRAY_OUT_OF_BOUNDS;
  }
  array_get(a, a->len - 1, data);
  a->len--;

  return OK;
}

void array_free(Array *a) { mp_free(a->data); }

#ifdef TEST

#include "test.h"

void test_array(void **state) {
  Array a = array_init(sizeof(usize));

  // test regular insert and get
  for (usize i = 0; i < a.max_len; i++) {
    assert_int_equal(OK, array_push(&a, &i));
    usize g = 0;
    assert_int_equal(OK, array_get(&a, i, &g));
    assert_int_equal(i, g);
  }
  assert_int_equal(a.max_len, a.len);

  // test get ptr
  {
    usize *gp = NULL;
    assert_int_equal(OK, array_get_ptr(&a, 1, (void **)&gp));
    assert_ptr_equal(a.data + a.stride, gp);
  }
  // test get out of bounds
  {
    usize g = 0;
    usize *gp = NULL;
    assert_int_equal(ERR_ARRAY_OUT_OF_BOUNDS, array_get(&a, a.len, &g));
    assert_int_equal(ERR_ARRAY_OUT_OF_BOUNDS,
                     array_get_ptr(&a, a.len, (void **)&gp));
  }
  // test push no resize allowed
  {
    a.allow_resize = FALSE;

    usize g = 0;
    usize prev_len = a.len;
    u8 *prev_data = a.data;
    assert_int_equal(ERR_ARRAY_RESIZE_NOT_ALLOWED, array_push(&a, &g));
    assert_int_equal(prev_len, a.len);
    assert_ptr_equal(prev_data, a.data);

    a.allow_resize = TRUE;
  }

  // test push resize
  {
    usize g = 64;
    usize gr = 0;
    array_push(&a, &g);
    assert_int_not_equal(ARRAY_DEFAULT_LEN, a.max_len);
    array_get(&a, a.len - 1, &gr);
    assert_int_equal(64, gr);
  }

  // test insert
  {
    usize g = 9001;
    usize gr = 0;
    assert_int_equal(OK, array_insert(&a, 1, &g));
    assert_int_equal(OK, array_get(&a, 1, &gr));
    assert_int_equal(9001, gr);
    g = 1;
    assert_int_equal(OK, array_insert(&a, 1, &g));
  }

  // test pop
  {
    usize g = 0;
    for (usize i = a.len; i > 0; i--) {
      assert_int_equal(OK, array_pop(&a, &g));
      assert_int_equal(i - 1, g);
    }
    assert_int_equal(0, a.len);
    // should not work now!
    assert_int_equal(ERR_ARRAY_OUT_OF_BOUNDS, array_pop(&a, &g));

    // restore data
    for (usize i = 0; i < 65; i++) {
      array_push(&a, &i);
    }
  }

  // test remove
  {
    usize g = 0;
    usize original_len = a.len;
    // remove out of bounds
    assert_int_equal(ERR_ARRAY_OUT_OF_BOUNDS, array_remove(&a, a.len + 1, &g));

    // remove last item
    assert_int_equal(OK, array_remove(&a, a.len - 1, &g));
    assert_int_equal(64, g);
    // remove first item
    assert_int_equal(OK, array_remove(&a, 0, &g));
    assert_int_equal(0, g);

    // remove middle item
    assert_int_equal(OK, array_remove(&a, 10, &g));
    assert_int_equal(11, g);

    assert_int_equal(original_len - 3, a.len);

    usize expected[14] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 15};
    // test values
    for (usize i = 0; i < 14; i++) {
      array_get(&a, i, &g);
      assert_int_equal(expected[i], g);
    }
  }
  array_free(&a);
}

#endif
