CFLAGS=-Werror -g -O0 -DDEBUG

all: speak speak.m

speak: speak.c
	$(CC) $(CFLAGS) -o $@ $?

speak.m: speak.v speak
	(cat speak.v; echo !w speak.m) | ./speak -v /dev/null
