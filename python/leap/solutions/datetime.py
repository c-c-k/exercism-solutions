"""datetime

Evaluate leap year logic according to if the day after February 28'th
is February 29'th.
"""

from datetime import date, timedelta


def leap_year(year):
    return (date(year, 2, 28) + timedelta(days=1)).day == 29
