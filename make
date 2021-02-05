#!/bin/bash

cd pascal-test

for FILE in bin elf hex o ppu s
do
    find -name \*.$FILE -exec rm {} \;
done

# -Cffpv4_s16
#-Fu/usr/local/lib/fpc/3.3.1/units/arm-embedded
#-Fu/usr/local/lib/fpc/3.3.1/units/arm-embedded/*
#-Fu/usr/local/lib/fpc/3.3.1/units/arm-embedded/rtl

FPC_OPTIONS="-Parm -Tembedded -WpSTM32F401CC -Cparmv7em -XParm-none-eabi- \
    -l -a -viwn -al -Xs -O2 -Sgi -Fl."

[ "$1" != "clean" ] && fpc $FPC_OPTIONS test.pp

cd ..
