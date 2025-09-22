import numpy as np
import openmc
import tempfile
import os
import h5py


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
    print(f"Testing {resolution}x{resolution}x{resolution} = {resolution**3} points using OpenMC voxel plot...")
    
    # Create OpenMC voxel plot using the neuronics-workshop approach
    try:
        # Create voxel plot
        vox_plot = openmc.Plot()
        vox_plot.type = 'voxel'
        
        # Use manual bounds for better control
        width = bounds[1] - bounds[0]
        vox_plot.width = (width, width, width)
        vox_plot.origin = (0.0, 0.0, 0.0)
        print(f"Using manual bounds: width=({width}, {width}, {width}), origin=(0.0, 0.0, 0.0)")
        
        # Set pixels to match our desired resolution
        vox_plot.pixels = (resolution, resolution, resolution)
        vox_plot.color_by = 'cell'
        
        # Generate voxel plot using HDF5 output
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
                with h5py.File('voxel_test.h5', 'r') as f:
                    cell_ids_3d = f['data'][()]
                    
                print(f"OpenMC voxel plot shape: {cell_ids_3d.shape}")
                print(f"Cell IDs found: {np.unique(cell_ids_3d)}")
                
            finally:
                os.chdir(original_dir)
        
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


def create_sphere_geometry(radius=2.5):
    """Create matched CSG4MC and OpenMC sphere geometries for testing."""
    # CSG4MC sphere
    import constructive_solid_geometry_for_mc as csg4mc
    csg4mc_sphere = csg4mc.Sphere(0., 0., 0., radius, 1)
    
    # OpenMC sphere geometry
    sphere_surf = openmc.Sphere(x0=0., y0=0., z0=0., r=radius, surface_id=1)
    sphere_region = -sphere_surf
    
    material = openmc.Material(material_id=1)
    material.add_nuclide('H1', 1.0)
    material.set_density('g/cc', 1.0)
    
    sphere_cell = openmc.Cell(cell_id=1, region=sphere_region, fill=material)
    void_cell = openmc.Cell(cell_id=2, region=~sphere_region)
    
    universe = openmc.Universe(cells=[sphere_cell, void_cell])
    geometry = openmc.Geometry(universe)
    materials = openmc.Materials([material])
    model = openmc.Model(geometry=geometry, materials=materials)
    
    return csg4mc_sphere, model