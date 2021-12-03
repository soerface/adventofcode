from typing import List


def main_1(inp: List[str]):
    groups = "".join(inp).split("\n\n")
    counts = [len(set(g.replace("\n", ""))) for g in groups]
    return sum(counts)


def main_2(inp: List[str]):
    groups = "".join(inp).split("\n\n")
    counts = []
    for group in groups:
        questions = set(group.replace("\n", ""))
        counts.append(len([True for x in questions if all([x in p for p in group.split("\n")])]))
    return sum(counts)


if __name__ == "__main__":
    with open("6.txt") as f:
        inp = f.readlines()
    print(main_1(inp))
    print(main_2(inp))
