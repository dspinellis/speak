# Revival of the 1973 Unix speak program
This repository contains the source code of Unix _speak_
program that appeared in the Third (1973) to Sixth (1975) Research
Unix editions, slightly adjusted to run on a modern computer.
Details on the code's provenance and the methods employed for
reviving it can be found in
[this blog post](https://www.spinellis.gr/blog/20210102/).

## Repository contents
### 1970s code
* `speak.c`: Original source code adjusted to run on modern computers;
   [commits](https://github.com/dspinellis/speak/commits/master/speak.c) detail
   the modifications made
* `speak.v`: Speech rules
### Modern additions
* `Makefile`: Code and rule compilation
* [votrax-espeak.md](https://github.com/dspinellis/speak/blob/master/votrax-espeak.md): Map between Votrax, speak, and espeak phoneme codes
* `votrax-mnemonics.sed`: Convert _speak_ output into Votrax mnemonics to help debugging
* `votrax-espeak.sed`: Convert _speak_ output into
   [espeak-ng](https://github.com/espeak-ng/espeak-ng) phonemes
* [Caldera-license.pdf](https://github.com/dspinellis/speak/blob/master/Caldera-license.pdf): Early Unix source code license terms

## Build
Run `make` on a system with a C compiler and _make_.

## Run
On a system containing the
[espeak](https://github.com/espeak-ng/espeak-ng) command, run:

```sh
echo Hello world |
speak speak.m |
LC_ALL=C ./votrax-espeak.sed |
espeak
```

## Contribute
Improvements to the [phoneme map](https://github.com/dspinellis/speak/blob/master/votrax-espeak.md)
as well as bug fixes that retain the historical accuracy of the _speak_ and rules source code are welcomed
through GitHub pull requests.
