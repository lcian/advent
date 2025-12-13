from collections import Counter


def problem1():
    input = None
    with open("input.txt") as f:
        input = f.readlines()
    assert input
    beams = set()
    start = input[0].find("S")
    beams.add(start)

    res = 0
    for line in input[1:]:
        next_beams = beams.copy()
        for i, c in enumerate(line):
            if c != "^":
                continue
            if i not in beams:
                continue
            res += 1
            for j in (i - 1, i + 1):
                if j < 0 or j >= len(line):
                    pass
                next_beams.add(j)
            next_beams.remove(i)
        beams = next_beams
    print(res)


def problem2():
    input = None
    with open("input.txt") as f:
        input = f.readlines()
    assert input
    beams = Counter()
    start = input[0].find("S")
    beams[start] = 1

    res = 0
    for line in input[1:]:
        next_beams = beams.copy()
        for i, c in enumerate(line):
            if c != "^":
                continue
            if beams[i] == 0:
                continue
            res += 1
            for j in (i - 1, i + 1):
                if j < 0 or j >= len(line):
                    pass
                next_beams[j] += beams[i]
            next_beams[i] = 0
        beams = next_beams
    print(sum(v for _, v in beams.items()))


problem1()
problem2()
