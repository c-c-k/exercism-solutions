"""if4

Evaluate leap year via a chain of early exit if's.
Start evaluation from the more common 4'th year rule.
"""


def leap_year(year: int) -> bool:
    if year % 4 != 0:
        return False
    if year % 100 == 0 and year % 400 != 0:
        return False
    return True
