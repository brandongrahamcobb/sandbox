"""#!/bin/python3
The purpose of this program is to be a tool for finding and replacing strings by walking through all files.
Copyright (C) 2026  Cobb, Brandon Graham

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
"""

from pathlib import Path
import logging
import sys

logger = logging.getLogger(__name__)


def setup():
    print("""
        replace_and_walk.py  Copyright (C) 2026  Cobb, Brandon Graham
        This program comes with ABSOLUTELY NO WARRANTY.
        This is free software, and you are welcome to redistribute it
        under certain conditions.
    """)
    logging.basicConfig(filename="replace_and_walk.log", level=logging.DEBUG)


class ReplaceWalk:
    def __init__(
        self,
        before: str | None = None,
        after: str | None = None,
        directory: str | None = None,
        file_spec: str | None = None,
    ):
        self.before = before
        self.after = after
        self.directory = directory
        self.file_spec = file_spec

    def main(self):
        setup()
        for i, arg in enumerate(sys.argv):
            match arg:
                case "--dir" | "-d" | "--directory":
                    self.directory = sys.argv[i + 1]
                case "--bef" | "-b" | "--before":
                    self.before = sys.argv[i + 1]
                case "--aft" | "-a" | "--after":
                    self.after = sys.argv[i + 1]
                case "--file" | "-f" | "--files":
                    self.file_spec = sys.argv[i + 1]
                case "--bff" | "-bf" | "--before_file":
                    paths = Path(".").rglob(sys.argv[i + 1])
                    for path in paths:
                        with open(path, "r") as f:
                            self.before = f.read()
                case "--aff" | "-af" | "--after_file":
                    paths = Path(".").rglob(sys.argv[i + 1])
                    for path in paths:
                        with open(path, "r") as f:
                            self.after = f.read()
                case "-w":
                    print()
                    return
                case "-c":
                    print()
                    return
                case "-s":
                    self.file_spec = f"*{sys.argv[i +1]}"

        if not self.directory:
            self.directory = "."
        if self.before is None or self.after is None:
            raise ValueError("Missing before and after strings.")
        if not self.file_spec:
            self.file_spec = "*.py"
        try:
            path = Path(self.directory)
        except FileNotFoundError:
            logger.debug(self.directory + "is not found.")
        paths = path.rglob(self.file_spec)
        for filename in paths:
            print(filename)
            if filename.name == Path(__file__).name:
                continue
            with open(filename, "r") as f:
                content = f.read()

            search_start = 0
            result = []

            while True:
                pos = content.find(self.before, search_start)
                if pos == -1:
                    result.append(content[search_start:])
                    break

                # grab some surrounding context
                context_start = content.rfind("\n", 0, pos) + 1
                context_end = content.find("\n", pos + len(self.before))
                context = content[
                    context_start : context_end if context_end != -1 else None
                ]

                go = input(f"Replace in {filename}?\n{context}\n[y/n]: ")

                if go.lower() != "n":
                    result.append(content[search_start:pos])
                    result.append(self.after)
                    search_start = pos + len(self.before)
                else:
                    result.append(content[search_start : pos + len(self.before)])
                    search_start = pos + len(self.before)

            with open(filename, "w") as f:
                f.write("".join(result))
            # with open(filename, "r") as f:
            #     lines: list[str] = []
            #     for line in f:
            #         if self.before in line:
            #             go = input(
            #                 f"Do you want to replace the occurence in this line? {line}"
            #             )
            #             if go.lower() != "n":
            #                 line = line.replace(self.before, self.after)
            #         lines.append(line)
            #     with open(filename, "w") as f:
            #         for line in lines:
            #             f.write(f"{line}")
            #


if __name__ == "__main__":
    obj = ReplaceWalk()
    obj.main()
