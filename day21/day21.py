from functools import reduce
import re

pattern = re.compile(r"(([a-z]+ )+)\(contains (([a-z]+(, )?)+)\)")
word_pat = re.compile(r"([a-z]+)")


def parse(contents):
    items = [
        (
            {x.group(0) for x in word_pat.finditer(m.group(1))}, 
            {x.group(0) for x in word_pat.finditer(m.group(3))}
        ) for m in pattern.finditer(contents)]

    return items


def possible(allergen, items):
    return reduce(set.intersection, [
        ingr
        for ingr, alrg in items
        if allergen in alrg
    ])


def proc(items):
    all_ingredients = {x for i in items for x in i[0]}
    all_allergens = {x for i in items for x in i[1]}

    # First enumerate all possibilities for each allergen
    al_possible = {allergen: possible(allergen, items) for allergen in all_allergens}

    accounted_for = reduce(set.union, list(al_possible.values()))
    unaccounted = all_ingredients.difference(accounted_for)

    count_unacc = len([x for i in items for x in i[0] if x in unaccounted])
    print(f"Count of unnaccounted = {count_unacc}")

    # Now eliminate them by singles...
    allergens = {}
    while len(allergens) < len(all_allergens):
        for (alrg, ingr) in al_possible.items():
            if len(ingr) == 1:
                ingredient = next(iter(ingr))
                allergens[alrg] = ingredient
                al_possible = {
                    k: [
                        i for i in v if i != ingredient
                    ]
                    for k, v in al_possible.items()
                    if k != alrg
                }
                print(al_possible)
                break
    
    allergens = list(allergens.items())
    allergens = sorted(allergens, key=lambda x: x[0])
    al_list = ",".join([a[1] for a in allergens])
    print("List = " + al_list)

    return (count_unacc, al_list)


def test():
    sample = """mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"""

    assert proc(parse(sample)) == (5, "mxmxvkd,sqjhc,fvjkl")


def main():
    with open("input.txt", "r") as f:
        proc(parse(f.read()))


if __name__ == "__main__":
    test()
    main()
