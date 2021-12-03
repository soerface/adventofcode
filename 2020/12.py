from typing import List

import numpy as np

CARDINAL_POINTS = ["N", "E", "S", "W"]

def main_1(inp: List[str]):
    facing = 1
    coords = np.zeros(2, dtype=int)
    for instruction in inp:
        action, units = instruction[0], int(instruction[1:])
        if action in ["L", "R"]:
            d = 1 if action == "R" else -1
            facing = (facing + (units // 90 * d)) % len(CARDINAL_POINTS)
            continue
        elif action == "F":
            direction = CARDINAL_POINTS[facing]
        else:
            direction = action
        axis = 0 if direction in ["E", "W"] else 1
        direction = 1 if direction in ["N", "E"] else -1
        coords[axis] += units * direction
    return sum(abs(coords))


def main_2(inp: List[str]):
    waypoint = np.array([10, 1], dtype=int)
    coords = np.zeros(2, dtype=int)
    for instruction in inp:
        action, units = instruction[0], int(instruction[1:])
        if action in ["L", "R"]:
            for _ in range(units // 90):
                d = 1 if action == "R" else -1
                waypoint = waypoint[1::-1] * [1, -1] * d
        elif action == "F":
            coords += waypoint * units
        else:
            axis = 0 if action in ["E", "W"] else 1
            direction = 1 if action in ["N", "E"] else -1
            waypoint[axis] += units * direction
    return sum(abs(coords))


if __name__ == "__main__":
    with open("12.txt") as f:
        inp = f.readlines()
    print(main_1(inp))
    print(main_2(inp))
