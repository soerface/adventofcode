import re
from itertools import chain
from typing import List, Dict, Set, Tuple


def parse_rules(inp: List[str]) -> Dict[str, List[Tuple[int, str]]]:
    rules = {}
    for line in inp:
        outer, inner = line.strip().split("contain")
        outer = outer.replace("bags", "").strip()
        inner = [(int(re.search(r"\d+", x).group(0)), color) for x in inner.split(",") if
                 (color := re.search(r"(?:\d+|no) (.+) bags?", x).group(1)) != "other"]
        rules[outer] = inner
    return rules


def parent_bags(rules: Dict[str, List[Tuple[int, str]]], colors: Set[str]) -> Set[str]:
    return set([k for k, v in rules.items() if colors.intersection(set([x[1] for x in v]))])


def main_1(inp: List[str]):
    rules = parse_rules(inp)
    valid_colors = parent_bags(rules, {"shiny gold"})
    a = {}
    while a != valid_colors:
        a = valid_colors
        valid_colors = valid_colors.union(parent_bags(rules, valid_colors))

    return len(valid_colors)


def main_2(inp: List[str]):
    rules = parse_rules(inp)
    total_bags = -1

    bag_stack = ["shiny gold"]
    while bag_stack:
        bag_stack.extend(chain.from_iterable([x[0] * [x[1]] for x in rules[bag_stack.pop()]]))
        total_bags += 1

    return total_bags


if __name__ == "__main__":
    with open("7.txt") as f:
        inp = f.readlines()
    print(main_1(inp))
    print(main_2(inp))
