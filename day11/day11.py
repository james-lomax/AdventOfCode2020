import numpy as np
import scipy.signal

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

occup = np.full(seats.shape, False)

kernel = np.array([
    [1, 1, 1],
    [1, 0, 1],
    [1, 1, 1]
])

def step(occup):
    count = scipy.signal.convolve2d(occup, kernel)[1:-1, 1:-1]

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
