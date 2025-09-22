"""
Example comparing CSG4MC and OpenMC geometries with plotting.
This example creates a sphere intersected with a box using both libraries and plots them side by side.
"""

import numpy as np
import matplotlib.pyplot as plt
import constructive_solid_geometry_for_mc as csg4mc
import openmc

openmc.config['cross_sections'] = '/home/jon/endf-b7.1-hdf5/endfb-vii.1-hdf5/cross_sections.xml'

def create_csg4mc_geometry():
    """Create geometry using CSG4MC package."""
    # Create surfaces
    s1 = csg4mc.XPlane(x0=2.1, surface_id=1)   # x = 2.1
    s2 = csg4mc.XPlane(x0=-2.1, surface_id=2)  # x = -2.1
    s3 = csg4mc.YPlane(y0=2.1, surface_id=3)   # y = 2.1  
    s4 = csg4mc.YPlane(y0=-2.1, surface_id=4)  # y = -2.1
    s5 = csg4mc.Sphere(x0=0, y0=0, z0=0, r=4.2, surface_id=5)  # sphere
    
    # Create region: inside box AND inside sphere
    box_region = -s1 & +s2 & -s3 & +s4  # -2.1 < x < 2.1 AND -2.1 < y < 2.1
    sphere_region = -s5  # inside sphere
    combined_region = box_region & sphere_region
    
    return combined_region

def create_openmc_geometry():
    """Create equivalent geometry using OpenMC."""
    # Create surfaces with same IDs for comparison
    s1 = openmc.XPlane(x0=2.1, surface_id=1)
    s2 = openmc.XPlane(x0=-2.1, surface_id=2)
    s3 = openmc.YPlane(y0=2.1, surface_id=3)
    s4 = openmc.YPlane(y0=-2.1, surface_id=4)
    s5 = openmc.Sphere(x0=0., y0=0., z0=0., r=4.2, surface_id=5)
    
    # Create region: inside box AND inside sphere
    box_region = -s1 & +s2 & -s3 & +s4
    sphere_region = -s5
    combined_region = box_region & sphere_region
    
    # Create cells with proper material assignment
    # Cell 1: The intersection region (with material)
    material = openmc.Material(material_id=1)
    material.add_nuclide('H1', 1.0)
    material.set_density('g/cc', 1.0)
    
    intersection_cell = openmc.Cell(cell_id=1, region=combined_region)
    intersection_cell.fill = material
    
    # Cell 2: Everything else (void)
    void_region = ~combined_region
    void_cell = openmc.Cell(cell_id=2, region=void_region)
    
    # Create universe and geometry
    universe = openmc.Universe(cells=[intersection_cell, void_cell])
    geometry = openmc.Geometry(universe)
    
    # Create materials collection
    materials = openmc.Materials([material])
    
    # Create model
    model = openmc.Model(geometry=geometry, materials=materials)
    
    return model, combined_region

def plot_csg4mc_geometry(region, ax, title="CSG4MC Geometry"):
    """Plot CSG4MC geometry by sampling points."""
    # Create grid of points
    x = np.linspace(-5, 5, 200)
    y = np.linspace(-5, 5, 200)
    X, Y = np.meshgrid(x, y)
    
    # Sample the geometry
    inside_map = np.zeros_like(X, dtype=bool)
    for i in range(X.shape[0]):
        for j in range(X.shape[1]):
            inside_map[i, j] = region.contains((X[i, j], Y[i, j], 0.0))
    
    # Plot
    ax.imshow(inside_map, extent=[-5, 5, -5, 5], origin='lower', 
              cmap='RdYlBu', alpha=0.8)
    ax.set_xlabel('X')
    ax.set_ylabel('Y')
    ax.set_title(title)
    ax.grid(True, alpha=0.3)
    ax.set_aspect('equal')

def plot_openmc_geometry(model, region, ax, title="OpenMC Geometry"):
    """Plot OpenMC geometry using the id_map feature."""
    try:
        # Use OpenMC's id_map feature
        id_map = model.id_map(
            origin=(0.0, 0.0, 0.0),
            width=(10.0, 10.0),
            pixels=(200, 200),
            basis='xy'
        )
        
        # Extract cell IDs (first channel)
        cell_ids = id_map[:, :, 0]
        
        # Create a binary mask (inside geometry = cell ID 1, outside = cell ID 2)
        inside_map = cell_ids == 1
        
        ax.imshow(inside_map, extent=[-5, 5, -5, 5], origin='lower',
                  cmap='RdYlBu', alpha=0.8)
                  
    except Exception as e:
        print(f"OpenMC plotting failed: {e}")
        print("Falling back to manual sampling...")
        
        # Fallback: manual sampling like CSG4MC
        x = np.linspace(-5, 5, 200)
        y = np.linspace(-5, 5, 200)
        X, Y = np.meshgrid(x, y)
        
        inside_map = np.zeros_like(X, dtype=bool)
        for i in range(X.shape[0]):
            for j in range(X.shape[1]):
                try:
                    inside_map[i, j] = (X[i, j], Y[i, j], 0.0) in region
                except:
                    inside_map[i, j] = False
        
        ax.imshow(inside_map, extent=[-5, 5, -5, 5], origin='lower',
                  cmap='RdYlBu', alpha=0.8)
    
    ax.set_xlabel('X')
    ax.set_ylabel('Y')
    ax.set_title(title)
    ax.grid(True, alpha=0.3)
    ax.set_aspect('equal')

def main():
    """Main function to create and plot both geometries."""
    print("Creating CSG4MC geometry...")
    csg4mc_region = create_csg4mc_geometry()
    
    print("Creating OpenMC geometry...")
    openmc_model, openmc_region = create_openmc_geometry()
    
    print("Creating plots...")
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(12, 5))
    
    # Plot CSG4MC geometry
    plot_csg4mc_geometry(csg4mc_region, ax1, "CSG4MC: Sphere ∩ Box")
    
    # Plot OpenMC geometry
    plot_openmc_geometry(openmc_model, openmc_region, ax2, "OpenMC: Sphere ∩ Box")
    
    plt.tight_layout()
    plt.savefig('geometry_comparison_sphere_box.png', dpi=150, bbox_inches='tight')
    plt.show()
    
    # Print some test points
    print("\nTesting some points:")
    test_points = [(0, 0, 0), (1, 1, 0), (3, 3, 0), (2.05, 1.0, 0)]
    for point in test_points:
        csg4mc_result = csg4mc_region.contains(point)
        try:
            openmc_result = point in openmc_region
        except:
            openmc_result = "Error"
        print(f"Point {point}: CSG4MC={csg4mc_result}, OpenMC={openmc_result}")

if __name__ == "__main__":
    main()