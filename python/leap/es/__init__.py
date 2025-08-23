"""ES (Exercise Solutions).

The es package is meant to serve as a container for different solutions
for an Exercism exercise.
The code in this file is meant to provide utilities for conveniently
accessing the solutions instead of the top level solution file which
should function as a documentation stub and a wrapper around the
currently worked on solution for the exercism test file.
"""

from importlib import import_module
from pathlib import Path


def get_solution_paths() -> tuple[Path, ...]:
    basedir = Path(__file__).parent
    paths = tuple(
        path
        for path in basedir.glob("[!_]*")
        if (path.is_file() and path.suffix == ".py")
        or (path.is_dir() and path.joinpath("__init__.py").exists())
    )

    if len(paths) == 0:
        # TODO: Add custom exception
        raise Exception(f"No solutions found in: {basedir}/")

    return paths


def module_name_from_path(path: Path) -> str:
    return f"{__name__}.{path.stem}"


def get_solution_names() -> tuple[str, ...]:
    return tuple(module_name_from_path(path) for path in get_solution_paths())


def get_latest_solution_path() -> Path:
    latest_mtime: float = 0
    latest_path: Path | None = None

    for path in get_solution_paths():
        if latest_path is None:
            latest_path = path
            latest_mtime = path.stat().st_mtime
        elif (
            path.is_file() and (mtime := path.stat().st_mtime) > latest_mtime
        ):
            latest_path = path
            latest_mtime = mtime
        else:
            for filepath in path.glob("**/*"):
                if (mtime := filepath.stat().st_mtime) > latest_mtime:
                    latest_path = path
                    latest_mtime = mtime

    if latest_path is None:
        # TODO: add custom exception
        raise Exception("Latest solution not found.")

    return latest_path


def importall_from_latest(module_name: str):
    """Import all names from the most recently modified solution.

    Import all public names from the most recently modified solution
    in the `es` package into the namespace of the module specified by
    `module_name`, which is intended to be the module imported by the
    exercise test file.
    """
    solution_path = get_latest_solution_path()
    solution_module_name = module_name_from_path(solution_path)
    solution_module = import_module(solution_module_name)
    names = (
        name for name in solution_module.__dict__ if not name.startswith("_")
    )
    module = import_module(module_name)
    for name in names:
        setattr(module, name, getattr(solution_module, name))
