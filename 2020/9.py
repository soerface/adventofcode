from itertools import combinations
from typing import List


def valid(n: int, l: List[int]):
    return n in [sum(x) for x in combinations(l, 2)]


def main_1(inp: List[int], preamble_len: int) -> int:
    n = preamble_len
    while valid(inp[n], inp[n-preamble_len:n]):
        n += 1
    return inp[n]


def main_2(inp: List[int], preamble_len: int) -> int:
    s = main_1(inp, preamble_len)
    a, b = 0, 1
    while sum(inp[a:b]) != s:
        if sum(inp[a:b]) < s:
            b += 1
        else:
            a += 1
    return min(inp[a:b]) + max(inp[a:b])


if __name__ == "__main__":
    with open("9.txt") as f:
        inp = [int(x.strip()) for x in f.readlines()]
    print(main_1(inp, 25))
    print(main_2(inp, 25))
