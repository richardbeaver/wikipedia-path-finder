"""
Usage:

`python titles/generate_titles.py`

- Execute after modifying `titles.json`
- Creates `titles/titles.py` and `titles/titles.rs` files
- Then, import created files from python or rust project files
"""

import json
from pathlib import Path

DATA_PATH = Path("titles/titles.json")
OUT_PY = Path("titles/titles.py")
OUT_RS = Path("titles/titles.rs")

data = json.loads(DATA_PATH.read_text(encoding="utf8"))

# generate Python constants
py_lines = ["# GENERATED FILE — do not edit\n"]
for key, title in data.items():
    const = key.upper()
    py_lines.append(f"{const} = {json.dumps(title)}")
OUT_PY.write_text("\n".join(py_lines) + "\n", encoding="utf8")

# generate Rust constants
rs_lines = ["// GENERATED FILE — do not edit\n"]
for key, title in data.items():
    const = key.upper()
    rs_lines.append(f"pub const {const}: &str = {json.dumps(title)};")
OUT_RS.write_text("\n".join(rs_lines) + "\n", encoding="utf8")

print("Generated", OUT_PY, "and", OUT_RS)
