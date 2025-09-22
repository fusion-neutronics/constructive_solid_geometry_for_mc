import pytest
from constructive_solid_geometry_for_mc import XPlane, YPlane, ZPlane, Sphere, Cylinder, ZCylinder

def test_xplane_creation():
    s = XPlane(x0=1.0, surface_id=42)
    assert s.id == 42
    assert s.evaluate((1.0, 0.0, 0.0)) == pytest.approx(0.0)

def test_yplane_creation():
    s = YPlane(y0=2.0, surface_id=43)
    assert s.id == 43
    assert s.evaluate((0.0, 2.0, 0.0)) == pytest.approx(0.0)

def test_zplane_creation():
    s = ZPlane(z0=3.0, surface_id=44)
    assert s.id == 44
    assert s.evaluate((0.0, 0.0, 3.0)) == pytest.approx(0.0)

def test_sphere_creation():
    s = Sphere(x0=1.0, y0=2.0, z0=3.0, r=5.0, surface_id=45)
    assert s.id == 45
    assert s.evaluate((1.0, 2.0, 8.0)) == pytest.approx(0.0)

def test_cylinder_creation():
    s = Cylinder(axis=(0.0, 1.0, 0.0), origin=(1.0, 2.0, 3.0), r=2.0, surface_id=46)
    assert s.id == 46
    # Point at radius from origin, perpendicular to axis (Y axis)
    assert s.evaluate((3.0, 2.0, 3.0)) == pytest.approx(0.0)

def test_zcylinder_creation():
    s = ZCylinder(x0=1.0, y0=2.0, r=3.0, surface_id=47)
    assert s.id == 47
    # Point at radius from center in XY plane
    assert s.evaluate((4.0, 2.0, 0.0)) == pytest.approx(0.0)
