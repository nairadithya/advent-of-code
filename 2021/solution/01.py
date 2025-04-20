#!/usr/bin/env python3


# A
with open("../input/01.txt", "r") as file:
    data = [int(line) for line in file.readlines()]
    increased_new_list = [
        "increased" if data[i] > data[i - 1] else "not increased"
        for i in range(1, len(data))
    ]
    print(increased_new_list.count("increased"))

    # B
    depthness = [sum(data[i : i + 3]) for i in range(0, len(data))]
    increased_new_list = [
        "increased" if depthness[i] > depthness[i - 1] else "not increased"
        for i in range(1, len(depthness))
    ]
    print(increased_new_list)
    print(increased_new_list.count("increased"))
