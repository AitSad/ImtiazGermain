import unittest
from primecheck import PrimeCheck

class TestPrimeCheck(unittest.TestCase):
    def setUp(self):
        self.pc = PrimeCheck()

    def test_is_prime(self):
        self.assertTrue(self.pc.is_prime(7))
        self.assertFalse(self.pc.is_prime(8))

    def test_is_germain_prime(self):
        self.assertTrue(self.pc.is_germain_prime(5))
        self.assertFalse(self.pc.is_germain_prime(7))

    def test_is_imtiaz_germain_prime(self):
        self.assertTrue(self.pc.is_imtiaz_germain_prime(3))
        self.assertFalse(self.pc.is_imtiaz_germain_prime(5))
