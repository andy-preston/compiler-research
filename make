#!/bin/bash

for FILE in bin elf hex o ppu res s
do
    find -name \*.$FILE -exec rm {} \;
done

if [ "$1" != "clean" ]
then
    FPC_OPTIONS="-Parm -Tembedded -WpSTM32F401CC -Cparmv7em -XParm-none-eabi- \
        -s -l -a -viwn -al -Xs -O2 -Sgi
        -Fu$(pwd)/src/ \
        -Fu$(pwd)/src/stm32/ \
        -Fu/usr/local/lib/fpc/3.3.1/units/arm-embedded/rtl/"

    fpc $FPC_OPTIONS $1
    echo hack $(echo $1 | sed -e 's/\.pp$/.s/g')
    ./ppas.sh
    rm ./ppas.sh
    arm-none-eabi-objdump -d -s $(echo $1 | sed -e 's/\.pp$/.elf/g')
fi
