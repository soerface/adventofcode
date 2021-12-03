from typing import List


def replace(s: str):
    chars = {"F": "0", "B": "1", "R": "1", "L": "0"}
    for k, v in chars.items():
        s = s.replace(k, v)
    return s


def main_1(inp: List[str]):
    return max(int(replace(line), base=2) for line in inp)


def main_2(inp: List[str]):
    seats = sorted([int(replace(line), base=2) for line in inp])
    i = seats[0]
    while i in seats:
        i += 1
    return i


if __name__ == "__main__":
    with open("5.txt") as f:
        inp = f.readlines()
    print(main_1(inp))
    print(main_2(inp))
