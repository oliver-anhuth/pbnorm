#!/usr/bin/env python3

import os
import re

def pbnorm():
    pb = re.compile(r"^PB_Kontoauszug_KtoNr_\d{10}_(\d{2})-(\d{2})-(\d{4})_\d{6}.pdf$")
    files = [f for f in os.listdir(".") if os.path.isfile(f)]
    for f in files:
        match = pb.match(f)
        if match:
            (day, month, year) = (match.group(1), match.group(2), match.group(3))
            to = "Postbank-{}-{}-{}.pdf".format(year, month, day)
            os.rename(f, to)

if __name__ == "__main__":
    pbnorm()
