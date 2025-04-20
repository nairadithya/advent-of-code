# a
data = parse.(Int16, readlines("../input/01.txt"))
increased_new_list = [data[i] > data[i-1] ? 1 : 0 for i in 2:length(data)]
display(count(i -> i == 1, increased_new_list))

# b
depthness = [sum(data[i : i + 2]) for i in 1:(length(data)-2)]
increased_new_list = [depthness[i] > depthness[i-1] ? 1 : 0 for i in 2:length(depthness)]
display(count(i -> i == 1, increased_new_list))
