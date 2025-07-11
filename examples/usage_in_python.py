import constructive_solid_geometry_for_mc as csg4mc


s1 = csg4mc.Plane(0, 0, 1, 5)
s2 = csg4mc.Sphere((0, 0, 0), 3, surface_id=1)
s3 = csg4mc.Cylinder((0,0,1), (0,0,0), 1,surface_id=2)

region1 = -s1 & +s2 | ~(-s3)

inside = region1.contains((0, 0, 0), {s1.id: s1, s2.id: s2, s3.id: s3})

print("Point inside region1?", inside)

s4 = csg4mc.XPlane(1.0, surface_id=10)
region2 = -s2

inside = region2.contains((0, 0, 0), {s2.id: s2})

print("Point inside region2?", inside)

bb = region2.bounding_box({s2.id: s2})
print("Bounding box of region2:", bb.lower_left_corner, bb.upper_right_corner)

print(f'Bounding box center {bb.center}')

print(f"bb width {bb.width}")


import numpy as np

results = []
for x in np.linspace(
    bb.lower_left_corner[0], bb.upper_right_corner[0], 10
):
    for y in np.linspace(
        bb.lower_left_corner[1], bb.upper_right_corner[1], 10
    ):
        contains = region2.contains((x, y, 0), {s2.id: s2})
        # print(f"Point ({x}, {y}, 0) inside region2? {contains}")
        results.append(int(contains))

results_np = np.array(results).reshape((10, 10))
print(results_np)
# import matplotlib.pyplot as plt

# plt.imshow(results_np)
# plt.show()