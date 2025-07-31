"""boolean

Consolidate all leap year rules into a single boolean expression.
Start evaluation from the more common 4'th year rule.
"""


def leap_year(year):
    return year % 4 == 0 and (year % 100 != 0 or year % 400 == 0)
