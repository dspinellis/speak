CFLAGS=-Werror -g -O0 -DDEBUG

all: speak speak.m

speak: speak.c
	$(CC) $(CFLAGS) -o $@ $?

speak.m: speak.v speak
	(cat speak.v; echo !w speak.m) | ./speak -v /dev/null

# VOTRAX phonemes
p:
	mkdir p
	git clone https://github.com/sealj553/votrax-speak.git
	cd votrax-speak/samples
	for i in * ; do sox $$i ../../p/$i silence 1 0.05 0.7% 1 0.05 0.7% ; done
