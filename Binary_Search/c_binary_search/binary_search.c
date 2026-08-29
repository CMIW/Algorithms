#include "binary_search.h"

int binary_search(const int a[], size_t len, int target) {
    int l = 0;
    int r = len - 1;
    while (l <= r) {
        int m = (l + r) / 2;
        if (a[m] < target) {
            l = m + 1;
        } else if (a[m] > target) {
            if (m == 0) {
                return -1;
            }
            r = m - 1;
        } else {
            return m;
        }
    }
    return -1;
}
