CC      = gcc
CFLAGS  = -std=c11 -Wall -Wextra -g
TARGET  = test_sparse_set
SRC     = sparse_set.c test_sparse_set.c

$(TARGET): $(SRC) sparse_set.h
	$(CC) $(CFLAGS) -o $@ $(SRC)

test: $(TARGET)
	./$(TARGET)

clean:
	rm -f $(TARGET)

.PHONY: test clean