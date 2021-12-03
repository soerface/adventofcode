import re
from typing import List


def apply_mask_to_value(val: int, mask: str) -> int:
    and_mask = int(mask.replace("X", "1"), base=2)
    or_mask = int(mask.replace("X", "0"), base=2)
    return (val & and_mask) | or_mask


def main_1(inp: List[str]):
    mask = "X" * 36
    mem = {}
    for instruction in inp:
        cmd, val = instruction.split("=")
        if cmd.strip() == "mask":
            mask = val.strip()
            continue
        address = int(re.search(r"\d+", cmd).group(0))
        val = int(val)
        mem[address] = apply_mask_to_value(val, mask)
    return sum(mem.values())


def apply_mask_to_address(address: int, mask: str) -> List[int]:
    indices = [m.start() for m in re.finditer("X", mask)]
    fst = "{0:0Xb}".replace("X", str(len(indices)))
    for n in range(2**len(indices)):
        addr = address | int(mask.replace("X", "0"), base=2)
        addr = list(("{0:0" + str(len(mask)) + "b}").format(addr))
        for i, index in enumerate(indices):
            val = fst.format(n)[i]
            addr[index] = val
        yield int("".join(addr), base=2)


def main_2(inp: List[str]):
    mask = "0" * 36
    mem = {}
    for instruction in inp:
        cmd, val = instruction.split("=")
        if cmd.strip() == "mask":
            mask = val.strip()
            continue
        address = int(re.search(r"\d+", cmd).group(0))
        val = int(val)
        for addr in apply_mask_to_address(address, mask):
            mem[addr] = val
    return sum(mem.values())


if __name__ == "__main__":
    with open("14.txt") as f:
        inp = f.readlines()
    print(main_1(inp))
    print(main_2(inp))
