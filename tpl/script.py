from pathlib import Path
from typing import List


def main_1(inp: List[str]):
    return


def main_2(inp: List[str]):
    return


def run(input_file: Path):
    print(f"Running for {input_file.name}")
    with open(input_file) as f:
        inp = f.readlines()
    print(f"Task 1: {main_1(inp)}")
    print(f"Task 2: {main_2(inp)}")


if __name__ == "__main__":
    example_file = Path(__file__).parent / "example.txt"
    input_file = Path(__file__).parent / "input.txt"
    if example_file.exists():
        run(example_file)
    if input_file.exists():
        run(input_file)
