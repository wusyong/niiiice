LDFLAGS = -L. -l:target/debug/libniiiice.so

OBJS = main.o

all: main

%.o: %.c
	$(CC) -c -o $@ $<

main: $(OBJS)
	$(CC) -o main $^ $(LDFLAGS)