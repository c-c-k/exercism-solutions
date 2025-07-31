# The following code is meant to load the most recently modified
# exercise solution as the solution to test.
# See __init__.pyi for stubs, documentation and annotations

import importlib.util
import os
import sys
from glob import glob

basedir = os.path.dirname(__file__)
submodule_direntry: os.DirEntry | None = None
max_mtime: float = 0

for entry in os.scandir(basedir):
    if entry.name.startswith("__"):
        continue

    if entry.is_file():
        if not entry.name.endswith(".py"):
            continue
        if submodule_direntry is None:
            submodule_direntry = entry
            max_mtime = entry.stat().st_mtime

        if (mtime := entry.stat().st_mtime) > max_mtime:
            submodule_direntry = entry
            max_mtime = mtime

    if entry.is_dir():
        dirpath = entry.path
        if not os.path.exists(os.path.join(dirpath, "__init__.py")):
            continue
        if submodule_direntry is None:
            submodule_direntry = entry
            max_mtime = entry.stat().st_mtime

        for filepath in glob("**", root_dir=dirpath, recursive=True):
            if (
                mtime := os.stat(os.path.join(dirpath, filepath)).st_mtime
            ) > max_mtime:
                submodule_direntry = entry
                max_mtime = mtime

if submodule_direntry is None:
    raise ImportError(f"No modules found in: {basedir}/")

submodule_name = f"{__name__}.{submodule_direntry.name.replace('.py', '')}"
spec = importlib.util.find_spec(submodule_name)

if spec is None:
    raise ImportError(f"Can't find spec for: {submodule_name}")
if spec.loader is None:
    raise ImportError(f"Spec {spec.name} has no loader")

module = importlib.util.module_from_spec(spec)
sys.modules[__name__] = module
sys.modules[submodule_name] = module
spec.loader.exec_module(module)
