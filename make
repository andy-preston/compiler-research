#!/bin/bash

cd pascal-test

for FILE in bin elf hex o ppu s
do
    find -name \*.$FILE -exec rm {} \;
done

[ "$1" != "clean" ] && fpc test.pp

cd ..
