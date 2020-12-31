all: speak speak.m

speak: speak.c
	$(CC) -w -o $@ $?

speak.m: speak.v speak
	(cat speak.v; echo !w speak.m) | ./speak -v /dev/null
