#include "sparse_set.h"

#include <stdlib.h>

// Initialize sparse set struct members to empty/NULL.
void sparse_set_init(sparse_set_t *s) {
    s->sparse = NULL;
    s->dense = NULL;
    s->payload = NULL;
    s->sparse_len = 0;
    s->dense_len = 0;
    s->dense_cap = 0;
    s->payload_len = 0;
    s->payload_cap = 0;
}

// Free allocated heap memory for all three arrays.
void sparse_set_free(sparse_set_t *s) {
    free(s->sparse);
    free(s->dense);
    free(s->payload);
    sparse_set_init(s);
}

// Dynamic array append helper using doubling capacity strategy (amortized O(1)).
void push(int **arr, size_t *len, size_t *cap, int value) {
    if (*len == *cap) {
        *cap = *cap ? *cap * 2 : 4;
        *arr = realloc(*arr, *cap * sizeof(int));
    }
    if (*arr != NULL) {
        (*arr)[(*len)++] = value;
    }
}

// Swaps two integer values via pointers.
static void swap(int *a, int *b) {
    int temp = *a;
    *a = *b;
    *b = temp;
}

// Inserts (id, value) pair into the sparse set in O(1) amortized time.
int sparse_set_insert(sparse_set_t *s, int id, int value) {
    // If id exceeds current sparse array bound, grow sparse to fit id+1 slots
    if (id >= (int)s->sparse_len) {
        int *grown = realloc(s->sparse, (size_t)(id + 1) * sizeof *grown);
        if (grown == NULL) {
            return -1; // Allocation failed; original set unmodified
        }
        // Initialize newly allocated sparse slots to sentinel -1 (absent)
        for (int k = s->sparse_len; k <= id; k++) {
            grown[k] = -1;
        }
        s->sparse = grown;
        s->sparse_len = (size_t)id + 1;
    }

    // Append id to dense and value to payload at index `dense_len`
    push(&s->dense, &s->dense_len, &s->dense_cap, id);
    push(&s->payload, &s->payload_len, &s->payload_cap, value);

    // Map sparse[id] to point to the element's position in dense/payload
    s->sparse[id] = s->dense_len - 1;
    return 0;
}

// Removes `id` in O(1) time, storing its payload in *out if found.
int sparse_set_remove(sparse_set_t *s, int id, int *out) {
    // Check if id is within bounds and currently present in the set
    if (id >= 0 && id < (int)s->sparse_len && s->sparse[id] != -1) {
        int dense_idx = s->sparse[id];
        int last = s->dense_len - 1;

        // Swap target element with the last element in dense and payload
        swap(&s->dense[dense_idx], &s->dense[last]);
        swap(&s->payload[dense_idx], &s->payload[last]);

        // Fixup sparse map for the element that was moved into dense_idx
        int moved_id = s->dense[dense_idx];
        s->sparse[moved_id] = dense_idx;

        // Unset sparse[id] sentinel and copy payload to out
        s->sparse[id] = -1;
        *out = s->payload[last];

        // Shrink dense and payload length (pop)
        s->dense_len--;
        s->payload_len--;
        return 0;
    }
    return -1; // ID not present
}

// Returns 1 if `id` is present in the sparse set, 0 otherwise.
int sparse_set_contains(const sparse_set_t *s, int id) {
    return id >= 0 && id < (int)s->sparse_len && s->sparse[id] != -1;
}
