from mycsg import Surface

s1 = Surface.new_plane(0, 0, 1, 5, id=1)
s2 = Surface.sphere((0, 0, 0), 3, id=2)
s3 = Surface.cylinder((0,0,1), (0,0,0), 1, id=3)

region = -s1 & +s2 | ~(-s3)

inside = region.contains((0, 0, 0), {s1.id: s1, s2.id: s2, s3.id: s3})

print("Point inside region?", inside)
