from functools import reduce
from typing import List


def main_1(inp: List[str], right=3, down=1):
    found_trees = 0
    for i, line in enumerate(inp[::down]):
        # first row should not be counted
        if i == 0:
            continue
        l = line.strip()
        if l[(i * right) % len(l)] == '#':
            found_trees += 1
    return found_trees


def main_2(inp: List[str]):
    l = [main_1(inp, right=r) for r in [1, 3, 5, 7]] + [main_1(inp, right=1, down=2)]
    return reduce(lambda x, y: x * y, l)


if __name__ == '__main__':
    with open('3.txt') as f:
        inp = f.readlines()
    print(main_1(inp))
    print(main_2(inp))
