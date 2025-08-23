"""ternary

Consolidate all leap year rules into a single ternary expression.
Start evaluation from the less common 100'th year rule.
"""


def leap_year(year):
    return year % 400 == 0 if year % 100 == 0 else year % 4 == 0
