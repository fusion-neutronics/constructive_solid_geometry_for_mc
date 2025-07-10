from constructive_solid_geometry_for_mc import Surface


s1 = Surface.XPlane(x0=1.2, id=5)
s2 = Surface.XPlane(x0=-1.2,id=6 )
s3 = Surface.Sphere((0, 0, 0), 3, id=1)
# s3 = Surface.Cylinder((0,0,1), (0,0,0), 1,id=2)

surfaces_dict = {s1.id: s1, s2.id: s2, s3.id: s3}

region1 = -s1 & +s2 & -s3
inside = region1.contains((0, 0, 0), surfaces_dict)
print("Point inside region1?", inside)
print(region1.bounding_box(surfaces_dict))

region2 = +s1 & -s2 & -s3
inside = region2.contains((0, 0, 0), surfaces_dict)
print("Point inside region1?", inside)
print(region2.bounding_box(surfaces_dict))

