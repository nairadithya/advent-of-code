#!/usr/bin/env python3


# A
file = open("../input/01.txt", "r")
data = file.read().strip().split("\n")
data = [int(item) for item in data]
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
