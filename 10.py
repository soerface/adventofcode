from typing import List
from functools import lru_cache

import numpy as np


def main_1(inp: List[int]):
    adapters = np.array(sorted(inp) + [max(inp) + 3])
    adapters_shifted = np.roll(adapters, 1)
    adapters_shifted[0] = 0
    steps = adapters - adapters_shifted
    occurences = dict(zip(*np.unique(steps, return_counts=True)))
    return occurences[1] * occurences[3]


def main_2(inp: List[int]):
    all_adapters = np.array([0] + sorted(inp))

    @lru_cache()
    def n_combinations(to):
        adapters = all_adapters[all_adapters <= to]
        n = 1
        if len(adapters) < 3:
            # Two adapter can't be combined in another way
            return 1
        # start with the number of combinations from the preceding number
        n = n_combinations(adapters[-2])
        # now, check if we could have skipped adapters, and add their number of combinations
        if to - adapters[-3] <= 3:
            n += n_combinations(adapters[-3])
        if len(adapters) > 3 and (to - adapters[-4] <= 3):
            n += n_combinations(adapters[-4])
        return n

    print(all_adapters)
    return n_combinations(all_adapters[-1])


if __name__ == "__main__":
    with open("10.txt") as f:
        inp = [int(x) for x in f.readlines()]
    print(main_1(inp))
    print(main_2(inp))
