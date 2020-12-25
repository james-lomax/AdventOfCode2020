def transform(subject, loopc):
    c = 1
    for _ in range(0, loopc):
        c = (c * subject) % 20201227
    return c

def loop_count(subject, out):
    c = 1
    loopc = 0
    while c != out:
        c = (c * subject) % 20201227
        loopc += 1
    return loopc

assert(loop_count(7, 5764801) == 8)

def break_crypt(card_pk, door_pk):
    card_lc = loop_count(7, card_pk)
    card_ek = transform(door_pk, card_lc)
    return card_ek

assert(break_crypt(5764801, 17807724) == 14897079)

ek = break_crypt(14205034, 18047856)
print(f"Part 1 = {ek}")
