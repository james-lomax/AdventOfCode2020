import numpy as np

# contents = """L.LL.LL.LL
# LLLLLLL.LL
# L.L.L..L..
# LLLL.LL.LL
# L.LL.LL.LL
# L.LLLLL.LL
# ..L.L.....
# LLLLLLLLLL
# L.LLLLLL.L
# L.LLLLL.LL"""

with open('input.txt') as infile:
    contents = infile.read()
seats = np.array([[x == "L" for x in row] for row in contents.splitlines()])

# Expand space for roll
col = np.array([False]*seats.shape[0])
seats = np.column_stack((col, seats, col))
row = np.array([False]*seats.shape[1])
seats = np.row_stack((row, seats, row))

occup = np.full(seats.shape, False)

def offsets(seats):
    left = np.roll(seats, -1, 1)
    right = np.roll(seats, 1, 1)
    for i in [-1, 0, 1]:
        yield np.roll(left, i, 0)
        yield np.roll(right, i, 0)
        if i != 0:
            yield np.roll(seats, i, 0)

def step(occup):
    count = np.full(seats.shape, 0)
    for off in offsets(occup):
        count += off + 0

    empties = np.logical_and(seats, np.logical_not(occup))
    # Rule 1, occupy empty seats with no surrounds
    r1 = np.logical_and(empties, count == 0)
    # Rule 2, leave seats with 4 or more surrounds
    r2 = np.logical_and(occup, count >= 4)

    flip = np.logical_or(r1, r2)
    return np.logical_xor(occup, flip)

def step_til_stable(occup):
    nxt = step(occup)
    while (nxt != occup).any():
        occup = nxt
        nxt = step(occup)
    return nxt

occup = step_til_stable(occup)
print(occup.sum())
