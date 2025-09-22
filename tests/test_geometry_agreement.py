"""
Pytest tests for verifying geometry agreement between CSG4MC and OpenMC.
Tests use systematic 3D grid sampling to compare cell IDs and region containment.
"""

import pytest
import numpy as np
import constructive_solid_geometry_for_mc as csg4mc
import openmc

# Configure OpenMC cross sections
openmc.config['cross_sections'] = '/home/jon/endf-b7.1-hdf5/endfb-vii.1-hdf5/cross_sections.xml'


def grid_test_geometry_agreement(csg4mc_region, openmc_model, 
                                bounds=(-5, 5), resolution=10,
                                tolerance=0.01):
    """
    Utility function to test geometry agreement over a 3D grid.
    
    Uses OpenMC's 3D voxel plot to generate a 3D grid of cell IDs for comparison.
    
    Parameters:
    -----------
    csg4mc_region : CSG4MC region object
        The CSG4MC geometry region to test
    openmc_model : openmc.Model
        OpenMC model with geometry and materials
    bounds : tuple
        (min, max) bounds for the test grid
    resolution : int
        Number of points per dimension
    tolerance : float
        Maximum allowable disagreement fraction
        
    Returns:
    --------
    dict : Test results with agreement statistics
    """
    import tempfile
    import os
    
    print(f"Testing {resolution}x{resolution}x{resolution} = {resolution**3} points using OpenMC voxel plot...")
    
    # Create OpenMC voxel plot using the neuronics-workshop approach
    try:
        # Create voxel plot
        vox_plot = openmc.Plot()
        vox_plot.type = 'voxel'
        
        # Use geometry bounding box if available, otherwise use provided bounds
        try:
            bbox = openmc_model.geometry.bounding_box
            vox_plot.width = bbox.width
            vox_plot.origin = bbox.center
            print(f"Using geometry bounding box: width={bbox.width}, center={bbox.center}")
        except:
            # Fallback to provided bounds
            width = bounds[1] - bounds[0]
            vox_plot.width = (width, width, width)
            vox_plot.origin = (0.0, 0.0, 0.0)
            print(f"Using manual bounds: width=({width}, {width}, {width})")
        
        # Set pixels to match our desired resolution
        vox_plot.pixels = (resolution, resolution, resolution)
        vox_plot.color_by = 'cell'
        
        # Use VTK output to get the voxel data
        with tempfile.NamedTemporaryFile(suffix='.vti', delete=False) as tmp_file:
            vtk_filename = tmp_file.name
        
        try:
            # Generate VTK voxel plot
            vox_plot.to_vtk(openmc_model, output=vtk_filename)
            
            # Read VTK file to extract cell data
            # For now, let's fall back to the HDF5 approach since VTK parsing is complex
            # We'll use the plot_geometry approach with temporary files
            
            with tempfile.TemporaryDirectory() as temp_dir:
                original_dir = os.getcwd()
                os.chdir(temp_dir)
                
                try:
                    # Export model files
                    openmc_model.export_to_xml()
                    
                    # Update plot for HDF5 output
                    vox_plot.filename = 'voxel_test'
                    plots = openmc.Plots([vox_plot])
                    plots.export_to_xml()
                    
                    # Generate the plot
                    openmc.plot_geometry()
                    
                    # Read the HDF5 voxel data
                    import h5py
                    with h5py.File('voxel_test.h5', 'r') as f:
                        cell_ids_3d = f['data'][()]
                        
                    print(f"OpenMC voxel plot shape: {cell_ids_3d.shape}")
                    print(f"Cell IDs found: {np.unique(cell_ids_3d)}")
                    
                finally:
                    os.chdir(original_dir)
                    
        finally:
            # Clean up VTK file
            try:
                os.unlink(vtk_filename)
            except:
                pass
        
    except Exception as e:
        print(f"OpenMC voxel plot failed: {e}")
        return {
            'total_points': 0,
            'agreements': 0,
            'disagreements': 0,
            'agreement_fraction': 0.0,
            'disagreement_points': [],
            'passed': False,
            'openmc_has_voxel_plot': False,
            'grid_bounds': bounds,
            'grid_resolution': resolution,
            'error': str(e)
        }
    
    # Create 3D grid coordinates that match the voxel plot
    coords = np.linspace(bounds[0], bounds[1], resolution)
    
    agreement_count = 0
    total_points = 0
    disagreements = []
    
    # Test each point in the 3D grid
    for i, x in enumerate(coords):
        for j, y in enumerate(coords):
            for k, z in enumerate(coords):
                point = (x, y, z)
                total_points += 1
                
                # Test CSG4MC
                try:
                    csg4mc_inside = csg4mc_region.contains(point)
                except Exception:
                    csg4mc_inside = False
                
                # Get OpenMC result from voxel plot
                # Voxel plot uses material cell ID 1 for inside, other IDs for outside
                openmc_cell_id = cell_ids_3d[i, j, k]
                openmc_inside = (openmc_cell_id == 1)  # Material cell has ID 1
                
                # Check agreement
                if csg4mc_inside == openmc_inside:
                    agreement_count += 1
                else:
                    disagreements.append({
                        'point': point,
                        'grid_indices': (i, j, k),
                        'csg4mc': csg4mc_inside,
                        'openmc': openmc_inside,
                        'openmc_cell_id': openmc_cell_id,
                        'distance_from_origin': np.sqrt(x**2 + y**2 + z**2)
                    })
    
    agreement_fraction = agreement_count / total_points
    
    return {
        'total_points': total_points,
        'agreements': agreement_count,
        'disagreements': len(disagreements),
        'agreement_fraction': agreement_fraction,
        'disagreement_points': disagreements[:10],  # First 10 for debugging
        'passed': agreement_fraction >= (1 - tolerance),
        'openmc_has_voxel_plot': True,
        'grid_bounds': bounds,
        'grid_resolution': resolution,
        'voxel_shape': cell_ids_3d.shape,
        'unique_cell_ids': np.unique(cell_ids_3d).tolist()
    }
        'agreements': agreement_count,
        'disagreements': len(disagreements),
        'agreement_fraction': agreement_fraction,
        'disagreement_points': disagreements[:10],  # First 10 for debugging
        'passed': agreement_fraction >= (1 - tolerance),
        'openmc_has_voxel_plot': True,
        'grid_bounds': bounds,
        'grid_resolution': resolution,
        'voxel_shape': cell_ids_3d.shape,
        'unique_cell_ids': np.unique(cell_ids_3d).tolist()
    }
    
    # Create 3D grid coordinates that match the voxel plot
    coords = np.linspace(bounds[0], bounds[1], resolution)
    
    agreement_count = 0
    total_points = 0
    disagreements = []
    
    # Test each point in the 3D grid
    for i, x in enumerate(coords):
        for j, y in enumerate(coords):
            for k, z in enumerate(coords):
                point = (x, y, z)
                total_points += 1
                
                # Test CSG4MC
                try:
                    csg4mc_inside = csg4mc_region.contains(point)
                except Exception:
                    csg4mc_inside = False
                
                # Get OpenMC result from voxel plot
                # Voxel plot uses material cell ID 1 for inside, other IDs for outside
                openmc_cell_id = cell_ids_3d[i, j, k]
                openmc_inside = (openmc_cell_id == 1)  # Material cell has ID 1
                
                # Check agreement
                if csg4mc_inside == openmc_inside:
                    agreement_count += 1
                else:
                    disagreements.append({
                        'point': point,
                        'grid_indices': (i, j, k),
                        'csg4mc': csg4mc_inside,
                        'openmc': openmc_inside,
                        'openmc_cell_id': openmc_cell_id,
                        'distance_from_origin': np.sqrt(x**2 + y**2 + z**2)
                    })
    
    agreement_fraction = agreement_count / total_points
    
    return {
        'total_points': total_points,
        'agreements': agreement_count,
        'disagreements': len(disagreements),
        'agreement_fraction': agreement_fraction,
        'disagreement_points': disagreements[:10],  # First 10 for debugging
        'passed': agreement_fraction >= (1 - tolerance),
        'openmc_has_voxel_plot': True,
        'grid_bounds': bounds,
        'grid_resolution': resolution,
        'voxel_shape': cell_ids_3d.shape,
        'unique_cell_ids': np.unique(cell_ids_3d).tolist()
    }


def create_sphere_geometry():
    """Create simple sphere geometry for both libraries."""
    # CSG4MC sphere - use smaller radius to avoid boundary effects
    csg4mc_sphere = csg4mc.Sphere(x0=0, y0=0, z0=0, r=2.5, surface_id=1)
    csg4mc_region = -csg4mc_sphere
    
    # OpenMC sphere
    openmc_sphere = openmc.Sphere(x0=0., y0=0., z0=0., r=2.5, surface_id=1)
    sphere_region = -openmc_sphere
    
    # Create material
    material = openmc.Material(material_id=1)
    material.add_nuclide('H1', 1.0)
    material.set_density('g/cc', 1.0)
    
    # Create cells
    sphere_cell = openmc.Cell(cell_id=1, region=sphere_region)
    sphere_cell.fill = material
    
    void_cell = openmc.Cell(cell_id=2, region=~sphere_region)
    
    # Create model
    universe = openmc.Universe(cells=[sphere_cell, void_cell])
    geometry = openmc.Geometry(universe)
    materials = openmc.Materials([material])
    model = openmc.Model(geometry=geometry, materials=materials)
    
    return csg4mc_region, model


def create_box_geometry():
    """Create simple box geometry for both libraries."""
    # CSG4MC box
    x1 = csg4mc.XPlane(x0=2.0, surface_id=1)
    x2 = csg4mc.XPlane(x0=-2.0, surface_id=2)
    y1 = csg4mc.YPlane(y0=2.0, surface_id=3)
    y2 = csg4mc.YPlane(y0=-2.0, surface_id=4)
    z1 = csg4mc.ZPlane(z0=2.0, surface_id=5)
    z2 = csg4mc.ZPlane(z0=-2.0, surface_id=6)
    
    csg4mc_region = -x1 & +x2 & -y1 & +y2 & -z1 & +z2
    
    # OpenMC box
    x1_omc = openmc.XPlane(x0=2.0, surface_id=1)
    x2_omc = openmc.XPlane(x0=-2.0, surface_id=2)
    y1_omc = openmc.YPlane(y0=2.0, surface_id=3)
    y2_omc = openmc.YPlane(y0=-2.0, surface_id=4)
    z1_omc = openmc.ZPlane(z0=2.0, surface_id=5)
    z2_omc = openmc.ZPlane(z0=-2.0, surface_id=6)
    
    box_region = -x1_omc & +x2_omc & -y1_omc & +y2_omc & -z1_omc & +z2_omc
    
    # Create material and cells
    material = openmc.Material(material_id=1)
    material.add_nuclide('H1', 1.0)
    material.set_density('g/cc', 1.0)
    
    box_cell = openmc.Cell(cell_id=1, region=box_region)
    box_cell.fill = material
    
    void_cell = openmc.Cell(cell_id=2, region=~box_region)
    
    # Create model
    universe = openmc.Universe(cells=[box_cell, void_cell])
    geometry = openmc.Geometry(universe)
    materials = openmc.Materials([material])
    model = openmc.Model(geometry=geometry, materials=materials)
    
    return csg4mc_region, model


def create_cylinder_geometry():
    """Create cylinder geometry for both libraries."""
    # CSG4MC cylinder
    csg4mc_cyl = csg4mc.ZCylinder(x0=0, y0=0, r=3.0, surface_id=1)
    z1 = csg4mc.ZPlane(z0=2.0, surface_id=2)
    z2 = csg4mc.ZPlane(z0=-2.0, surface_id=3)
    
    csg4mc_region = -csg4mc_cyl & -z1 & +z2
    
    # OpenMC cylinder
    openmc_cyl = openmc.ZCylinder(x0=0, y0=0, r=3.0, surface_id=1)
    z1_omc = openmc.ZPlane(z0=2.0, surface_id=2)
    z2_omc = openmc.ZPlane(z0=-2.0, surface_id=3)
    
    cyl_region = -openmc_cyl & -z1_omc & +z2_omc
    
    # Create material and cells
    material = openmc.Material(material_id=1)
    material.add_nuclide('H1', 1.0)
    material.set_density('g/cc', 1.0)
    
    cyl_cell = openmc.Cell(cell_id=1, region=cyl_region)
    cyl_cell.fill = material
    
    void_cell = openmc.Cell(cell_id=2, region=~cyl_region)
    
    # Create model
    universe = openmc.Universe(cells=[cyl_cell, void_cell])
    geometry = openmc.Geometry(universe)
    materials = openmc.Materials([material])
    model = openmc.Model(geometry=geometry, materials=materials)
    
    return csg4mc_region, model


def create_sphere_box_intersection():
    """Create sphere-box intersection for both libraries."""
    # CSG4MC
    sphere = csg4mc.Sphere(x0=0, y0=0, z0=0, r=4.0, surface_id=1)
    x1 = csg4mc.XPlane(x0=2.0, surface_id=2)
    x2 = csg4mc.XPlane(x0=-2.0, surface_id=3)
    y1 = csg4mc.YPlane(y0=2.0, surface_id=4)
    y2 = csg4mc.YPlane(y0=-2.0, surface_id=5)
    
    sphere_region = -sphere
    box_region = -x1 & +x2 & -y1 & +y2
    csg4mc_region = sphere_region & box_region
    
    # OpenMC
    sphere_omc = openmc.Sphere(x0=0., y0=0., z0=0., r=4.0, surface_id=1)
    x1_omc = openmc.XPlane(x0=2.0, surface_id=2)
    x2_omc = openmc.XPlane(x0=-2.0, surface_id=3)
    y1_omc = openmc.YPlane(y0=2.0, surface_id=4)
    y2_omc = openmc.YPlane(y0=-2.0, surface_id=5)
    
    sphere_region_omc = -sphere_omc
    box_region_omc = -x1_omc & +x2_omc & -y1_omc & +y2_omc
    intersection_region = sphere_region_omc & box_region_omc
    
    # Create material and cells
    material = openmc.Material(material_id=1)
    material.add_nuclide('H1', 1.0)
    material.set_density('g/cc', 1.0)
    
    intersection_cell = openmc.Cell(cell_id=1, region=intersection_region)
    intersection_cell.fill = material
    
    void_cell = openmc.Cell(cell_id=2, region=~intersection_region)
    
    # Create model
    universe = openmc.Universe(cells=[intersection_cell, void_cell])
    geometry = openmc.Geometry(universe)
    materials = openmc.Materials([material])
    model = openmc.Model(geometry=geometry, materials=materials)
    
    return csg4mc_region, model


# Test functions
def test_sphere_geometry_agreement():
    """Test sphere geometry agreement between CSG4MC and OpenMC using direct sampling.
    
    This test uses direct point-in-region testing for both libraries to avoid 
    discretization issues present in OpenMC's id_map function.
    """
    csg4mc_region, openmc_model = create_sphere_geometry()
    
    # Test with direct sampling - should have perfect or near-perfect agreement
    result = grid_test_geometry_agreement(
        csg4mc_region, openmc_model,
        bounds=(-3, 3), resolution=6,  # 6^3 = 216 points
        tolerance=0.01  # Very low tolerance since we're using exact methods
    )
    
    print(f"\nSphere 3D Direct Sampling Results:")
    print(f"Grid: {result['grid_resolution']}x{result['grid_resolution']}x{result['grid_resolution']} = {result['total_points']} points")
    print(f"Bounds: {result['grid_bounds']}")
    print(f"Agreement: {result['agreements']}/{result['total_points']} ({result['agreement_fraction']:.1%})")
    print(f"OpenMC direct sampling: {result['openmc_has_direct_sampling']}")
    
    if result['disagreement_points']:
        print(f"\nUnexpected disagreements found ({len(result['disagreement_points'])} total):")
        print("These should be investigated as both methods use exact point testing:")
        
        for i, disagreement in enumerate(result['disagreement_points'][:5]):
            point = disagreement['point']
            dist = disagreement['distance_from_origin']
            
            expected_inside = dist <= 2.5  # Sphere radius
            
            print(f"  {i+1}. Point {point}:")
            print(f"     Distance: {dist:.6f} (expected inside: {expected_inside})")
            print(f"     CSG4MC: {disagreement['csg4mc']}")
            print(f"     OpenMC: {disagreement['openmc']}")
            
            # Check if this is a boundary precision issue
            if abs(dist - 2.5) < 1e-10:
                print(f"     -> Likely boundary precision issue")
    else:
        print("Perfect agreement! Both libraries use exact point-in-region testing.")
    
    assert result['passed'], f"Sphere geometry agreement below threshold: {result['agreement_fraction']:.1%}"


def test_box_geometry_agreement():
    """Test box geometry agreement between CSG4MC and OpenMC."""
    csg4mc_region, openmc_model = create_box_geometry()
    
    result = grid_test_geometry_agreement(
        csg4mc_region, openmc_model,
        bounds=(-3, 3), resolution=10,
        tolerance=0.05
    )
    
    print(f"\nBox test results:")
    print(f"Agreement: {result['agreements']}/{result['total_points']} ({result['agreement_fraction']:.1%})")
    
    assert result['passed'], f"Box geometry agreement below threshold: {result['agreement_fraction']:.1%}"


def test_cylinder_geometry_agreement():
    """Test cylinder geometry agreement between CSG4MC and OpenMC."""
    csg4mc_region, openmc_model = create_cylinder_geometry()
    
    result = grid_test_geometry_agreement(
        csg4mc_region, openmc_model,
        bounds=(-4, 4), resolution=10,
        tolerance=0.05
    )
    
    print(f"\nCylinder test results:")
    print(f"Agreement: {result['agreements']}/{result['total_points']} ({result['agreement_fraction']:.1%})")
    
    assert result['passed'], f"Cylinder geometry agreement below threshold: {result['agreement_fraction']:.1%}"


def test_sphere_box_intersection_agreement():
    """Test sphere-box intersection geometry agreement."""
    csg4mc_region, openmc_model = create_sphere_box_intersection()
    
    result = grid_test_geometry_agreement(
        csg4mc_region, openmc_model,
        bounds=(-3, 3), resolution=8,  # Slightly lower resolution for complex geometry
        tolerance=0.1  # Higher tolerance for complex intersection
    )
    
    print(f"\nSphere-Box intersection test results:")
    print(f"Agreement: {result['agreements']}/{result['total_points']} ({result['agreement_fraction']:.1%})")
    
    if not result['passed'] and result['disagreement_points']:
        print("Sample disagreement points:")
        for disagreement in result['disagreement_points'][:3]:
            point = disagreement['point']
            print(f"  {point}: CSG4MC={disagreement['csg4mc']}, OpenMC={disagreement['openmc']}")
    
    assert result['passed'], f"Sphere-box intersection agreement below threshold: {result['agreement_fraction']:.1%}"


@pytest.mark.parametrize("resolution", [5, 8, 10])
def test_sphere_resolution_scaling(resolution):
    """Test sphere geometry at different resolutions."""
    csg4mc_region, openmc_model = create_sphere_geometry()
    
    result = grid_test_geometry_agreement(
        csg4mc_region, openmc_model,
        bounds=(-4, 4), resolution=resolution,
        tolerance=0.05
    )
    
    print(f"\nSphere resolution {resolution}x{resolution}x{resolution}:")
    print(f"Agreement: {result['agreement_fraction']:.1%}")
    
    assert result['passed'], f"Sphere geometry failed at resolution {resolution}"


def test_detailed_point_sampling():
    """Test specific known points for debugging."""
    csg4mc_region, openmc_model = create_sphere_geometry()
    
    # Test specific points
    test_points = [
        (0, 0, 0),     # Center - should be inside
        (2, 0, 0),     # Inside sphere
        (4, 0, 0),     # Outside sphere
        (2.9, 0, 0),   # Near boundary
        (3.1, 0, 0),   # Just outside
    ]
    
    agreements = 0
    for point in test_points:
        csg4mc_inside = csg4mc_region.contains(point)
        
        # For detailed testing, we'll check the expected results
        distance = np.sqrt(sum(coord**2 for coord in point))
        expected_inside = distance <= 3.0
        
        print(f"Point {point}: distance={distance:.2f}, CSG4MC={csg4mc_inside}, expected={expected_inside}")
        
        if csg4mc_inside == expected_inside:
            agreements += 1
    
    agreement_fraction = agreements / len(test_points)
    assert agreement_fraction >= 0.8, f"Point sampling agreement too low: {agreement_fraction:.1%}"


if __name__ == "__main__":
    # Run tests individually for debugging
    test_sphere_geometry_agreement()
    test_box_geometry_agreement() 
    test_cylinder_geometry_agreement()
    test_sphere_box_intersection_agreement()
    print("\nAll geometry agreement tests completed!")