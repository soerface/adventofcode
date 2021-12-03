from pathlib import Path
from typing import List


def main_1(inp: List[str]):
    return sum(map(lambda t: t[1] > t[0], zip(inp, inp[1:])))


def main_2(inp: List[str]):
    w_size = 3
    sl_win = list(map(lambda i: sum(inp[i:i+w_size]), range(len(inp))))
    return sum(map(lambda t: t[1] > t[0], zip(sl_win, sl_win[1:])))


def run(input_file: Path):
    print(f"Running for {input_file.name}")
    with open(input_file) as f:
        inp = list(map(int, f.readlines()))
    print(f"Task 1: {main_1(inp)}")
    print(f"Task 2: {main_2(inp)}")


if __name__ == "__main__":
    example_file = Path(__file__).parent / "example.txt"
    input_file = Path(__file__).parent / "input.txt"
    if example_file.exists():
        run(example_file)
    if input_file.exists():
        run(input_file)
