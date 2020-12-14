from typing import List


def main_1(inp: List[str]):
    earliest = int(inp[0])
    busses = [int(x) for x in inp[1].split(",") if x != "x"]
    n = earliest
    l = []
    while not any(l):
        n += 1
        l = [n % b == 0 for b in busses]
    b = busses[l.index(True)]
    return (n - earliest) * b


def main_2(inp: List[str]):
    busses = [int(x) if x != "x" else 1 for x in inp[1].split(",")]
    bus_orders = {b: i for i, b in enumerate(busses)}
    sorted_busses = sorted(busses, reverse=True)
    n = 1
    steps = 0
    while True:
        # if n % 10000000 == 0:
        print(n)
        ok = True
        for b in sorted_busses:
            i = bus_orders[b]
            if (offset := (n + i) % b) != 0:
                ok = False
                n += b - offset
                break
        if ok:
            break
        steps += 1
    print("Steps:", steps)
    return n


if __name__ == "__main__":
    with open("13.example.txt") as f:
        inp = f.readlines()
    print(main_1(inp))
    print(main_2(inp))
