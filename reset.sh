#!/bin/bash

rm 2> /dev/null *.pdf
if [ "$1" != "clean" ]; then
    touch PB_Kontoauszug_KtoNr_1234567890_06-01-2020_043030.pdf
    touch PB_Kontoauszug_KtoNr_1111111111_06-02-2020_043030.pdf
    touch PB_Kontoauszug_KtoNr_2222222222_06-03-2020_043030.pdf
    touch PB_Kontoauszug_KtoNr_3333333333_06-04-2020_043030.pdf
    touch PB_Kontoauszug_KtoNr_4444444444_06-05-2020_043030.pdf
fi
