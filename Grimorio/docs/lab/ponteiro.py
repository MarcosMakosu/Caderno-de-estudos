x = 5

y = x 

x = 10

print(x, y)
print("--a & b--------------------------")

a = 3

b = a

print(a, b)
print(id(a), id(b))
print(a is b)
print("--C & D--------------------------")

c = [0, 1, 2, 3]

d = c

d.append(3)

print(c, d)
print(id(c), id(d))

