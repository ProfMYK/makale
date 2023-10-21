import os
import sys
import random

n = 500
arr = []
for x in range(1000):
    arr.append([])
    arr[len(arr) - 1].append([])
    for y in range(n):
        r = random.randint(-500, 500)
        arr[len(arr) - 1][0].append(r)

    r = random.randint(-500, 500)
    arr[len(arr) - 1].append(r)
    arr[len(arr) - 1].append(r in arr[len(arr) - 1][0])

final = ""
for a in arr:
    st = ""
    for i in a[0]:
        st += str(i) + " "
    final += st[:-1] + "\n"

for a in arr:
    final += str(a[1]) + " "
    final += str(a[2]) + "\n"

final = final[:-1]

with open("input.txt", "w") as f:
    f.write(final)
