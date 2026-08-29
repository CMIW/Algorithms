#ifndef BINARY_SEARCH_H
#define BINARY_SEARCH_H

#include <stddef.h>

/* Returns the index of `target` in sorted array `a`, or -1 if not present. */
int binary_search(const int a[], size_t len, int target);

#endif /* BINARY_SEARCH_H */