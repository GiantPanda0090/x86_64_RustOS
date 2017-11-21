#!/bin/bash
#indentation
#ask johan if it is possible to integrate it into the Makefile

sed 's/ \+ /\t/g' Makefile > Makefile2
mv Makefile2 Makefile
