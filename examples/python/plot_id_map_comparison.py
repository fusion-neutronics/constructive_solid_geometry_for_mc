"""
Advanced example using OpenMC's id_map feature to create detailed comparisons.
This example focuses on using the new id_map functionality for precise geometry mapping.
"""

import numpy as np
import matplotlib.pyplot as plt
import constructive_solid_geometry_for_mc as csg4mc
import openmc
openmc.config['cross_sections'] = '/home/jon/endf-b7.1-hdf5/endfb-vii.1-hdf5/cross_sections.xml'
def create_multi_region_csg4mc():
    """Create a multi-region geometry with CSG4MC."""
    # Create multiple surfaces
    inner_sphere = csg4mc.Sphere(x0=0, y0=0, z0=0, r=2.0, surface_id=1)
    outer_sphere = csg4mc.Sphere(x0=0, y0=0, z0=0, r=4.0, surface_id=2)
    
    # Planes to create sectors
    x_plane = csg4mc.XPlane(x0=0, surface_id=3)
    y_plane = csg4mc.YPlane(y0=0, surface_id=4)
    
    # Create different regions
    inner_region = -inner_sphere  # Region 1: Inner sphere
    
    # Region 2: Annular region in first quadrant
    annular_first_quad = -outer_sphere & +inner_sphere & +x_plane & +y_plane
    
    # Region 3: Annular region in second quadrant  
    annular_second_quad = -outer_sphere & +inner_sphere & -x_plane & +y_plane
    
    # Region 4: Annular region in third quadrant
    annular_third_quad = -outer_sphere & +inner_sphere & -x_plane & -y_plane
    
    # Region 5: Annular region in fourth quadrant
    annular_fourth_quad = -outer_sphere & +inner_sphere & +x_plane & -y_plane
    
    return {
        'inner': inner_region,
        'quad1': annular_first_quad,
        'quad2': annular_second_quad, 
        'quad3': annular_third_quad,
        'quad4': annular_fourth_quad
    }

def create_multi_region_openmc():
    """Create equivalent multi-region geometry with OpenMC."""
    # Create surfaces with same IDs
    inner_sphere = openmc.Sphere(x0=0, y0=0, z0=0, r=2.0, surface_id=1)
    outer_sphere = openmc.Sphere(x0=0, y0=0, z0=0, r=4.0, surface_id=2)
    x_plane = openmc.XPlane(x0=0, surface_id=3)
    y_plane = openmc.YPlane(y0=0, surface_id=4)
    
    # Create cells with different IDs for identification
    inner_cell = openmc.Cell(cell_id=1, region=-inner_sphere)
    quad1_cell = openmc.Cell(cell_id=2, region=(-outer_sphere & +inner_sphere & +x_plane & +y_plane))
    quad2_cell = openmc.Cell(cell_id=3, region=(-outer_sphere & +inner_sphere & -x_plane & +y_plane))
    quad3_cell = openmc.Cell(cell_id=4, region=(-outer_sphere & +inner_sphere & -x_plane & -y_plane))
    quad4_cell = openmc.Cell(cell_id=5, region=(-outer_sphere & +inner_sphere & +x_plane & -y_plane))
    
    # Create universe and geometry
    universe = openmc.Universe(cells=[inner_cell, quad1_cell, quad2_cell, quad3_cell, quad4_cell])
    geometry = openmc.Geometry(universe)
    
    # Create model with geometry and settings
    model = openmc.Model()
    model.geometry = geometry
    model.settings = openmc.Settings()
    model.settings.run_mode = 'fixed source'
    model.settings.particles = 1000
    model.settings.batches = 10
    
    return model, {
        'inner': inner_cell.region,
        'quad1': quad1_cell.region,
        'quad2': quad2_cell.region,
        'quad3': quad3_cell.region,
        'quad4': quad4_cell.region
    }

def plot_csg4mc_regions(regions, ax, title="CSG4MC Multi-Region"):
    """Plot CSG4MC regions with different colors."""
    x = np.linspace(-5, 5, 300)
    y = np.linspace(-5, 5, 300)
    X, Y = np.meshgrid(x, y)
    
    # Create a region map
    region_map = np.zeros_like(X, dtype=int)
    
    colors = {'inner': 1, 'quad1': 2, 'quad2': 3, 'quad3': 4, 'quad4': 5}
    
    for name, region in regions.items():
        color_id = colors[name]
        for i in range(X.shape[0]):
            for j in range(X.shape[1]):
                if region.contains((X[i, j], Y[i, j], 0.0)):
                    region_map[i, j] = color_id
    
    # Plot with discrete colormap
    im = ax.imshow(region_map, extent=[-5, 5, -5, 5], origin='lower',
                   cmap='tab10', vmin=0, vmax=5)
    ax.set_xlabel('X')
    ax.set_ylabel('Y') 
    ax.set_title(title)
    ax.grid(True, alpha=0.3)
    ax.set_aspect('equal')
    
    return im

def plot_openmc_id_map(model, ax, title="OpenMC ID Map"):
    """Plot OpenMC geometry using id_map feature."""
    try:
        # Use OpenMC's id_map feature
        id_map = model.id_map(
            origin=(0.0, 0.0, 0.0),
            width=(10.0, 10.0),
            pixels=(300, 300),
            basis='xy'
        )
        
        # Extract cell IDs (first channel)
        cell_ids = id_map[:, :, 0]
        
        # Plot the cell ID map
        im = ax.imshow(cell_ids, extent=[-5, 5, -5, 5], origin='lower',
                       cmap='tab10', vmin=0, vmax=5)
        
        ax.set_xlabel('X')
        ax.set_ylabel('Y')
        ax.set_title(title)
        ax.grid(True, alpha=0.3)
        ax.set_aspect('equal')
        
        return im, cell_ids
        
    except Exception as e:
        print(f"OpenMC id_map failed: {e}")
        ax.text(0.5, 0.5, f"OpenMC id_map\nfailed:\n{str(e)}", 
                transform=ax.transAxes, ha='center', va='center')
        ax.set_title(f"{title} (Failed)")
        return None, None

def compare_region_maps(csg4mc_regions, model, ax, title="Comparison"):
    """Compare CSG4MC and OpenMC region maps."""
    # Get CSG4MC map
    x = np.linspace(-5, 5, 200)
    y = np.linspace(-5, 5, 200)
    X, Y = np.meshgrid(x, y)
    
    csg4mc_map = np.zeros_like(X, dtype=int)
    colors = {'inner': 1, 'quad1': 2, 'quad2': 3, 'quad3': 4, 'quad4': 5}
    
    for name, region in csg4mc_regions.items():
        color_id = colors[name]
        for i in range(X.shape[0]):
            for j in range(X.shape[1]):
                if region.contains((X[i, j], Y[i, j], 0.0)):
                    csg4mc_map[i, j] = color_id
    
    # Get OpenMC map
    try:
        id_map = model.id_map(
            origin=(0.0, 0.0, 0.0),
            width=(10.0, 10.0),
            pixels=(200, 200),
            basis='xy'
        )
        openmc_map = id_map[:, :, 0]
        
        # Compute difference
        diff_map = (csg4mc_map != openmc_map).astype(int)
        
        # Plot difference
        im = ax.imshow(diff_map, extent=[-5, 5, -5, 5], origin='lower',
                       cmap='Reds', alpha=0.8)
        ax.set_title(f"{title}\n(Red = Different)")
        
        # Calculate statistics
        total_pixels = diff_map.size
        different_pixels = np.sum(diff_map)
        agreement_percent = (1 - different_pixels/total_pixels) * 100
        
        ax.text(0.02, 0.98, f"Agreement: {agreement_percent:.1f}%", 
                transform=ax.transAxes, va='top',
                bbox=dict(boxstyle="round,pad=0.3", facecolor="white", alpha=0.8))
        
    except Exception as e:
        ax.text(0.5, 0.5, f"Comparison failed:\n{str(e)}", 
                transform=ax.transAxes, ha='center', va='center')
        ax.set_title(f"{title} (Failed)")
    
    ax.set_xlabel('X')
    ax.set_ylabel('Y')
    ax.grid(True, alpha=0.3)
    ax.set_aspect('equal')

def analyze_id_map_details(model):
    """Analyze the OpenMC id_map in detail."""
    try:
        id_map = model.id_map(
            origin=(0.0, 0.0, 0.0),
            width=(10.0, 10.0),
            pixels=(100, 100),
            basis='xy'
        )
        
        print("\nOpenMC ID Map Analysis:")
        print(f"Shape: {id_map.shape}")
        print(f"Data type: {id_map.dtype}")
        
        # Analyze each channel
        cell_ids = id_map[:, :, 0]
        cell_instances = id_map[:, :, 1] 
        material_ids = id_map[:, :, 2]
        
        print(f"\nCell IDs found: {np.unique(cell_ids)}")
        print(f"Cell instances found: {np.unique(cell_instances)}")
        print(f"Material IDs found: {np.unique(material_ids)}")
        
        # Count pixels for each cell
        for cell_id in np.unique(cell_ids):
            if cell_id != 0:  # Skip void regions
                count = np.sum(cell_ids == cell_id)
                percentage = count / cell_ids.size * 100
                print(f"Cell {cell_id}: {count} pixels ({percentage:.1f}%)")
                
        return id_map
        
    except Exception as e:
        print(f"ID map analysis failed: {e}")
        return None

def main():
    """Main function for advanced comparison."""
    print("Creating multi-region geometries...")
    
    # Create geometries
    csg4mc_regions = create_multi_region_csg4mc()
    openmc_model, openmc_regions = create_multi_region_openmc()
    
    print("Creating advanced comparison plots...")
    fig = plt.figure(figsize=(18, 6))
    
    # Plot CSG4MC regions
    ax1 = plt.subplot(1, 3, 1)
    im1 = plot_csg4mc_regions(csg4mc_regions, ax1, "CSG4MC Multi-Region")
    
    # Plot OpenMC ID map
    ax2 = plt.subplot(1, 3, 2)
    im2, cell_ids = plot_openmc_id_map(openmc_model, ax2, "OpenMC ID Map")
    
    # Plot comparison
    ax3 = plt.subplot(1, 3, 3)
    compare_region_maps(csg4mc_regions, openmc_model, ax3, "Difference Map")
    
    # Add colorbars
    if im1 is not None:
        plt.colorbar(im1, ax=ax1, label='Region ID')
    if im2 is not None:
        plt.colorbar(im2, ax=ax2, label='Cell ID')
    
    plt.tight_layout()
    plt.savefig('geometry_comparison_id_map.png', dpi=150, bbox_inches='tight')
    plt.show()
    
    # Analyze OpenMC ID map details
    analyze_id_map_details(openmc_model)
    
    # Test specific points
    print("\nTesting specific points:")
    test_points = [
        (0, 0, 0),      # Center (should be in inner sphere)
        (3, 3, 0),      # First quadrant annular
        (-3, 3, 0),     # Second quadrant annular
        (-3, -3, 0),    # Third quadrant annular
        (3, -3, 0),     # Fourth quadrant annular
        (5, 0, 0),      # Outside all regions
    ]
    
    for point in test_points:
        print(f"\nPoint {point}:")
        for name, region in csg4mc_regions.items():
            result = region.contains(point)
            if result:
                print(f"  CSG4MC: In region '{name}'")
        
        try:
            for name, region in openmc_regions.items():
                if point in region:
                    print(f"  OpenMC: In region '{name}'")
        except Exception as e:
            print(f"  OpenMC test failed: {e}")

if __name__ == "__main__":
    main()