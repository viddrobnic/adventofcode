discs_1 = [(13, 11), (5, 0), (17, 11), (3, 0), (7, 2), (19, 17)]
discs_2 = [(13, 11), (5, 0), (17, 11), (3, 0), (7, 2), (19, 17), (11, 0)]

time_1 = 0
time_2 = 0

solved_1 = False
solved_2 = False

while not solved_1 or not solved_2:
    result_1 = []
    result_2 = []
    if not solved_1:
        for i in range(len(discs_1)):
            disc = discs_1[i]
            result_1.append(((disc[1] + time_1 + i + 1) % disc[0]))
        if sum(result_1) == 0:
            print('#1:', time_1)
            solved_1 = True
        time_1 += 1

    if not solved_2:
        for i in range(len(discs_2)):
            disc = discs_2[i]
            result_2.append(((disc[1] + time_2 + i + 1) % disc[0]))
        if sum(result_2) == 0:
            print('#2:', time_2)
            solved_2 = True
    time_1 += 1
    time_2 += 1
