import re
from typing import List


def parse_passports(inp: List[str]) -> List[dict]:
    passports = []
    passport = {}
    for line in inp:
        if not line.strip():
            passports.append(passport)
            passport = {}
            continue
        passport.update({k: v for k, v in (x.split(':') for x in line.split())})
    passports.append(passport)
    return passports


def main_1(inp: List[str]):
    required_keys = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']
    passports = parse_passports(inp)
    return len([True for x in passports if all([k in x.keys() for k in required_keys])])


def four_digit(v, mini, maxi):
    if not re.match(r'^\d{4}$', v):
        return False
    return mini <= int(v) <= maxi


def main_2(inp: List[str]):
    def valid(passport: dict):
        for k, v in passport.items():
            if k in ['byr', 'iyr', 'eyr']:
                minimum, maximum = {'byr': (1920, 2002), 'iyr': (2010, 2020), 'eyr': (2020, 2030)}[k]
                if not four_digit(v, minimum, maximum):
                    return False
            elif k == 'hgt':
                if re.match(r'^\d+cm$', v):
                    if not 150 <= int(v[:-2]) <= 193:
                        return False
                elif re.match(r'^\d+in$', v):
                    if not 59 <= int(v[:-2]) <= 76:
                        return False
                else:
                    return False
            elif k == 'hcl':
                if not re.match(r'^#[0-9a-f]{6}$', v):
                    return False
            elif k == 'ecl':
                if v not in ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth']:
                    return False
            elif k == 'pid':
                if not re.match(r'^\d{9}$', v):
                    return False
        return True

    required_keys = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']
    passports = parse_passports(inp)
    basically_valid_passports = [x for x in passports if all([k in x.keys() for k in required_keys])]
    return len([True for p in basically_valid_passports if valid(p)])


if __name__ == '__main__':
    with open('4.txt') as f:
        inp = f.readlines()
    print(main_1(inp))
    print(main_2(inp))
