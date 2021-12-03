from typing import List
import numpy as np


def input_to_matrix(inp: List[str]) -> np.ndarray:
    numbers = [
        [int(x.replace(".", "0").replace("L", "1")) for x in l if x in "L."]
        for l in inp
    ]
    return np.array(numbers)


def apply_rule(m: np.ndarray) -> int:
    if m[1, 1] == 0:
        return 0
    # remember that bincount will also count our own seat
    _, empty, occupied = np.bincount(m.flatten(), minlength=3)
    if occupied == 0:
        return 2
    if occupied >= 5:
        return 1
    return m[1, 1]


def main_1(inp: List[str]):
    m = np.pad(input_to_matrix(inp), 1)
    # probably filtering / sliding window can be done way more efficient, but my numpy skills are rusty :(
    changes = True
    while changes:
        new_m = np.zeros(shape=m.shape, dtype=int)
        changes = False
        w, h = m.shape
        for x in range(1, w - 1):
            for y in range(1, h - 1):
                new_m[x, y] = apply_rule(m[x-1:x+2, y-1:y+2])
                if new_m[x, y] != m[x, y]:
                    changes = True
        m = new_m.copy()
    _, empty, occupied = np.bincount(m.flatten(), minlength=3)
    return occupied


def first_in_sight(m: np.ndarray, x0: int, y0: int, x_dir: int, y_dir: int):
    x, y = x0, y0
    w, h = m.shape
    while True:
        x, y = x + x_dir, y + y_dir
        if x < 0 or x >= w or y < 0 or y >= h:
            return 0
        v = m[x, y]
        if v != 0:
            return v


def apply_rule2(m: np.ndarray, x0: int, y0: int):
    if m[x0, y0] == 0:
        return 0
    directions = [(0, 1), (0, -1), (1, 0), (1, 1), (1, -1), (-1, 0), (-1, 1), (-1, -1)]
    occupied = 0
    for d in directions:
        occupied += 1 if first_in_sight(m, x0, y0, *d) == 2 else 0
    if m[x0, y0] == 1:
        if occupied == 0:
            return 2
    if m[x0, y0] == 2:
        if occupied >= 5:
            return 1
    return m[x0, y0]


def main_2(inp: List[str]):
    m = input_to_matrix(inp)
    changes = True
    n = 0
    while changes:
        n += 1
        new_m = np.zeros(shape=m.shape, dtype=int)
        changes = False
        w, h = m.shape
        for x in range(w):
            for y in range(h):
                new_m[x, y] = apply_rule2(m, x, y)
                if new_m[x, y] != m[x, y]:
                    changes = True
        m = new_m.copy()
    _, empty, occupied = np.bincount(m.flatten(), minlength=3)
    return occupied


if __name__ == "__main__":
    with open("11.txt") as f:
        inp = f.readlines()
    print(main_2(inp))
