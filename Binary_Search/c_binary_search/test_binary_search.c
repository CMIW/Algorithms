#include <stdio.h>

#include "binary_search.h"

int main(void) {
    int array[] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17};
    size_t len = sizeof(array) / sizeof(array[0]);
    int failures = 0;

    /* Checking for a value that is in the array */
    int got = binary_search(array, len, 14);
    if (got != 14) {
        printf("FAIL expected index 14, got %d\n", got);
        failures++;
    } else {
        printf("PASS: found index 14\n");
    }

    /* Checking for a value that isn't in the array */
    got = binary_search(array, len, 54);
    if (got != -1) {
        printf("FAIL expected -1, got %d\n", got);
        failures++;
    } else {
        printf("PASS: 54 not found -> -1\n");
    }

    if (failures > 0) {
        printf("%d test(s) failed\n", failures);
        return 1;
    }
    printf("all tests passed\n");
    return 0;
}