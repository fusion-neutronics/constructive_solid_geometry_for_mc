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
