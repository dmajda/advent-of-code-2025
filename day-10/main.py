def solve(wirings, joltages):
    return solve2(sorted(wirings, key=lambda x: [len(x), *x], reverse=True), joltages)


def solve2(wirings, joltages):
    # print(wirings, joltages)

    if len(wirings) == 0:
        if max(joltages) == 0:
            return 0
        else:
            return None

    max_wiring_len = max([len(wiring) for wiring in wirings])
    longest_wirings = [wiring for wiring in wirings if len(wiring) == max_wiring_len]

    max_presses = [compute_max_presses(wiring, joltages) for wiring in longest_wirings]
    presses = [range(0, p + 1) for p in max_presses]

    applications = []
    for wiring, pp in zip(longest_wirings, presses):
        for p in pp:
            applications.append((p, wiring))

    applications = reversed(sorted(applications, key=lambda a: [a[0], *a[1]]))

    for p, w in applications:
        # print(f"  pressing {wiring} {presses} times (max {max_presses})")
        new_joltages = apply_presses(w, joltages, p)
        remainin_presses = solve2([ww for ww in wirings if ww != w], new_joltages)

        if remainin_presses is not None:
            return p + remainin_presses


def compute_max_presses(wiring, joltages):
    return min([joltages[i] for i in wiring])


def apply_presses(wiring, joltages, presses):
    new_joltages = joltages.copy()

    for i in wiring:
        new_joltages[i] -= presses

    return new_joltages


print(solve(wirings=[[3], [1, 3], [2], [2, 3], [0, 2], [0, 1]], joltages=[3, 5, 4, 7]))

print(
    solve(
        wirings=[[0, 2, 3, 4], [2, 3], [0, 4], [0, 1, 2], [1, 2, 3, 4]],
        joltages=[7, 5, 12, 7, 2],
    )
)

print(
    solve(
        wirings=[[0, 1, 2, 3, 4], [0, 3, 4], [0, 1, 2, 4, 5], [1, 2]],
        joltages=[10, 11, 11, 5, 10, 5],
    )
)

print(
    solve(
        wirings=[
            [3, 4, 5, 7],
            [2, 4, 5, 6, 7],
            [1, 4, 7],
            [1, 3, 4, 7],
            [1, 2, 3, 4, 5, 7],
            [7],
            [1, 2, 3, 6],
            [0, 1, 3, 6, 7],
        ],
        joltages=[4, 59, 39, 250, 242, 220, 26, 250],
    )
)
