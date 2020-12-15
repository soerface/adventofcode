from typing import List


def main(inp: List[int], to=2020):
    last_occurence = {v: i+1 for i, v in enumerate(inp[:-1])}
    last_number = inp[-1]
    if to < len(inp):
        return inp[to]
    for i in range(len(inp), to):
        new_number = i - last_occurence.get(last_number, i)
        last_occurence[last_number] = i
        last_number = new_number
    return new_number


if __name__ == "__main__":
    with open("15.txt") as f:
        inp = f.readlines()
    for line in inp:
        print(main([int(x) for x in line.split(",")], to=2020))
        print(main([int(x) for x in line.split(",")], to=30000000))
