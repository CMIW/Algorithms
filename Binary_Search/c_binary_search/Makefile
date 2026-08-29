CC      = gcc
CFLAGS  = -std=c11 -Wall -Wextra -g
TARGET  = test_binary_search
SRC     = binary_search.c test_binary_search.c

$(TARGET): $(SRC) binary_search.h
	$(CC) $(CFLAGS) -o $@ $(SRC)

test: $(TARGET)
	./$(TARGET)

clean:
	rm -f $(TARGET)

.PHONY: test clean