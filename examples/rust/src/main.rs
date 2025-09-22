use constructive_solid_geometry_for_mc::surface::Surface;
use constructive_solid_geometry_for_mc::region::{Region, HalfspaceType};
use std::sync::Arc;

fn main() {
    println!("Creating cube surfaces...");
    // Cube from x=0..1, y=0..1, z=0..1
    let sx0 = Arc::new(Surface::x_plane(0.0, 1, None));
    let sx1 = Arc::new(Surface::x_plane(1.0, 2, None));
    let sy0 = Arc::new(Surface::y_plane(0.0, 3, None));
    let sy1 = Arc::new(Surface::y_plane(1.0, 4, None));
    let sz0 = Arc::new(Surface::z_plane(0.0, 5, None));
    let sz1 = Arc::new(Surface::z_plane(1.0, 6, None));

    // Cube region: intersection of all halfspaces
    let cube = Region::new_from_halfspace(HalfspaceType::Above(sx0.clone()))
        .intersection(&Region::new_from_halfspace(HalfspaceType::Below(sx1.clone())))
        .intersection(&Region::new_from_halfspace(HalfspaceType::Above(sy0.clone())))
        .intersection(&Region::new_from_halfspace(HalfspaceType::Below(sy1.clone())))
        .intersection(&Region::new_from_halfspace(HalfspaceType::Above(sz0.clone())))
        .intersection(&Region::new_from_halfspace(HalfspaceType::Below(sz1.clone())));
    println!("Cube region created.");
    let cube_bb = cube.bounding_box();
    println!("Cube bounding box: {:?}", cube_bb);

    println!("Creating sphere surface...");
    let sphere = Arc::new(Surface::new_sphere(0.5, 0.5, 0.5, 0.4, 7, None));
    let sphere_region = Region::new_from_halfspace(HalfspaceType::Below(sphere.clone()));
    println!("Sphere region created.");
    let sphere_bb = sphere_region.bounding_box();
    println!("Sphere bounding box: {:?}", sphere_bb);

    println!("Making intersection and union regions...");
    let intersection = cube.intersection(&sphere_region);
    let union = cube.union(&sphere_region);
    println!("Intersection bounding box: {:?}", intersection.bounding_box());
    println!("Union bounding box: {:?}", union.bounding_box());

    println!("Testing point containment...");
    let pt_inside = (0.5, 0.5, 0.5);
    let pt_outside = (1.5, 1.5, 1.5);
    println!("Point {:?} in intersection: {}", pt_inside, intersection.contains(pt_inside));
    println!("Point {:?} in union: {}", pt_inside, union.contains(pt_inside));
    println!("Point {:?} in intersection: {}", pt_outside, intersection.contains(pt_outside));
    println!("Point {:?} in union: {}", pt_outside, union.contains(pt_outside));
}
