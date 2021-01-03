# rspeak

This is an experimental Rust port of the 1973 Unix speak program.

It may differ in function from the original C version.

## Tested with

* macOS 10.15.4 (Intel), rustc 1.51.0-nightly (fde692739 2021-01-02) (from rustup), espeak 1.48.03 04.Mar.14 (from homebrew)

## Setup

Install [Rust](https://www.rust-lang.org/) using version specified above or another compatible version. As of 2020-01-03, the nightly version of Rust was required.

Install [espeak](http://espeak.sourceforge.net/).

## Build

In the root directory of this project:
```
rustc rs/rspeak.rs
```

## Usage

### votrax-espeak

```
echo greetings-professor-falken. \
... ... ... ... ... \
a strange-game... \
... ... \
the-only-winning-move-is - \
not-to-play... \
... ... ... ... ... \
... ... ... ... ... \
how-about-a-nice-game-of-chess |
rs rspeak |
LC_ALL=C ./votrax-espeak.sed |
espeak -s 120
```

or outputting to mp3 using [ffmpeg](https://en.wikipedia.org/wiki/FFmpeg):


```
echo greetings-professor-falken. \
... ... ... ... ... \
a strange-game... \
... ... \
the-only-winning-move-is - \
not-to-play... \
... ... ... ... ... \
... ... ... ... ... \
how-about-a-nice-game-of-chess |
rs rspeak |
LC_ALL=C ./votrax-espeak.sed |
espeak -s 120 --stdout |
ffmpeg -i - -ar 44100 -ac 2 -ab 192k -f mp3 rspeak_votrax-espeak.mp3
```

### votrax-mnemonics

```
echo greetings-professor-falken. \
... ... ... ... ... \
a strange-game... \
... ... \
the-only-winning-move-is - \
not-to-play... \
... ... ... ... ... \
... ... ... ... ... \
how-about-a-nice-game-of-chess |
rs rspeak |
LC_ALL=C ./votrax-mnemonics.sed |
espeak -s 120
```

or outputting to mp3 using [ffmpeg](https://en.wikipedia.org/wiki/FFmpeg):


```
echo greetings-professor-falken. \
... ... ... ... ... \
a strange-game... \
... ... \
the-only-winning-move-is - \
not-to-play... \
... ... ... ... ... \
... ... ... ... ... \
how-about-a-nice-game-of-chess |
rs rspeak |
LC_ALL=C ./votrax-mnemonics.sed |
espeak -s 120 --stdout |
ffmpeg -i - -ar 44100 -ac 2 -ab 192k -f mp3 rspeak_votrax-mneumonics.mp3
```

## How it was ported

### 2021-01-03

Translated speak.c (sha 7c7583098f5bad4ac801e084d48811a077406f62) to rspeak.rs using [C2Rust](https://c2rust.com/).

Then made the following changes to the resulting rspeak.rs:

* Replaced "libc::" with "std::os::raw::".
* Replaced "wrapping_offset_from" with "offset_from"
