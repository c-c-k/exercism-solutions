import pytest

from . import solutions

pytestmark = pytest.mark.parametrize(
    "solution",
    solutions.get_solution_modules(),
    ids=lambda module: module.__name__.split(".")[1],
)


@pytest.mark.benchmark(
    # group="group-name",  # noqa: ERA001
    # min_time=0.1,  # noqa: ERA001
    max_time=0.5,
    min_rounds=1,
    # timer=time.time,  # noqa: ERA001
    # disable_gc=True,  # noqa: ERA001
    # warmup=False  # noqa: ERA001
)
def test_leap_2k_years(solution, benchmark):
    def leap_2k_years():
        for year in range(1, 2200):
            solution.leap_year(year)

    benchmark(leap_2k_years)
