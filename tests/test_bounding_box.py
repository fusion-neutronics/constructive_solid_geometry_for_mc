import constructive_solid_geometry_for_mc as csg4mc

def test_sphere_bb_moved_on_z_axis():
    s2 = csg4mc.Sphere(x0=0, y0=0, z0=1, r=3, surface_id=1)
    region2 = -s2
    bb = region2.bounding_box()
    assert bb.lower_left == [-3.0, -3.0, -2.0]
    assert bb.upper_right == [3.0, 3.0, 4.0]

def test_sphere_with_xplanes():
    s1 = csg4mc.XPlane(x0=2.1, surface_id=5)
    s2 = csg4mc.XPlane(x0=-2.1, surface_id=6)
    s3 = csg4mc.Sphere(x0=0, y0=0, z0=0, r=4.2, surface_id=1)

    region1 = -s1 & +s2 & -s3
    assert region1.contains((0, 0, 0))
    bb = region1.bounding_box()
    assert bb.lower_left == [-2.1, -4.2, -4.2]
    assert bb.upper_right == [2.1, 4.2, 4.2]