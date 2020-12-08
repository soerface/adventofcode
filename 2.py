import re
from collections import Counter
from typing import List


def main_1(inp: List[str]):
    valid_passwords = 0
    for line in inp:
        minimum, maximum, letter, word = re.search(r"(\d+)-(\d+) (\w): (\w+)", line).groups()
        c = Counter(word)
        if int(minimum) <= c[letter] <= int(maximum):
            valid_passwords += 1
    return valid_passwords


def main_2(inp: List[str]):
    valid_passwords = 0
    for line in inp:
        pos1, pos2, letter, word = re.search(r"(\d+)-(\d+) (\w): (\w+)", line).groups()
        # positions are not zero indexed, fix it
        pos1 = int(pos1) - 1
        pos2 = int(pos2) - 1
        letter_a, letter_b = word[pos1], word[pos2]
        if (letter_a == letter) ^ (letter_b == letter):
            valid_passwords += 1
    return valid_passwords


if __name__ == "__main__":
    with open("2.txt") as f:
        inp = f.readlines()
    print(main_1(inp))
    print(main_2(inp))
