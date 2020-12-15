from typing import List


def main_1(inp: List[int]):
    l = inp.copy()
    for i in range(len(l), 2020):
        last_number = l[-1]
        try:
            last_occurence = len(l) - 1 - l[-2::-1].index(last_number)
        except ValueError:
            last_occurence = i
        l.append(i - last_occurence)
    return l[-1]


def main_2(inp: List[str]):
    return


if __name__ == "__main__":
    with open("15.examples.txt") as f:
        inp = f.readlines()
    for line in inp:
        print(main_1([int(x) for x in line.split(",")]))
    print(main_2(inp))
