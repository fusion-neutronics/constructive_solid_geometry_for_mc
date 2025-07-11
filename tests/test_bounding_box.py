import constructive_solid_geometry_for_mc as csg4mc

def test_sphere_bb_moved_on_z_axis():
    s2 = csg4mc.Sphere(x0=0, y0=0, z0=1, r=3, surface_id=1)
    region2 = -s2
    bb = region2.bounding_box({s2.id: s2})
    assert bb.lower_left == [-3.0, -3.0, -2.0]
    assert bb.upper_right == [3.0, 3.0, 4.0]

# You can add more tests here using geometry from your examples
