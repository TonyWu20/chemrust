#![allow(dead_code)]

mod geometry;

#[cfg(test)]
mod test {
    use std::{
        fs::read_to_string,
        io::{self, BufRead},
    };

    use chemrust_core::data::atom::AtomCollections;
    use chemrust_parser::CellParser;
    use kd_tree::KdIndexTree;
    use nalgebra::Point3;

    use crate::analyzer::geometry::{
        Circle, CircleIntersectChecker, CircleIntersectResult, Intersect, Sphere,
        SphereIntersectResult,
    };

    #[test]
    fn test_kd_tree() {
        let cell = read_to_string("../chemrust-parser/SAC_GDY_V.cell").unwrap();
        let lattice = CellParser::new(&cell)
            .to_lattice_cart()
            .to_positions()
            .build_lattice();
        let atom_collections: AtomCollections = lattice.atoms().into();
        let coords: Vec<Point3<f64>> = atom_collections.cartesian_coords().to_vec();
        let kdtree = KdIndexTree::build_by_ordered_float(&coords);
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_line(&mut buffer).unwrap();
        let radius = buffer.trim().parse::<f64>().unwrap();
        let spheres: Vec<Sphere> = coords.iter().map(|&xyz| Sphere::new(xyz, radius)).collect();
        // let mut walked_over_set: HashSet<usize> = HashSet::new();
        let sphere_intersections: Vec<SphereIntersectResult> = coords
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let found = kdtree.nearests(p, 2);
                let found_id = found.get(1).unwrap().item;
                let found_sphere = spheres[*found_id];
                let this_sphere = spheres[i];
                let intersects = this_sphere.intersects(&found_sphere);
                intersects
                // if let None = walked_over_set.get(found_id) {
                //     let distance = found.get(1).unwrap().squared_distance;
                //     if (2.0 * radius - distance).abs() <= 0.001 {
                //         println!("One intersect between atom {} and {}", i, found_id);
                //     } else if 2.0 * radius - distance > 0.001 {
                //         println!("Two intersects between atom {} and {}", i, found_id);
                //         let circle_center = center(&p, &coords[*found_id]);
                //         let circle_radius = (radius.powi(2) - (distance / 2.0).powi(2)).sqrt();
                //         let normal = Unit::new_normalize(coords[*found_id] - p);
                //         let circle = Circle::new(circle_center, circle_radius, normal);
                //         circles.push(circle);
                //     } else {
                //         println!("No intersect between atom {} and {}", i, found_id)
                //     }
                //     walked_over_set.insert(i);
                // }
            })
            .collect();
        let mut single_coordination_sites: Vec<Sphere> = Vec::new();
        let mut sphere_cut_points_sites: Vec<Point3<f64>> = Vec::new();
        let mut circle_sites: Vec<Circle> = Vec::new();
        sphere_intersections
            .iter()
            .enumerate()
            .for_each(|(i, res)| match res {
                SphereIntersectResult::Zero => {
                    single_coordination_sites.push(spheres[i]);
                }
                SphereIntersectResult::SinglePoint(p) => sphere_cut_points_sites.push(*p),
                SphereIntersectResult::Circle(c) => circle_sites.push(*c),
                _ => (),
            });
        println!(
            r#"At radius: {}, the structure has:
{} singly coordinated sites,
{} doubly coordinated sites as single points,
{} doubly coordinated sites allowed in circles"#,
            radius,
            single_coordination_sites.len(),
            sphere_cut_points_sites.len(),
            circle_sites.len()
        );
        let circle_centers: Vec<Point3<f64>> = circle_sites.iter().map(|c| c.center).collect();
        let circles_kdtree = KdIndexTree::build_by_ordered_float(&circle_centers);
        let circle_itersections: Vec<CircleIntersectResult> = circle_sites
            .iter()
            .map(|circle| {
                let circle_center = circle.center;
                let found = circles_kdtree.nearests(&circle_center, 2);
                let found_id = found.get(1).unwrap().item;
                let found_circle = circle_sites[*found_id];
                CircleIntersectChecker::new(circle, &found_circle).check()
            })
            .collect();
        let mut pure_circles = Vec::new();
        let mut quad_coord_points: Vec<Point3<f64>> = Vec::new();
        circle_itersections
            .iter()
            .enumerate()
            .for_each(|(j, res)| match res {
                CircleIntersectResult::Zero => pure_circles.push(circle_sites[j]),
                CircleIntersectResult::Single(p) => quad_coord_points.push(*p),
                CircleIntersectResult::Double(points) => {
                    quad_coord_points.push(points.0);
                    quad_coord_points.push(points.1);
                }
                _ => (),
            });
        println!(
            r#"The circle sites are further checked:
{} doubly coordinated sites allowed in circles,
{} quadruply coordinated sites allowed in points"#,
            pure_circles.len(),
            quad_coord_points.len()
        );
    }
}
