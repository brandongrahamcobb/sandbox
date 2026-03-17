from pathlib import Path
import logging
import sys

logger = logging.getLogger(__name__)


def setup():
    print("""
        remove_lines.py  Copyright (C) 2026  Cobb, Brandon Graham
        This program comes with ABSOLUTELY NO WARRANTY.
        This is free software, and you are welcome to redistribute it
        under certain conditions.
    """)
    logging.basicConfig(filename="replace_and_walk.log", level=logging.DEBUG)


class RemoveLines:
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

        if not self.directory:
            self.directory == "."
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
            if filename.name == Path(__file__).name:
                continue
            with open(filename, "r") as f:
                lines = f.readlines()
            for i, line in enumerate(lines):
                if self.before in line:
                    lines[i] = line.replace(line, self.after)
            with open(filename, "w") as f:
                f.write("".join(lines))


if __name__ == "__main__":
    obj = RemoveLines()
    obj.main()
