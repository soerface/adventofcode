import re
from typing import List


def main_1(inp: List[str]):
    section = "rules"
    ranges = []
    s = 0
    for line in inp:
        if "nearby tickets" in line:
            section = "nearby tickets"
            continue
        if section == "rules":
            ranges += [tuple([int(n) for n in x.split("-")]) for x in re.findall(r"\d+-\d+", line)]
        elif section == "nearby tickets":
            numbers = [int(x) for x in line.split(",")]
            for n in numbers:
                if not any([a <= n <= b for a, b in ranges]):
                    s += n
    return s


def main_2(inp: List[str]):
    section = "rules"
    my_ticket = []
    rule_candidates = []
    rules = {}
    for line in inp:
        if "nearby tickets" in line:
            section = "nearby tickets"
            rule_candidates = [list(rules.keys()) for _ in range(len(my_ticket))]
            continue
        if "your ticket" in line:
            section = "your ticket"
            continue
        if section == "rules":
            rule_name = line.strip().partition(":")[0]
            if not rule_name:
                continue
            rules[rule_name] = [tuple([int(n) for n in x.split("-")]) for x in re.findall(r"\d+-\d+", line)]
        elif section == "your ticket":
            my_ticket = [int(x) for x in line.split(",")]
            section = None
        elif section == "nearby tickets":
            numbers = [int(x) for x in line.split(",")]
            for i, n in enumerate(numbers):
                for name, rule in rules.items():
                    if name not in rule_candidates[i]:
                        continue
                    if not any([a <= n <= b for a, b in rule]):
                        print("removing", name, "from col", i)
                        print(n, rule)
                        rule_candidates[i].remove(name)
                        print(rule_candidates[i])
    print(rule_candidates)
    # at least one column will be unambiguous - remove this column from the other candidates
    # until we only have one possible column for each number
    while any([len(x) > 1 for x in rule_candidates]):
        for i, candidate in enumerate(rule_candidates):
            if len(candidate) == 1:
                for j, candidate2 in enumerate(rule_candidates):
                    if j == i or candidate[0] not in candidate2:
                        continue
                    candidate2.remove(candidate[0])
    # TODO: only works for example
    print(rule_candidates)


if __name__ == "__main__":
    with open("16.txt") as f:
        inp = f.readlines()
    print(main_1(inp))
    print(main_2(inp))
