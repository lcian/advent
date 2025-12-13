from math import sqrt


class Point:
    def __init__(self, x, y, z):
        self.x = x
        self.y = y
        self.z = z

    @classmethod
    def from_str(cls, s) -> "Point":
        x, y, z = s.strip().split(",")
        x = int(x)
        y = int(y)
        z = int(z)
        return cls(x, y, z)

    def dist(self, other: "Point") -> float:
        return sqrt(
            (self.x - other.x) ** 2 + (self.y - other.y) ** 2 + (self.z - other.z) ** 2
        )


class UnionFind:
    def __init__(self, n: int):
        self.parent = [i for i in range(n)]
        self.size = [1 for _ in range(n)]

    def union(self, x: int, y: int) -> bool:
        p_x = self.find(x)
        p_y = self.find(y)
        if p_x == p_y:
            return False
        if self.size[p_x] < self.size[p_y]:
            return self.union(y, x)
        self.parent[p_y] = p_x
        self.size[p_x] += self.size[p_y]
        return True

    def find(self, x: int) -> int:
        if self.parent[x] != x:
            self.parent[x] = self.find(self.parent[x])
        return self.parent[x]


def problem1():
    lines = []
    with open("input.txt") as f:
        lines = f.readlines()
    points = [Point.from_str(line) for line in lines]
    n = len(points)
    uf = UnionFind(n)
    done = set()
    for _ in range(1000):
        best = (-1, -1)
        for i in range(n):
            for j in range(i + 1, n):
                if (i, j) in done:
                    continue
                d = points[i].dist(points[j])
                if best == (-1, -1) or points[best[0]].dist(points[best[1]]) > d:
                    best = (i, j)
        done.add((best[0], best[1]))
        uf.union(best[0], best[1])

    sizes = {}
    for i in range(n):
        sizes[uf.find(i)] = uf.size[uf.find(i)]
    pairs = list(sorted(sizes.values()))
    a, b, c = pairs[-3:]
    print(a * b * c)


def problem2():
    lines = []
    with open("input.txt") as f:
        lines = f.readlines()
    points = [Point.from_str(line) for line in lines]
    n = len(points)
    uf = UnionFind(n)
    done = set()
    while True:
        best = (-1, -1)
        for i in range(n):
            for j in range(i + 1, n):
                if (i, j) in done:
                    continue
                d = points[i].dist(points[j])
                if best == (-1, -1) or points[best[0]].dist(points[best[1]]) > d:
                    best = (i, j)
        done.add((best[0], best[1]))
        uf.union(best[0], best[1])

        groups = set()
        for i in range(n):
            groups.add(uf.find(i))
        if len(groups) == 1:
            print(points[best[0]].x * points[best[1]].x)
            break


problem1()
problem2()
