from lib import if4
import leap

leap.leap_year(5)


def test_leap_100_years(benchmark):
    def leap_100_years():
        for i in range(100):
            if4.leap_year(1967 + i)

    benchmark(leap_100_years)
