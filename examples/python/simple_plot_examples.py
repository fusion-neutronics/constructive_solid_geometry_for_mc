"""
Simple example demonstrating the basic geometry plotting capabilities.
This serves as a starting point for users to understand both libraries.
"""

import numpy as np
import matplotlib.pyplot as plt
import constructive_solid_geometry_for_mc as csg4mc
import openmc

def simple_sphere_example():
    """Simple sphere comparison."""
    print("=== Simple Sphere Example ===")
    
    # CSG4MC sphere
    csg4mc_sphere = csg4mc.Sphere(x0=0, y0=0, z0=0, r=3.0, surface_id=1)
    csg4mc_region = -csg4mc_sphere
    
    # OpenMC sphere
    openmc_sphere = openmc.Sphere(x0=0, y0=0, z0=0, r=3.0, surface_id=11)  # Different ID
    openmc_region = -openmc_sphere
    openmc_cell = openmc.Cell(cell_id=1, region=openmc_region)
    openmc_universe = openmc.Universe(cells=[openmc_cell])
    openmc_geometry = openmc.Geometry(openmc_universe)
    openmc_model = openmc.Model(geometry=openmc_geometry)
    
    return csg4mc_region, openmc_model, openmc_region, "Simple Sphere (r=3.0)"

def box_example():
    """Simple box comparison."""
    print("=== Simple Box Example ===")
    
    # CSG4MC box
    x1 = csg4mc.XPlane(x0=2, surface_id=2)
    x2 = csg4mc.XPlane(x0=-2, surface_id=3)
    y1 = csg4mc.YPlane(y0=2, surface_id=4)
    y2 = csg4mc.YPlane(y0=-2, surface_id=5)
    csg4mc_region = -x1 & +x2 & -y1 & +y2
    
    # OpenMC box
    ox1 = openmc.XPlane(x0=2, surface_id=12)
    ox2 = openmc.XPlane(x0=-2, surface_id=13)
    oy1 = openmc.YPlane(y0=2, surface_id=14)
    oy2 = openmc.YPlane(y0=-2, surface_id=15)
    openmc_region = -ox1 & +ox2 & -oy1 & +oy2
    openmc_cell = openmc.Cell(cell_id=2, region=openmc_region)
    openmc_universe = openmc.Universe(cells=[openmc_cell])
    openmc_geometry = openmc.Geometry(openmc_universe)
    openmc_model = openmc.Model(geometry=openmc_geometry)
    
    return csg4mc_region, openmc_model, openmc_region, "Simple Box (4x4)"

def cylinder_example():
    """Simple cylinder comparison."""
    print("=== Simple Cylinder Example ===")
    
    # CSG4MC cylinder
    csg4mc_cyl = csg4mc.ZCylinder(x0=0, y0=0, r=2.5, surface_id=6)
    csg4mc_region = -csg4mc_cyl
    
    # OpenMC cylinder
    openmc_cyl = openmc.ZCylinder(x0=0, y0=0, r=2.5, surface_id=16)
    openmc_region = -openmc_cyl
    openmc_cell = openmc.Cell(cell_id=3, region=openmc_region)
    openmc_universe = openmc.Universe(cells=[openmc_cell])
    openmc_geometry = openmc.Geometry(openmc_universe)
    openmc_model = openmc.Model(geometry=openmc_geometry)
    
    return csg4mc_region, openmc_model, openmc_region, "Simple Z-Cylinder (r=2.5)"

def plot_comparison(csg4mc_region, openmc_model, openmc_region, title, ax_row):
    """Plot comparison between CSG4MC and OpenMC for a given geometry."""
    
    # Create grid
    x = np.linspace(-4, 4, 150)
    y = np.linspace(-4, 4, 150)
    X, Y = np.meshgrid(x, y)
    
    # Sample CSG4MC
    csg4mc_map = np.zeros_like(X, dtype=bool)
    for i in range(X.shape[0]):
        for j in range(X.shape[1]):
            csg4mc_map[i, j] = csg4mc_region.contains((X[i, j], Y[i, j], 0.0))
    
    # Plot CSG4MC
    ax_row[0].imshow(csg4mc_map, extent=[-4, 4, -4, 4], origin='lower',
                     cmap='RdYlBu', alpha=0.8)
    ax_row[0].set_title(f'CSG4MC: {title}')
    ax_row[0].set_xlabel('X')
    ax_row[0].set_ylabel('Y')
    ax_row[0].grid(True, alpha=0.3)
    ax_row[0].set_aspect('equal')
    
    # Sample OpenMC using manual sampling (id_map requires cross-sections)
    print(f"Using manual sampling for {title} (id_map requires cross-sections)")
    
    openmc_map = np.zeros_like(X, dtype=bool)
    
    for i in range(X.shape[0]):
        for j in range(X.shape[1]):
            try:
                openmc_map[i, j] = (X[i, j], Y[i, j], 0.0) in openmc_region
            except:
                openmc_map[i, j] = False
    
    ax_row[1].imshow(openmc_map, extent=[-4, 4, -4, 4], origin='lower',
                     cmap='RdYlBu', alpha=0.8)
    method_text = "manual"
    
    ax_row[1].set_title(f'OpenMC: {title} ({method_text})')
    ax_row[1].set_xlabel('X')
    ax_row[1].set_ylabel('Y')
    ax_row[1].grid(True, alpha=0.3)
    ax_row[1].set_aspect('equal')
    
    # Compute and plot difference
    diff_map = csg4mc_map != openmc_map
    diff_percentage = np.sum(diff_map) / diff_map.size * 100
    
    ax_row[2].imshow(diff_map, extent=[-4, 4, -4, 4], origin='lower',
                     cmap='Reds', alpha=0.8)
    ax_row[2].set_title(f'Difference: {diff_percentage:.2f}% pixels differ')
    ax_row[2].set_xlabel('X')
    ax_row[2].set_ylabel('Y')
    ax_row[2].grid(True, alpha=0.3)
    ax_row[2].set_aspect('equal')
    
    return diff_percentage

def test_points(csg4mc_region, openmc_model, openmc_region, title):
    """Test specific points and print results."""
    print(f"\n--- Testing points for {title} ---")
    
    test_points = [
        (0, 0, 0),
        (1, 1, 0),
        (2, 2, 0),
        (3, 0, 0),
        (0, 3, 0),
        (-2, -2, 0)
    ]
    
    for point in test_points:
        csg4mc_result = csg4mc_region.contains(point)
        try:
            openmc_result = point in openmc_region
        except Exception as e:
            openmc_result = f"Error: {e}"
        
        match = "✓" if csg4mc_result == openmc_result else "✗"
        print(f"  {point}: CSG4MC={csg4mc_result}, OpenMC={openmc_result} {match}")

def main():
    """Main function to run all simple examples."""
    print("Running simple geometry comparisons...")
    
    # Create examples
    examples = [
        simple_sphere_example(),
        box_example(),
        cylinder_example()
    ]
    
    # Create plots
    fig, axes = plt.subplots(3, 3, figsize=(15, 12))
    
    total_diff = 0
    for i, (csg4mc_region, openmc_model, openmc_region, title) in enumerate(examples):
        diff_percent = plot_comparison(csg4mc_region, openmc_model, openmc_region, title, axes[i])
        total_diff += diff_percent
        test_points(csg4mc_region, openmc_model, openmc_region, title)
    
    avg_diff = total_diff / len(examples)
    fig.suptitle(f'CSG4MC vs OpenMC Comparison (Avg difference: {avg_diff:.2f}%)', 
                 fontsize=16)
    
    plt.tight_layout()
    plt.savefig('simple_geometry_comparisons.png', dpi=150, bbox_inches='tight')
    plt.show()
    
    print(f"\nOverall average difference: {avg_diff:.2f}%")
    
    # Demonstrate bounding box functionality
    print("\n=== Bounding Box Comparison ===")
    for csg4mc_region, openmc_model, openmc_region, title in examples:
        try:
            bbox = csg4mc_region.bounding_box()
            print(f"{title}:")
            print(f"  CSG4MC bounding box: {bbox.lower_left} to {bbox.upper_right}")
            
            # OpenMC bounding box (if available)
            try:
                openmc_bbox = openmc_region.bounding_box
                print(f"  OpenMC bounding box: {openmc_bbox}")
            except:
                print(f"  OpenMC bounding box: Not available")
        except Exception as e:
            print(f"  Bounding box error: {e}")

if __name__ == "__main__":
    main()