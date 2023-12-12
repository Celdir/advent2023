import sys
from functools import cache

@cache
def count(springs, groups, last = '.'):
    groups = list(groups)
    if len(springs) == 0:
        if len(groups) == 0 or groups == [0]:
            return 1
        else:
            return 0
    if len(groups) == 0:
        if springs.count('#') == 0:
            return 1
        else:
            return 0

    spring = springs[0]
    if groups[0] < 0:
        return 0

    if last == '#':
        if spring == '#':
            groups[0] -= 1
            return count(springs[1:], tuple(groups[:]), spring)
        elif spring == '?':
            ans = 0
            if groups[0] == 0:
                ans += count(springs[1:], tuple(groups[1:]), '.')
            groups[0] -= 1
            ans += count(springs[1:], tuple(groups[:]), '#')
            return ans
        else:
            if groups[0] == 0:
                return count(springs[1:], tuple(groups[1:]), spring)
            else:
                return 0
    else:
        if spring == '#':
            groups[0] -= 1
            return count(springs[1:], tuple(groups[:]), spring)
        elif spring == '?':
            ans = count(springs[1:], tuple(groups[:]), '.')
            groups[0] -= 1
            ans += count(springs[1:], tuple(groups[:]), '#')
            return ans
        else:
            return count(springs[1:], tuple(groups[:]), spring)

sum = 0
for line in sys.stdin:
    springs, groups_str = line.split(" ")
    springs = '?'.join([springs, springs, springs, springs, springs])
    groups = tuple(map(lambda x: int(x), groups_str.split(",")))
    groups = groups + groups + groups + groups + groups
    sum += count(springs, groups)
print(sum)
