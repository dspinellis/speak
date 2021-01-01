#!/bin/sed -f
#
# Convert speak(6) output into VOTRAX phonetic codes, one per line
#

# Protect all characters to avoid double replacements
s/./&/g

# Generated mostly from the comments in speak.c
# Note that phonemes have the high bit set 
s/\\o133/AH\n/g
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
s/\o103/IE\n/g
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
s/\o134/UH3\n/g
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
s/\o156,/SH\n/g
s/\o170/ZH\n/g
s/\o145/J\n/g
s/\o157/CH\n/g
s/\o106/TH\n/g
s/\o107/THV\n/g
s/\o153/NG\n/g
s/\o101/PA1\n/g
s/\o174/PA0\n/g
