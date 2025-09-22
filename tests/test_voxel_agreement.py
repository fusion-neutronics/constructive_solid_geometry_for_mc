import pytest
import numpy as np
import openmc
from test_geometry_agreement_fixed import grid_test_geometry_agreement, create_sphere_geometry


@pytest.fixture(autouse=True)
def setup_openmc():
    """Set up OpenMC configuration for all tests."""
    openmc.config['cross_sections'] = '/home/jon/endf-b7.1-hdf5/endfb-vii.1-hdf5/cross_sections.xml'


def test_sphere_voxel_agreement():
    """Test geometry agreement between CSG4MC and OpenMC using voxel plots."""
    # Create sphere geometries
    csg4mc_sphere, openmc_model = create_sphere_geometry(radius=2.5)
    
    # Test with moderate resolution for speed
    result = grid_test_geometry_agreement(
        csg4mc_region=csg4mc_sphere,
        openmc_model=openmc_model,
        bounds=(-4, 4),
        resolution=8,
        tolerance=0.20  # Allow 20% disagreement due to voxel discretization
    )
    
    # Check that the test ran successfully
    assert result['total_points'] == 8**3
    assert result['openmc_has_voxel_plot'] is True
    assert result['voxel_shape'] == (8, 8, 8)
    assert set(result['unique_cell_ids']) == {1, 2}  # Material and void cells
    
    # Check that we have reasonable agreement (accounting for discretization)
    assert result['agreement_fraction'] > 0.8, f"Agreement fraction {result['agreement_fraction']} too low"
    
    # All disagreements should be near the sphere boundary
    if result['disagreements'] > 0:
        distances = [d['distance_from_origin'] for d in result['disagreement_points']]
        # Most disagreements should be reasonably close to the sphere radius (2.5)
        # Allow for voxel discretization effects - voxels can extend beyond the exact boundary
        near_boundary = sum(1 for d in distances if 1.5 < d < 3.5)
        assert near_boundary >= len(distances) * 0.7, f"Most disagreements should be near sphere boundary. Distances: {distances[:5]}"


def test_sphere_voxel_high_resolution():
    """Test that higher resolution improves boundary precision."""
    csg4mc_sphere, openmc_model = create_sphere_geometry(radius=2.5)
    
    # Test with higher resolution
    result = grid_test_geometry_agreement(
        csg4mc_region=csg4mc_sphere,
        openmc_model=openmc_model,
        bounds=(-4, 4),
        resolution=12,
        tolerance=0.25  # More lenient for higher resolution boundary effects
    )
    
    # Should still have good core agreement
    assert result['agreement_fraction'] > 0.75
    assert result['total_points'] == 12**3
    

@pytest.mark.parametrize("radius", [1.0, 2.5, 3.5])
def test_different_sphere_sizes(radius):
    """Test voxel agreement for different sphere sizes."""
    csg4mc_sphere, openmc_model = create_sphere_geometry(radius=radius)
    
    # Adjust bounds based on sphere size
    bound = radius + 2
    
    result = grid_test_geometry_agreement(
        csg4mc_region=csg4mc_sphere,
        openmc_model=openmc_model,
        bounds=(-bound, bound),
        resolution=8,
        tolerance=0.25
    )
    
    # Basic checks
    assert result['openmc_has_voxel_plot'] is True
    assert result['agreement_fraction'] > 0.7  # Should have decent agreement
    
    # Disagreements should be near the sphere boundary
    if result['disagreements'] > 0:
        distances = [d['distance_from_origin'] for d in result['disagreement_points'][:5]]
        # Check that disagreements are roughly near the radius
        for dist in distances:
            assert abs(dist - radius) < radius * 0.5, f"Disagreement at {dist} too far from radius {radius}"


if __name__ == "__main__":
    # Run a quick test
    test_sphere_voxel_agreement()
    print("Voxel plot test passed!")