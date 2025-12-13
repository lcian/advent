from collections import defaultdict
from copy import deepcopy


def count_ways(start, end, rev_adj, nodes):
    nodes = deepcopy(nodes)
    while nodes and nodes[-1] != end:
        nodes.pop()

    ways = defaultdict(int)
    ways[end] = 1
    while nodes:
        u = nodes.pop()
        for v in rev_adj[u]:
            ways[v] += ways[u]
    return ways[start]


def problem1():
    adj = defaultdict(set)
    rev_adj = defaultdict(set)
    with open("input.txt") as f:
        lines = f.readlines()
        for line in lines:
            u = line.strip().split(":")[0].strip()
            for v in line.strip().split(":")[1].strip().split(" "):
                adj[u].add(v)
                rev_adj[v].add(u)

    visited = set()
    nodes = []

    def dfs(u):
        if u in visited:
            return
        visited.add(u)
        for v in adj[u]:
            dfs(v)
        nodes.append(u)

    dfs("you")
    nodes.reverse()
    print(count_ways("you", "out", rev_adj, nodes))


def problem2():
    adj = defaultdict(set)
    rev_adj = defaultdict(set)
    with open("input.txt") as f:
        lines = f.readlines()
        for line in lines:
            u = line.strip().split(":")[0].strip()
            for v in line.strip().split(":")[1].strip().split(" "):
                adj[u].add(v)
                rev_adj[v].add(u)

    visited = set()
    nodes = []

    def dfs(u):
        if u in visited:
            return
        visited.add(u)
        for v in adj[u]:
            dfs(v)
        nodes.append(u)

    dfs("svr")
    nodes.reverse()

    res = 0
    paths = [("svr", "dac", "fft", "out"), ("svr", "fft", "dac", "out")]
    for path in paths:
        val = 1
        for i in range(3):
            val *= count_ways(path[i], path[i + 1], rev_adj, nodes)
        res += val
    print(res)


problem1()
problem2()
