from typing import List

import numpy as np


def main_1(inp: List[str]):
    earliest = int(inp[0])
    busses = [int(x) for x in inp[1].split(",") if "x" not in x]
    n = earliest
    l = []
    while not any(l):
        n += 1
        l = [n % b == 0 for b in busses]
    b = busses[l.index(True)]
    return (n - earliest) * b


def main_2(inp: List[str]):
    busses = [int(x) if "x" not in x else 1 for x in inp[1].split(",")]
    n = step_size = busses[0]
    steps = 0
    for idx, b in enumerate(busses):
        while (n+idx) % b:
            n += step_size
            steps += 1
        step_size = np.prod(busses[:idx+1])
        print("step size now", step_size)
    print("Steps:", steps)
    return n


if __name__ == "__main__":
    with open("13.txt") as f:
        inp = f.readlines()
    print(main_1(inp))
    print(main_2(inp))
