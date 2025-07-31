"""if100

Evaluate leap year via a single early exit if.
Start evaluation from the less common 100'th year rule.
"""


def leap_year(year: int) -> bool:
    if year % 100 == 0:
        return year % 400 == 0
    return year % 4 == 0
