#!/usr/bin/env sed -f
#
# Convert speak(6) output into VOTRAX phonetic code mnemonics, one per line.
# It is assumed that input has level-2 and level-2 inflection (0100, 0200).
# Inflection is marked with '
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

# Map VOTRAX codes to VOTRAX mnemonics
# \133
s/\[/AH\n/g
s/\o152/AH1\n/g
s/\o167/AH2\n/g
s/\o102/AW\n/g
s/\o154/AW1\n/g
s/\o117/AW2\n/g
s/\o121/AE\n/g
s/\o120/EA1\n/g
s/\o137/A\n/g
s/\o171/A1\n/g
s/\o172/A2\n/g
s/\o104/EH\n/g
s/\o175/EH1\n/g
s/\o176/EH2\n/g
s/\o177/EH3\n/g
s/\o105/ER\n/g
s/\o123/E\n/g
s/\o126/Y\n/g
s/\o135/Y1\n/g
# No sample for IE use I2
s/\o103/I2\n/g
s/\o136/AY\n/g
s/\o130/I\n/g
s/\o164/I1\n/g
s/\o165/I2\n/g
s/\o166/I3\n/g
s/\o131/O\n/g
s/\o112/O1\n/g
s/\o113/O2\n/g
s/\o151/OO1\n/g
s/\o111/IU\n/g
s/\o150/OO\n/g
s/\o114/UH\n/g
s/\o115/UH1\n/g
s/\o116/UH2\n/g
# \o134
s/\\/UH3\n/g
s/\o127/U\n/g
s/\o110/U1\n/g
s/\o161/B\n/g
s/\o141/D\n/g
s/\o173,/DT\n/g
s/\o142/F\n/g
s/\o143/G\n/g
s/\o144/H\n/g
s/\o146/K\n/g
s/\o147/L\n/g
s/\o163/M\n/g
s/\o162/N\n/g
s/\o132/P\n/g
s/\o124/R\n/g
s/\o140/S\n/g
s/\o125/T\n/g
s/\o160/V\n/g
s/\o122/W\n/g
s/\o155/Z\n/g
s/\o156/SH\n/g
s/\o170/ZH\n/g
s/\o145/J\n/g
s/\o157/CH\n/g
s/\o106/TH\n/g
s/\o107/THV\n/g
s/\o153/NG\n/g
s/\o101/PA1\n/g
s/\o174/PA0\n/g
