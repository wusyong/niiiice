LDFLAGS = -L. -l:target/debug/libniiiice.so

OBJS = main.o

all: main

%.o: %.c
	$(CC) -c -o $@ $<

main: $(OBJS)
	$(CC) -o main $^ $(LDFLAGS)

clean:
	$(RM) *~ *.o main

ifeq ($(PREFIX),)
    PREFIX := /usr/local
endif

install:
	install -d $(DESTDIR)$(PREFIX)/lib/
	install -m 644 target/debug/libniiiice.so $(DESTDIR)$(PREFIX)/lib/
