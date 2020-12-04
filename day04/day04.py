import re
import typing as t
from functools import partial


def parse_entry(entry):
    fields = [s.split(":") for line in entry.split("\n") for s in line.split(" ") if s.find(":") >= 0]
    return {s[0]: s[1] for s in fields}


def parse(contents) -> t.List[dict]:
    entries = contents.split("\n\n")
    return list(map(parse_entry, entries))


def is_valid_passport(passport: dict) -> bool:
    required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
    return len([f for f in required_fields if f not in passport]) == 0


def count_valid(passports: t.List[dict]) -> int:
    return len(list(filter(is_valid_passport, passports)))


def validate_num_range(min: int, max: int, s: str) -> bool:
    if not re.match("^[0-9]{4}$", s):
        return False
    else:
        n = int(s)
        return n >= min and n <= max

def validate_height(s: str) -> bool:
    hgt_m = re.match(r"^([0-9]+)(cm|in)$", s)
    if hgt_m:
        hgt = int(hgt_m.group(1))
        if hgt_m.group(2) == "cm":
            if hgt < 150 or hgt > 193:
                return False
        elif hgt_m.group(2) == "in":
            if hgt < 59 or hgt > 76:
                return False
        else:
            return False
    else:
        return False
    
    return True


def validate_enum(opts: t.List[str], s: str) -> bool:
    return s in opts


def is_valid_fields(passport: dict) -> bool:
    if not is_valid_passport(passport):
        return False

    rules = {
        "byr": partial(validate_num_range, 1920, 2002),
        "iyr": partial(validate_num_range, 2010, 2020),
        "eyr": partial(validate_num_range, 2020, 2030),
        "hgt": validate_height,
        "hcl": lambda s: re.match(r"^#[0-9a-f]{6}$", s) is not None,
        "ecl": partial(validate_enum, ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]),
        "pid": lambda s: re.match(r"^[0-9]{9}$", s) is not None
    }
    
    for field, rule in rules.items():
        if not rule(passport[field]):
            return False
    
    return True


def test_part1():
    sample = """ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"""

    assert count_valid(parse(sample)) == 2


def test_part2():
    assert not is_valid_fields(parse_entry("eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"))
    assert not is_valid_fields(parse_entry("iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946"))
    assert not is_valid_fields(parse_entry("hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"))
    assert not is_valid_fields(parse_entry("hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007"))
    
    assert is_valid_fields(parse_entry("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f"))
    assert is_valid_fields(parse_entry("eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"))
    assert is_valid_fields(parse_entry("hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022"))
    assert is_valid_fields(parse_entry("iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"))


test_part1()

with open("input.txt", "r") as f:
    contents = f.read()

passports = parse(contents)
print(f"Part 1: {count_valid(passports)}")

test_part2()

count = len(list(filter(is_valid_fields, passports)))
print(f"Part 2: {count}")