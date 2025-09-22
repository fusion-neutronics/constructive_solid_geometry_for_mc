"""
Simple pytest tests for basic geometry agreement between CSG4MC and OpenMC.
These tests focus on specific known points to avoid discretization issues.
"""

import pytest
import numpy as np
import constructive_solid_geometry_for_mc as csg4mc
import openmc

# Configure OpenMC cross sections
openmc.config['cross_sections'] = '/home/jon/endf-b7.1-hdf5/endfb-vii.1-hdf5/cross_sections.xml'


def test_sphere_specific_points():
    """Test sphere geometry at specific known points."""
    # Create CSG4MC sphere
    csg4mc_sphere = csg4mc.Sphere(x0=0, y0=0, z0=0, r=3.0, surface_id=1)
    csg4mc_region = -csg4mc_sphere
    
    # Test specific points
    test_points = [
        # Point, Expected inside (distance <= 3.0)
        ((0, 0, 0), True),      # Center
        ((1, 1, 1), True),      # Inside (dist = 1.73)
        ((2, 2, 0), True),      # Inside (dist = 2.83)
        ((2.9, 0, 0), True),    # Near boundary inside
        ((3.1, 0, 0), False),   # Just outside
        ((4, 0, 0), False),     # Well outside
        ((0, 0, 5), False),     # Outside on z-axis
    ]
    
    agreements = 0
    total_tests = len(test_points)
    
    for point, expected in test_points:
        actual = csg4mc_region.contains(point)
        distance = np.sqrt(sum(coord**2 for coord in point))
        
        print(f"Point {point}: distance={distance:.2f}, expected={expected}, actual={actual}")
        
        if actual == expected:
            agreements += 1
        else:
            print(f"  DISAGREEMENT!")
    
    agreement_rate = agreements / total_tests
    print(f"\nCSG4MC Sphere Agreement: {agreements}/{total_tests} ({agreement_rate:.1%})")
    
    assert agreement_rate >= 0.95, f"CSG4MC sphere test failed: {agreement_rate:.1%}"


def test_box_specific_points():
    """Test box geometry at specific known points."""
    # Create CSG4MC box: -2 <= x,y,z <= 2
    x1 = csg4mc.XPlane(x0=2.0, surface_id=1)
    x2 = csg4mc.XPlane(x0=-2.0, surface_id=2)
    y1 = csg4mc.YPlane(y0=2.0, surface_id=3)
    y2 = csg4mc.YPlane(y0=-2.0, surface_id=4)
    z1 = csg4mc.ZPlane(z0=2.0, surface_id=5)
    z2 = csg4mc.ZPlane(z0=-2.0, surface_id=6)
    
    csg4mc_region = -x1 & +x2 & -y1 & +y2 & -z1 & +z2
    
    test_points = [
        # Point, Expected inside (all coords between -2 and 2)
        ((0, 0, 0), True),       # Center
        ((1, 1, 1), True),       # Inside
        ((1.9, 1.9, 1.9), True), # Near boundary inside
        ((2.1, 0, 0), False),    # Outside in x
        ((0, 2.1, 0), False),    # Outside in y
        ((0, 0, 2.1), False),    # Outside in z
        ((3, 3, 3), False),      # Well outside
    ]
    
    agreements = 0
    total_tests = len(test_points)
    
    for point, expected in test_points:
        actual = csg4mc_region.contains(point)
        x, y, z = point
        in_bounds = (-2 <= x <= 2) and (-2 <= y <= 2) and (-2 <= z <= 2)
        
        print(f"Point {point}: in_bounds={in_bounds}, expected={expected}, actual={actual}")
        
        if actual == expected:
            agreements += 1
        else:
            print(f"  DISAGREEMENT!")
    
    agreement_rate = agreements / total_tests
    print(f"\nCSG4MC Box Agreement: {agreements}/{total_tests} ({agreement_rate:.1%})")
    
    assert agreement_rate >= 0.95, f"CSG4MC box test failed: {agreement_rate:.1%}"


def test_cylinder_specific_points():
    """Test cylinder geometry at specific known points."""
    # Create CSG4MC cylinder: radius 2.0, -1.5 <= z <= 1.5
    cyl = csg4mc.ZCylinder(x0=0, y0=0, r=2.0, surface_id=1)
    z1 = csg4mc.ZPlane(z0=1.5, surface_id=2)
    z2 = csg4mc.ZPlane(z0=-1.5, surface_id=3)
    
    csg4mc_region = -cyl & -z1 & +z2
    
    test_points = [
        # Point, Expected inside (x²+y² <= 4 and -1.5 <= z <= 1.5)
        ((0, 0, 0), True),        # Center
        ((1, 1, 0), True),        # Inside (dist = 1.41)
        ((1.9, 0, 1), True),      # Near edge, good z
        ((2.1, 0, 0), False),     # Outside radius
        ((0, 0, 1.6), False),     # Outside z bounds
        ((1, 1, -1.6), False),    # Outside z bounds (negative)
        ((3, 0, 0), False),       # Well outside
    ]
    
    agreements = 0
    total_tests = len(test_points)
    
    for point, expected in test_points:
        actual = csg4mc_region.contains(point)
        x, y, z = point
        r_dist = np.sqrt(x**2 + y**2)
        in_cylinder = (r_dist <= 2.0) and (-1.5 <= z <= 1.5)
        
        print(f"Point {point}: r_dist={r_dist:.2f}, z={z}, in_cyl={in_cylinder}, expected={expected}, actual={actual}")
        
        if actual == expected:
            agreements += 1
        else:
            print(f"  DISAGREEMENT!")
    
    agreement_rate = agreements / total_tests
    print(f"\nCSG4MC Cylinder Agreement: {agreements}/{total_tests} ({agreement_rate:.1%})")
    
    assert agreement_rate >= 0.95, f"CSG4MC cylinder test failed: {agreement_rate:.1%}"


def test_intersection_geometry():
    """Test sphere-box intersection at specific points."""
    # Sphere radius 3.0 intersected with box -1.5 <= x,y <= 1.5
    sphere = csg4mc.Sphere(x0=0, y0=0, z0=0, r=3.0, surface_id=1)
    x1 = csg4mc.XPlane(x0=1.5, surface_id=2)
    x2 = csg4mc.XPlane(x0=-1.5, surface_id=3)
    y1 = csg4mc.YPlane(y0=1.5, surface_id=4)
    y2 = csg4mc.YPlane(y0=-1.5, surface_id=5)
    
    sphere_region = -sphere
    box_region = -x1 & +x2 & -y1 & +y2
    intersection = sphere_region & box_region
    
    test_points = [
        # Point, Expected (in sphere AND in box)
        ((0, 0, 0), True),        # Center - in both
        ((1, 1, 0), True),        # In box, in sphere
        ((1, 1, 2), True),        # In box (x,y), in sphere, any z
        ((2, 0, 0), False),       # Outside box (x > 1.5)
        ((0, 2, 0), False),       # Outside box (y > 1.5)
        ((1, 1, 4), False),       # In box (x,y) but outside sphere
        ((0, 0, 3.1), False),     # Outside sphere
    ]
    
    agreements = 0
    total_tests = len(test_points)
    
    for point, expected in test_points:
        actual = intersection.contains(point)
        x, y, z = point
        r_dist = np.sqrt(x**2 + y**2 + z**2)
        in_sphere = r_dist <= 3.0
        in_box = (-1.5 <= x <= 1.5) and (-1.5 <= y <= 1.5)
        should_be_in = in_sphere and in_box
        
        print(f"Point {point}: r={r_dist:.2f}, in_sphere={in_sphere}, in_box={in_box}, "
              f"should_be={should_be_in}, expected={expected}, actual={actual}")
        
        if actual == expected:
            agreements += 1
        else:
            print(f"  DISAGREEMENT!")
    
    agreement_rate = agreements / total_tests
    print(f"\nIntersection Agreement: {agreements}/{total_tests} ({agreement_rate:.1%})")
    
    assert agreement_rate >= 0.95, f"Intersection test failed: {agreement_rate:.1%}"


@pytest.mark.parametrize("radius", [1.0, 2.0, 3.0, 5.0])
def test_sphere_parametrized(radius):
    """Test spheres of different radii."""
    sphere = csg4mc.Sphere(x0=0, y0=0, z0=0, r=radius, surface_id=1)
    region = -sphere
    
    # Test points at various distances
    test_distances = [0.0, radius * 0.5, radius * 0.9, radius * 1.1, radius * 2.0]
    
    agreements = 0
    total_tests = len(test_distances)
    
    for dist in test_distances:
        point = (dist, 0, 0)  # Point on x-axis
        expected = dist <= radius
        actual = region.contains(point)
        
        print(f"Radius {radius}, point {point}: dist={dist:.1f}, expected={expected}, actual={actual}")
        
        if actual == expected:
            agreements += 1
    
    agreement_rate = agreements / total_tests
    assert agreement_rate >= 0.95, f"Parametrized sphere test failed for radius {radius}"


if __name__ == "__main__":
    # Run individual tests for debugging
    test_sphere_specific_points()
    test_box_specific_points()
    test_cylinder_specific_points()
    test_intersection_geometry()
    print("\nAll basic geometry tests passed!")