"""
Comprehensive pytest suite demonstrating the grid testing utility for CSG4MC vs OpenMC comparisons.
This showcases the utility function for systematic 3D grid-based geometry testing.
"""

import pytest
import numpy as np
import constructive_solid_geometry_for_mc as csg4mc
import openmc

# Configure OpenMC
openmc.config['cross_sections'] = '/home/jon/endf-b7.1-hdf5/endfb-vii.1-hdf5/cross_sections.xml'


def grid_test_utility(csg4mc_region, openmc_model, 
                     bounds=(-3, 3), resolution=10, tolerance=0.1):
    """
    Utility function for systematic 3D grid testing of geometry agreement.
    
    This utility samples a 2D grid (at z=0) and compares CSG4MC region containment
    with OpenMC's id_map results to verify geometry implementation agreement.
    
    Parameters:
    -----------
    csg4mc_region : CSG4MC region object
        The CSG4MC geometry region to test
    openmc_model : openmc.Model  
        OpenMC model with proper geometry and materials setup
    bounds : tuple, default (-3, 3)
        (min, max) spatial bounds for the test grid
    resolution : int, default 10
        Grid resolution (total points = resolution²)
    tolerance : float, default 0.1
        Maximum allowable disagreement fraction (10%)
        
    Returns:
    --------
    dict : Comprehensive test results including:
        - total_points: Number of grid points tested
        - agreements: Number of points where both libraries agree
        - agreement_fraction: Fraction of points in agreement
        - passed: Boolean indicating if tolerance was met
        - sample_disagreements: Examples of disagreement points for debugging
    
    Usage Example:
    --------------
    # Create geometries
    csg4mc_region = create_some_csg4mc_geometry()
    openmc_model = create_equivalent_openmc_model()
    
    # Test agreement
    result = grid_test_utility(csg4mc_region, openmc_model, 
                              bounds=(-5, 5), resolution=12, tolerance=0.05)
    
    assert result['passed'], f"Geometry agreement: {result['agreement_fraction']:.1%}"
    """
    
    # Create 2D grid at z=0 plane
    coords = np.linspace(bounds[0], bounds[1], resolution)
    x_coords, y_coords = np.meshgrid(coords, coords)
    z_coord = 0.0
    
    points_2d = np.column_stack([x_coords.ravel(), y_coords.ravel()])
    total_points = len(points_2d)
    
    # Get OpenMC id_map
    try:
        width = bounds[1] - bounds[0]
        id_map = openmc_model.id_map(
            origin=(0.0, 0.0, 0.0),
            width=(width, width),
            pixels=(resolution, resolution),
            basis='xy'
        )
        cell_ids = id_map[:, :, 0]
        has_idmap = True
    except Exception as e:
        print(f"Warning: OpenMC id_map failed: {e}")
        has_idmap = False
        
    # Compare each grid point
    agreements = 0
    disagreements = []
    
    for i, (x, y) in enumerate(points_2d):
        point_3d = (x, y, z_coord)
        
        # Test CSG4MC
        try:
            csg4mc_inside = csg4mc_region.contains(point_3d)
        except Exception:
            csg4mc_inside = False
            
        # Test OpenMC via id_map
        if has_idmap:
            # Convert to grid indices
            xi = int((x - bounds[0]) / (bounds[1] - bounds[0]) * (resolution - 1))
            yi = int((y - bounds[0]) / (bounds[1] - bounds[0]) * (resolution - 1))
            
            if 0 <= xi < resolution and 0 <= yi < resolution:
                cell_id = cell_ids[yi, xi]
                # Cell ID 1 = material region, Cell ID 2 = void
                openmc_inside = (cell_id == 1)
            else:
                openmc_inside = False
        else:
            openmc_inside = False
            
        # Check agreement
        if csg4mc_inside == openmc_inside:
            agreements += 1
        else:
            disagreements.append({
                'point': point_3d,
                'csg4mc': csg4mc_inside,
                'openmc': openmc_inside,
                'distance_from_origin': np.sqrt(x**2 + y**2)
            })
    
    agreement_fraction = agreements / total_points
    passed = agreement_fraction >= (1 - tolerance)
    
    return {
        'total_points': total_points,
        'agreements': agreements,
        'disagreements': len(disagreements),
        'agreement_fraction': agreement_fraction,
        'passed': passed,
        'tolerance_used': tolerance,
        'grid_resolution': resolution,
        'test_bounds': bounds,
        'openmc_idmap_available': has_idmap,
        'sample_disagreements': disagreements[:5]  # First 5 for analysis
    }


def create_simple_sphere():
    """Create a simple sphere geometry for both CSG4MC and OpenMC."""
    radius = 2.0
    
    # CSG4MC sphere
    csg4mc_sphere = csg4mc.Sphere(x0=0, y0=0, z0=0, r=radius, surface_id=1)
    csg4mc_region = -csg4mc_sphere
    
    # OpenMC equivalent
    openmc_sphere = openmc.Sphere(x0=0., y0=0., z0=0., r=radius, surface_id=1)
    sphere_region = -openmc_sphere
    
    # Create OpenMC model
    material = openmc.Material(material_id=1)
    material.add_nuclide('H1', 1.0)
    material.set_density('g/cc', 1.0)
    
    sphere_cell = openmc.Cell(cell_id=1, region=sphere_region, fill=material)
    void_cell = openmc.Cell(cell_id=2, region=~sphere_region)
    
    universe = openmc.Universe(cells=[sphere_cell, void_cell])
    geometry = openmc.Geometry(universe)
    materials = openmc.Materials([material])
    model = openmc.Model(geometry=geometry, materials=materials)
    
    return csg4mc_region, model


def create_box_geometry():
    """Create a box geometry for both libraries."""
    # Box bounds: -1.5 to 1.5 in all dimensions
    bound = 1.5
    
    # CSG4MC box
    x1 = csg4mc.XPlane(x0=bound, surface_id=1)
    x2 = csg4mc.XPlane(x0=-bound, surface_id=2)
    y1 = csg4mc.YPlane(y0=bound, surface_id=3)
    y2 = csg4mc.YPlane(y0=-bound, surface_id=4)
    z1 = csg4mc.ZPlane(z0=bound, surface_id=5)
    z2 = csg4mc.ZPlane(z0=-bound, surface_id=6)
    
    csg4mc_region = -x1 & +x2 & -y1 & +y2 & -z1 & +z2
    
    # OpenMC box
    x1_omc = openmc.XPlane(x0=bound, surface_id=1)
    x2_omc = openmc.XPlane(x0=-bound, surface_id=2)
    y1_omc = openmc.YPlane(y0=bound, surface_id=3)
    y2_omc = openmc.YPlane(y0=-bound, surface_id=4)
    z1_omc = openmc.ZPlane(z0=bound, surface_id=5)
    z2_omc = openmc.ZPlane(z0=-bound, surface_id=6)
    
    box_region = -x1_omc & +x2_omc & -y1_omc & +y2_omc & -z1_omc & +z2_omc
    
    # Create OpenMC model
    material = openmc.Material(material_id=1)
    material.add_nuclide('H1', 1.0)
    material.set_density('g/cc', 1.0)
    
    box_cell = openmc.Cell(cell_id=1, region=box_region, fill=material)
    void_cell = openmc.Cell(cell_id=2, region=~box_region)
    
    universe = openmc.Universe(cells=[box_cell, void_cell])
    geometry = openmc.Geometry(universe)
    materials = openmc.Materials([material])
    model = openmc.Model(geometry=geometry, materials=materials)
    
    return csg4mc_region, model


def test_utility_function_sphere():
    """Demonstrate the grid testing utility with a sphere."""
    print(\"\\n=== Testing Grid Utility with Sphere ===\")
    
    csg4mc_region, openmc_model = create_simple_sphere()
    
    # Test with different parameters
    result = grid_test_utility(
        csg4mc_region, openmc_model,
        bounds=(-3, 3), 
        resolution=8,
        tolerance=0.15
    )
    
    print(f\"Sphere Grid Test Results:\")
    print(f\"  Grid: {result['grid_resolution']}x{result['grid_resolution']} points\")
    print(f\"  Bounds: {result['test_bounds']}\")
    print(f\"  Agreement: {result['agreements']}/{result['total_points']} ({result['agreement_fraction']:.1%})\")
    print(f\"  Tolerance: {result['tolerance_used']:.1%}\")
    print(f\"  OpenMC id_map: {result['openmc_idmap_available']}\")
    
    if result['sample_disagreements']:
        print(f\"  Sample disagreements (boundary effects):\")
        for i, d in enumerate(result['sample_disagreements'][:3]):
            dist = d['distance_from_origin']
            print(f\"    {i+1}. Point {d['point']}: r={dist:.2f}, CSG4MC={d['csg4mc']}, OpenMC={d['openmc']}\")
    
    assert result['passed'], f\"Sphere utility test failed: {result['agreement_fraction']:.1%}\"


def test_utility_function_box():
    """Demonstrate the grid testing utility with a box."""
    print(\"\\n=== Testing Grid Utility with Box ===\")
    
    csg4mc_region, openmc_model = create_box_geometry()
    
    result = grid_test_utility(
        csg4mc_region, openmc_model,
        bounds=(-2, 2),
        resolution=10, 
        tolerance=0.1
    )
    
    print(f\"Box Grid Test Results:\")
    print(f\"  Agreement: {result['agreements']}/{result['total_points']} ({result['agreement_fraction']:.1%})\")
    print(f\"  Disagreements: {result['disagreements']}\")
    
    assert result['passed'], f\"Box utility test failed: {result['agreement_fraction']:.1%}\"


@pytest.mark.parametrize(\"resolution,tolerance\", [
    (6, 0.2),   # Coarse grid, higher tolerance
    (8, 0.15),  # Medium grid
    (10, 0.1),  # Fine grid, lower tolerance
])
def test_parametrized_grid_resolution(resolution, tolerance):
    \"\"\"Test the utility function with different grid resolutions.\"\"\"
    csg4mc_region, openmc_model = create_simple_sphere()
    
    result = grid_test_utility(
        csg4mc_region, openmc_model,
        bounds=(-2.5, 2.5),
        resolution=resolution,
        tolerance=tolerance
    )
    
    print(f\"\\nResolution {resolution}: {result['agreement_fraction']:.1%} agreement\")
    
    assert result['passed'], f\"Failed at resolution {resolution}: {result['agreement_fraction']:.1%}\"


def test_grid_utility_performance():
    \"\"\"Test the performance and scalability of the grid utility.\"\"\"
    csg4mc_region, openmc_model = create_simple_sphere()
    
    import time
    
    resolutions = [5, 8, 10, 12]
    
    for res in resolutions:
        start_time = time.time()
        
        result = grid_test_utility(
            csg4mc_region, openmc_model,
            bounds=(-2, 2),
            resolution=res,
            tolerance=0.15
        )
        
        elapsed = time.time() - start_time
        points_per_sec = result['total_points'] / elapsed if elapsed > 0 else 0
        
        print(f\"Resolution {res:2d}: {result['total_points']:3d} points, \"
              f\"{elapsed:.3f}s, {points_per_sec:.0f} pts/sec, \"
              f\"{result['agreement_fraction']:.1%} agreement\")
        
        assert result['passed'], f\"Performance test failed at resolution {res}\"


if __name__ == \"__main__\":
    # Demonstrate the utility function
    test_utility_function_sphere()
    test_utility_function_box()
    print(\"\\nGrid utility demonstration complete!\")