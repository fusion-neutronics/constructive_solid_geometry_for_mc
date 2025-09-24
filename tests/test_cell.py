import pytest
import constructive_solid_geometry_for_mc as csg

def test_cell_contains_simple():
    # Sphere of radius 2 at (0,0,0)
    sphere = csg.Sphere(surface_id=1, x0=0.0, y0=0.0, z0=0.0, r=2.0, boundary_type='vacuum')
    region = -sphere
    cell = csg.Cell(cell_id=1, region=region, name=None)
    assert cell.contains(0.0, 0.0, 0.0)
    assert not cell.contains(3.0, 0.0, 0.0)

def test_cell_naming():
    sphere = csg.Sphere(surface_id=101, x0=0.0, y0=0.0, z0=0.0, r=2.0, boundary_type='vacuum')
    region = -sphere
    cell = csg.Cell(cell_id=222, region=region, name="fuel")
    assert cell.name == "fuel"
    assert cell.cell_id == 222

def test_cell_complex_region():
    # s1: x = 2.1, s2: x = -2.1, s3: sphere at origin, r=4.2
    s1 = csg.XPlane(x0=2.1, surface_id=5)
    s2 = csg.XPlane(x0=-2.1, surface_id=6)
    s3 = csg.Sphere(x0=0.0, y0=0.0, z0=0.0, r=4.2, surface_id=1)
    region = -s1 & +s2 & -s3
    cell = csg.Cell(cell_id=42, region=region, name="complex")
    # Point inside all constraints
    assert cell.contains(0.0, 0.0, 0.0)
    # Point outside s1 (x > 2.1)
    assert not cell.contains(3.0, 0.0, 0.0)
    # Point outside s2 (x < -2.1)
    assert not cell.contains(-3.0, 0.0, 0.0)
    # Point outside sphere (r > 4.2)
    assert not cell.contains(0.0, 0.0, 5.0)
