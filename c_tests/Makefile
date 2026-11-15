TARGET = tests
# Set MODE to debug to build in debug mode
MODE ?= release
# The library's soname (see build.rs): the tests load it under this name, from this directory
SONAME = libetebase.so.0
LIBS = -L. -l:$(SONAME) -Wl,-rpath,'$$ORIGIN'
CC = gcc
CFLAGS = -g -Wall -I ../target

.PHONY: default all clean

default: $(TARGET)
all: default

OBJECTS = $(patsubst %.c, %.o, $(wildcard *.c))
HEADERS = $(wildcard *.h) ../target/etebase.h

%.o: %.c $(HEADERS)
	$(CC) $(CFLAGS) -c $< -o $@

.PRECIOUS: $(TARGET) $(OBJECTS)

$(SONAME): FORCE
	ln -sf ../target/$(MODE)/libetebase.so $@

$(TARGET): $(OBJECTS) $(SONAME)
	$(CC) $(OBJECTS) -Wall -fsanitize=address -fno-omit-frame-pointer $(LIBS) -o $@

check: $(TARGET)
	./tests

clean:
	-rm -f *.o
	-rm -f $(TARGET) $(SONAME)

FORCE:
