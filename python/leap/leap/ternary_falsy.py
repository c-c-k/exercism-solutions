"""ternary_falsy

Consolidate all leap year rules into a single ternary expression.
Start evaluation from the less common 100'th year rule.
Use falsiness instead of comparison to 0.
"""


def leap_year(year):
    return not year % 4 if year % 100 else not year % 400
