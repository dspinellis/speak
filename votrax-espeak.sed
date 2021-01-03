#!/usr/bin/env sed -f
#
# Convert speak(6) output into espeak phonemes
# It is assumed that input has level-2 and level-2 inflection (0100, 0200).
#

# Protect all characters to avoid double replacements
# For this to work sed must be invoked with LC_ALL=C
s/[\d001-\d255]/&/g

# Generated mostly from the comments in speak.c

# Map inflection to 'uninflected
s/\o233/'\o133/g
s/\o252/'\o152/g
s/\o267/'\o167/g
s/\o202/'\o102/g
s/\o254/'\o154/g
s/\o217/'\o117/g
s/\o221/'\o121/g
s/\o220/'\o120/g
s/\o237/'\o137/g
s/\o271/'\o171/g
s/\o272/'\o172/g
s/\o204/'\o104/g
s/\o275/'\o175/g
s/\o276/'\o176/g
s/\o277/'\o177/g
s/\o205/'\o105/g
s/\o223/'\o123/g
s/\o226/'\o126/g
s/\o235/'\o135/g
s/\o203/'\o103/g
s/\o236/'\o136/g
s/\o230/'\o130/g
s/\o264/'\o164/g
s/\o265/'\o165/g
s/\o266/'\o166/g
s/\o231/'\o131/g
s/\o212/'\o112/g
s/\o213/'\o113/g
s/\o251/'\o151/g
s/\o211/'\o111/g
s/\o250/'\o150/g
s/\o214/'\o114/g
s/\o215/'\o115/g
s/\o216/'\o116/g
s/\o234/'\\/g
s/\o227/'\o127/g
s/\o210/'\o110/g
s/\o261/'\o161/g
s/\o241/'\o141/g
s/\o273/'\o173/g
s/\o242/'\o142/g
s/\o243/'\o143/g
s/\o244/'\o144/g
s/\o246/'\o146/g
s/\o247/'\o147/g
s/\o263/'\o163/g
s/\o262/'\o162/g
s/\o232/'\o132/g
s/\o224/'\o124/g
s/\o240/'\o140/g
s/\o225/'\o125/g
s/\o260/'\o160/g
s/\o222/'\o122/g
s/\o255/'\o155/g
s/\o256/'\o156/g
s/\o270/'\o170/g
s/\o245/'\o145/g
s/\o257/'\o157/g
s/\o206/'\o106/g
s/\o207/'\o107/g
s/\o253/'\o153/g
s/\o201/'\o101/g
s/\o274/'\o174/g

# \o133
s/\[/A@/g
s/\o152/a#/g
s/\o167/e/g
s/\o102/O:/g
s/\o154/O:/g
s/\o117/a/g
s/\o121/a/g
s/\o120/a/g
s/\o137/a#/g
s/\o171/a#/g
s/\o172/a#/g
s/\o104/E/g
s/\o175/E/g
s/\o176/E/g
s/\o177/E/g
s/\o105/3/g
s/\o123/i:/g
s/\o126/I/g
s/\o135/j/g
s/\o103/i:/g
s/\o136/eI/g
s/\o130/I/g
s/\o164/I2/g
s/\o165/I/g
s/\o166/I/g
s/\o131/oU/g
s/\o112/oU/g
s/\o113/0/g
s/\o151/u:/g
s/\o111/U@/g
s/\o150/U/g
s/\o114/V/g
s/\o115/V/g
s/\o116/V/g
# \o134
s/\\/@L/g
s/\o127/ju:/g
s/\o110/ju:/g
s/\o161/b/g
s/\o141/d/g
s/\o173/Y/g
s/\o142/f/g
s/\o143/g/g
s/\o144/h/g
s/\o146/k/g
s/\o147/l/g
s/\o163/m/g
s/\o162/n/g
s/\o132/p/g
s/\o124/r/g
s/\o140/s/g
s/\o125/t/g
s/\o160/v/g
s/\o122/w/g
s/\o155/z/g
s/\o156/S/g
s/\o170/Z/g
s/\o145/dZ/g
s/\o157/tS/g
s/\o106/T/g
s/\o107/D/g
s/\o153/N/g
s/\o101/  /g
s/\o174/ /g

# Mark the output as espeak phonemes
s/^/[[/
s/$/]]/
