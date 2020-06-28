import itertools

""" rev_rule! {not_ffff, "(~ (vec false false false false))", "(~ (vec true true true true))"} """


# f <x1, x2, x3, x4> = <f x1, f x2, f x3, f x4>
# f <(x1, y1), (x2, y2), (x3, y3), (x4, y4)> = <f (x1, y1), f (x2, y2), f (x3,
# y3) >

def main():
    # not
    # for (x, y, z, w) in list(itertools.product(*[[False, True]]*4)):
    #     strs = [str(b).lower() for b in [x, y, z, w]]
    #     not_strs = [str(not b).lower() for b in [x, y, z, w]]
    #     print(f"rev_rule! {{not_{''.join([s[0] for s in strs])}, \"", end='')
    #     print(
    #         f"(~ (vec {' '.join(strs)}))\", ", end='')
    #     print(
    #         f"\"(vec {' '.join(not_strs)})\"}}")

    # and
    for x1, y1, x2, y2, x3, y3, x4, y4 in list(itertools.product(*[[False, True]]*8)):
        strs = [str(b).lower() for b in (x1, y1, x2, y2, x3, y3, x4, y4)]
        prs = [(x1, y1), (x2, y2), (x3, y3), (x4, y4)]
        # print(f"rev_rule! {{and_{''.join([s[0] for s in strs])}, \"", end='')

        print(f"and_{''.join([s[0] for s in strs])}(),")
        # print(f"(& (vec", end='')
        # for pr in prs:
        #     print(f" (pr {str(pr[0]).lower()} {str(pr[1]).lower()})", end='')
        # print(f"))\", ", end='')
        # and_prs = [x and y for x, y in prs]
        # print(f" \"(vec {' '.join([str(b).lower() for b in and_prs])})\"}}")


if __name__ == "__main__":
    main()
