#include <stdio.h>

#include "sparse_set.h"

#define CHECK(cond, msg)                  \
    do {                                  \
        if (!(cond)) {                    \
            printf("FAIL: %s\n", msg);    \
            failures++;                   \
        } else {                          \
            printf("PASS: %s\n", msg);    \
        }                                 \
    } while (0)

int main(void) {
    sparse_set_t set;
    int failures;

    /* Basic insert + contains */
    failures = 0;
    sparse_set_init(&set);
    CHECK(sparse_set_insert(&set, 2, 20) == 0, "insert 2");
    CHECK(sparse_set_insert(&set, 5, 50) == 0, "insert 5");
    CHECK(sparse_set_insert(&set, 7, 70) == 0, "insert 7");
    CHECK(sparse_set_contains(&set, 2), "contains 2");
    CHECK(sparse_set_contains(&set, 5), "contains 5");
    CHECK(sparse_set_contains(&set, 7), "contains 7");
    CHECK(!sparse_set_contains(&set, 3), "not contains 3");
    CHECK(!sparse_set_contains(&set, 100), "not contains 100 (sparse out of range)");
    sparse_set_free(&set);
    if (failures > 0) return 1;

    /* Remove the middle element: exercises the swap + sparse fixup */
    failures = 0;
    sparse_set_init(&set);
    sparse_set_insert(&set, 2, 20);
    sparse_set_insert(&set, 5, 50);
    sparse_set_insert(&set, 7, 70);
    {
        int out = -1;
        CHECK(sparse_set_remove(&set, 5, &out) == 0, "remove 5 present");
        CHECK(out == 50, "remove returns value 50");
        CHECK(!sparse_set_contains(&set, 5), "5 gone after remove");
        CHECK(sparse_set_contains(&set, 2), "2 still present");
        CHECK(sparse_set_contains(&set, 7), "7 still present (moved/last swap)");
        CHECK(sparse_set_remove(&set, 5, &out) == -1, "remove 5 twice fails");
    }
    sparse_set_free(&set);
    if (failures > 0) return 1;

    /* Remove everything, ending empty */
    failures = 0;
    sparse_set_init(&set);
    sparse_set_insert(&set, 0, 1);
    sparse_set_insert(&set, 1, 2);
    {
        int out = -1;
        CHECK(sparse_set_remove(&set, 0, &out) == 0 && out == 1, "remove 0 -> 1");
        CHECK(sparse_set_remove(&set, 1, &out) == 0 && out == 2, "remove 1 -> 2");
        CHECK(!sparse_set_contains(&set, 0), "empty, 0 gone");
        CHECK(!sparse_set_contains(&set, 1), "empty, 1 gone");
    }
    sparse_set_free(&set);
    if (failures > 0) return 1;

    printf("all tests passed\n");
    return 0;
}