from __future__ import annotations

import os
from typing import Any

from mypyc.build import mypycify  # type: ignore
from setuptools_rust import RustExtension, Binding

mypyc_paths = [
    "pycooldown/__init__.py",
    "pycooldown/fixed_mapping.py",
    "pycooldown/flexible_mapping.py",
    # Disabled and re-exports rust file
    # "pycooldown/sliding_window.py",
]


def build(setup_kwargs: dict[str, Any]) -> None:
    # Don't build wheels in CI.
    if os.environ.get("CI", False):
        return
    setup_kwargs["ext_modules"] = mypycify(mypyc_paths)
    setup_kwargs["rust_extensions"] = [
        RustExtension(
            "pycooldown._rust_bindings",
            binding=Binding.PyO3,
            # `--release` used so speed can be tested.
            debug=False,
        )
    ]
    setup_kwargs["zip_safe"] = False
