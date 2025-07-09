from mycsg import Surface

s1 = Surface.Plane(0, 0, 1, 5)
s2 = Surface.Sphere((0, 0, 0), 3, id=1)
s3 = Surface.Cylinder((0,0,1), (0,0,0), 1,id=2)

region = -s1 & +s2 | ~(-s3)

inside = region.contains((0, 0, 0), {s1.id: s1, s2.id: s2, s3.id: s3})

print("Point inside region?", inside)
