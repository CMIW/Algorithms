#ifndef SPARSE_SET_H
#define SPARSE_SET_H

#include <stddef.h>

/*
 * Sparse set over integer ids with int values.
 *   sparse[id] = index into dense (or -1 if absent)
 *   dense  = pack of live ids, in insertion order
 *   payload = value for the id in the same dense slot
 */
typedef struct {
    int *sparse;
    int *dense;
    int *payload;
    size_t sparse_len;
    size_t dense_len;
    size_t dense_cap;
    size_t payload_len;
    size_t payload_cap;
} sparse_set_t;

void sparse_set_init(sparse_set_t *s);
void sparse_set_free(sparse_set_t *s);

/* Insert id with value, growing sparse when needed. 0 on success, -1 on error. */
int sparse_set_insert(sparse_set_t *s, int id, int value);

/* Output the removed value into *out. 0 if removed, -1 if id absent. */
int sparse_set_remove(sparse_set_t *s, int id, int *out);

/* 1 if present, 0 if not. */
int sparse_set_contains(const sparse_set_t *s, int id);

#endif /* SPARSE_SET_H */
