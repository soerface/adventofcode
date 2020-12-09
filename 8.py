from typing import List, Tuple


def parse_instruction(s: str) -> Tuple[str, int]:
    return s[:3], int(s[4:])


def execute_instruction(cmd: str, arg: int, instruction_pointer: int, acc: int) -> Tuple[int, int]:
    if cmd == "nop":
        return instruction_pointer + 1, acc
    if cmd == "acc":
        return instruction_pointer + 1, acc + arg
    if cmd == "jmp":
        return instruction_pointer + arg, acc


def main_1(inp: List[str]):
    i = 0
    visited = []
    acc = 0
    terminated = False
    while i not in visited:
        visited.append(i)
        cmd, arg = parse_instruction(inp[i])
        i, acc = execute_instruction(cmd, arg, i, acc)
        if i == len(inp):
            terminated = True
            break
    return acc, terminated


def swap_command(inp: str):
    cmd, _ = parse_instruction(inp)
    if cmd == "nop":
        return inp.replace("nop", "jmp")
    elif cmd == "jmp":
        return inp.replace("jmp", "nop")
    return False


def main_2(inp: List[str]):
    terminated = False
    i = -1
    acc = 0
    while not terminated:
        i += 1
        fixed_code = inp.copy()
        fixed_code[i] = swap_command(fixed_code[i])
        if not fixed_code[i]:
            continue
        acc, terminated = main_1(fixed_code)
    return acc, terminated


if __name__ == "__main__":
    with open("8.txt") as f:
        inp = f.readlines()
    print(main_1(inp))
    print(main_2(inp))
