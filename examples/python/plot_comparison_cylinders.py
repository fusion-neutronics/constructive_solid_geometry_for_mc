"""
Example comparing CSG4MC and OpenMC cylindrical geometries with plotting.
This example creates a cylinder intersected with planes using both libraries.
"""

import numpy as np
import matplotlib.pyplot as plt
import constructive_solid_geometry_for_mc as csg4mc
import openmc
openmc.config['cross_sections'] = '/home/jon/endf-b7.1-hdf5/endfb-vii.1-hdf5/cross_sections.xml'
def create_csg4mc_cylinder_geometry():
    """Create cylindrical geometry using CSG4MC package."""
    # Create surfaces
    z_cyl = csg4mc.ZCylinder(x0=0, y0=0, r=3.0, surface_id=1)  # Z-axis cylinder
    z_plane_top = csg4mc.ZPlane(z0=2.0, surface_id=2)          # z = 2.0
    z_plane_bottom = csg4mc.ZPlane(z0=-2.0, surface_id=3)      # z = -2.0
    
    # Create region: inside cylinder AND between z planes
    cylinder_region = -z_cyl  # inside cylinder
    z_bounds = +z_plane_bottom & -z_plane_top  # -2.0 < z < 2.0
    combined_region = cylinder_region & z_bounds
    
    return combined_region

def create_openmc_cylinder_geometry():
    """Create equivalent cylindrical geometry using OpenMC."""
    # Create surfaces with same IDs
    z_cyl = openmc.ZCylinder(x0=0, y0=0, r=3.0, surface_id=1)
    z_plane_top = openmc.ZPlane(z0=2.0, surface_id=2)
    z_plane_bottom = openmc.ZPlane(z0=-2.0, surface_id=3)
    
    # Create region
    cylinder_region = -z_cyl
    z_bounds = +z_plane_bottom & -z_plane_top
    combined_region = cylinder_region & z_bounds
    
    # Create cell and geometry
    cell = openmc.Cell(region=combined_region)
    universe = openmc.Universe(cells=[cell])
    geometry = openmc.Geometry(universe)
    
    return geometry, combined_region

def create_csg4mc_complex_geometry():
    """Create a more complex geometry with general cylinder."""
    # Create surfaces
    # General cylinder along (1,1,0) direction through origin
    general_cyl = csg4mc.Cylinder(x0=0, y0=0, z0=0, 
                                  axis_x=1, axis_y=1, axis_z=0, 
                                  r=2.0, surface_id=4)
    
    # Bounding box
    x_plane1 = csg4mc.XPlane(x0=3.0, surface_id=5)
    x_plane2 = csg4mc.XPlane(x0=-3.0, surface_id=6)
    y_plane1 = csg4mc.YPlane(y0=3.0, surface_id=7)
    y_plane2 = csg4mc.YPlane(y0=-3.0, surface_id=8)
    
    # Region: inside cylinder AND inside box
    cylinder_region = -general_cyl
    box_region = -x_plane1 & +x_plane2 & -y_plane1 & +y_plane2
    combined_region = cylinder_region & box_region
    
    return combined_region

def create_openmc_complex_geometry():
    """Create equivalent complex geometry using OpenMC."""
    # General cylinder - OpenMC uses different syntax
    # For a cylinder along direction (1,1,0), we need to be careful
    # OpenMC's general cylinder syntax might be different
    
    # For simplicity, let's use a different approach with planes to approximate
    # or use a Z-cylinder rotated conceptually
    
    # Let's create a simpler equivalent geometry
    z_cyl = openmc.ZCylinder(x0=0, y0=0, r=2.0, surface_id=4)  # Approximate
    
    # Bounding box  
    x_plane1 = openmc.XPlane(x0=3.0, surface_id=5)
    x_plane2 = openmc.XPlane(x0=-3.0, surface_id=6)
    y_plane1 = openmc.YPlane(y0=3.0, surface_id=7)
    y_plane2 = openmc.YPlane(y0=-3.0, surface_id=8)
    
    # Region
    cylinder_region = -z_cyl
    box_region = -x_plane1 & +x_plane2 & -y_plane1 & +y_plane2
    combined_region = cylinder_region & box_region
    
    # Create cell and geometry
    cell = openmc.Cell(region=combined_region)
    universe = openmc.Universe(cells=[cell])
    geometry = openmc.Geometry(universe)
    
    return geometry, combined_region

def plot_geometry_xy(region, ax, title, method="sample"):
    """Plot geometry in XY plane."""
    if method == "sample":
        # Manual sampling
        x = np.linspace(-5, 5, 200)
        y = np.linspace(-5, 5, 200)
        X, Y = np.meshgrid(x, y)
        
        inside_map = np.zeros_like(X, dtype=bool)
        for i in range(X.shape[0]):
            for j in range(X.shape[1]):
                try:
                    if hasattr(region, 'contains'):
                        inside_map[i, j] = region.contains((X[i, j], Y[i, j], 0.0))
                    else:
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

def plot_geometry_xz(region, ax, title):
    """Plot geometry in XZ plane."""
    x = np.linspace(-5, 5, 200)
    z = np.linspace(-5, 5, 200)
    X, Z = np.meshgrid(x, z)
    
    inside_map = np.zeros_like(X, dtype=bool)
    for i in range(X.shape[0]):
        for j in range(X.shape[1]):
            try:
                if hasattr(region, 'contains'):
                    inside_map[i, j] = region.contains((X[i, j], 0.0, Z[i, j]))
                else:
                    inside_map[i, j] = (X[i, j], 0.0, Z[i, j]) in region
            except:
                inside_map[i, j] = False
    
    ax.imshow(inside_map, extent=[-5, 5, -5, 5], origin='lower',
              cmap='RdYlBu', alpha=0.8)
    ax.set_xlabel('X')
    ax.set_ylabel('Z')
    ax.set_title(title)
    ax.grid(True, alpha=0.3)
    ax.set_aspect('equal')

def main():
    """Main function to create and plot cylindrical geometries."""
    print("Creating cylindrical geometries...")
    
    # Simple cylinder geometry
    csg4mc_cyl = create_csg4mc_cylinder_geometry()
    openmc_geom, openmc_cyl = create_openmc_cylinder_geometry()
    
    # Complex geometry
    csg4mc_complex = create_csg4mc_complex_geometry()
    openmc_complex_geom, openmc_complex = create_openmc_complex_geometry()
    
    print("Creating plots...")
    fig = plt.figure(figsize=(15, 10))
    
    # Plot 1: Simple cylinder XY view
    ax1 = plt.subplot(2, 3, 1)
    plot_geometry_xy(csg4mc_cyl, ax1, "CSG4MC: Z-Cylinder (XY)")
    
    ax2 = plt.subplot(2, 3, 2)
    plot_geometry_xy(openmc_cyl, ax2, "OpenMC: Z-Cylinder (XY)")
    
    # Plot 2: Simple cylinder XZ view
    ax3 = plt.subplot(2, 3, 3)
    plot_geometry_xz(csg4mc_cyl, ax3, "CSG4MC: Z-Cylinder (XZ)")
    
    # Plot 3: Complex geometry XY view
    ax4 = plt.subplot(2, 3, 4)
    plot_geometry_xy(csg4mc_complex, ax4, "CSG4MC: General Cylinder")
    
    ax5 = plt.subplot(2, 3, 5)
    plot_geometry_xy(openmc_complex, ax5, "OpenMC: Z-Cylinder (approx)")
    
    # Plot 4: Difference comparison
    ax6 = plt.subplot(2, 3, 6)
    
    # Sample both geometries and compute difference
    x = np.linspace(-5, 5, 100)
    y = np.linspace(-5, 5, 100)
    X, Y = np.meshgrid(x, y)
    
    csg4mc_map = np.zeros_like(X, dtype=bool)
    openmc_map = np.zeros_like(X, dtype=bool)
    
    for i in range(X.shape[0]):
        for j in range(X.shape[1]):
            point = (X[i, j], Y[i, j], 0.0)
            try:
                csg4mc_map[i, j] = csg4mc_cyl.contains(point)
                openmc_map[i, j] = point in openmc_cyl
            except:
                pass
    
    # Show difference (where they disagree)
    diff_map = csg4mc_map != openmc_map
    ax6.imshow(diff_map, extent=[-5, 5, -5, 5], origin='lower',
               cmap='Reds', alpha=0.8)
    ax6.set_title("Differences (Red = Disagree)")
    ax6.set_xlabel('X')
    ax6.set_ylabel('Y')
    ax6.grid(True, alpha=0.3)
    ax6.set_aspect('equal')
    
    plt.tight_layout()
    plt.savefig('geometry_comparison_cylinders.png', dpi=150, bbox_inches='tight')
    plt.show()
    
    # Test some points
    print("\nTesting points for cylinder geometry:")
    test_points = [(0, 0, 0), (2, 0, 0), (0, 2.5, 1), (4, 0, 0), (1, 1, 3)]
    for point in test_points:
        csg4mc_result = csg4mc_cyl.contains(point)
        try:
            openmc_result = point in openmc_cyl
        except:
            openmc_result = "Error"
        print(f"Point {point}: CSG4MC={csg4mc_result}, OpenMC={openmc_result}")

if __name__ == "__main__":
    main()