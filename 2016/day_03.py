triangles = []

while True:
    try:
        triangles.append([int(i) for i in input().split()])
    except:
        break

count_1 = len(triangles)

for triangle in triangles:
    tr = sorted(triangle)
    if tr[0] + tr[1] <= tr[2]:
        count_1 -= 1

print('#1:', count_1)


count_2 = len(triangles)

for i in range(0, len(triangles), 3):
    for j in range(3):
        triangle = [triangles[i][j], triangles[i + 1][j], triangles[i + 2][j]]
        triangle.sort()
        if triangle[0] + triangle[1] <= triangle[2]:
            count_2 -= 1

print('#2:', count_2)
