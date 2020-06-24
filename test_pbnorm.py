#!/usr/bin/env python3

import os
import subprocess
import unittest
from unittest import TestCase

from pbnorm import pbnorm

class Test(TestCase):
    def setUp(self):
        for file in (
            "PB_Kontoauszug_KtoNr_1234567890_06-01-2020_043030.pdf",
            "PB_Kontoauszug_KtoNr_1111111111_06-02-2020_043030.pdf",
            "PB_Kontoauszug_KtoNr_2222222222_06-03-2020_043030.pdf",
            "PB_Kontoauszug_KtoNr_3333333333_06-04-2020_043030.pdf",
            "PB_Kontoauszug_KtoNr_4444444444_06-05-2020_043030.pdf",
        ):
            open(file, "w").close()
           
    def tearDown(self):
        for file in os.listdir("."):
            if file.startswith("PB_Kontoauszug_KtoNr_") or file.startswith("Postbank-"):
                if file.endswith(".pdf"):
                    os.remove(file)

    def test_pbnorm(self):
        pbnorm()
        for result_file in (
            "Postbank-2020-01-06.pdf",
            "Postbank-2020-02-06.pdf",
            "Postbank-2020-03-06.pdf",
            "Postbank-2020-04-06.pdf",
            "Postbank-2020-05-06.pdf",
        ):
            self.assertTrue(os.path.isfile(result_file), "Expected result file {} not found".format(result_file))
        

if __name__ == "__main__":
    unittest.main()
