#![allow(dead_code)]

mod algorithm;
mod geometry;
mod mounting_analyze;

pub use crate::analyzer::mounting_analyze::MountingChecker;
pub use algorithm::{FinalReport, IntersectChecker};

#[cfg(test)]
mod test {
    use std::{
        fs::read_to_string,
        io::{self, BufRead},
    };

    use castep_periodic_table::{data::ELEMENT_TABLE, element::LookupElement};
    use chemrust_core::data::atom::AtomCollections;
    use chemrust_parser::CellParser;
    use kd_tree::KdIndexTree;
    use nalgebra::Point3;

    use crate::analyzer::{
        algorithm::Visualize,
        geometry::{
            Circle, CircleIntersectChecker, CircleIntersectResult, Intersect, Sphere,
            SphereIntersectResult,
        },
        mounting_analyze::MountingChecker,
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
        let kdtree: KdIndexTree<Point3<f64>> = KdIndexTree::build_by_ordered_float(&coords);
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
                CircleIntersectResult::NonCoplanarZero => pure_circles.push(circle_sites[j]),
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
    #[test]
    fn test_checker() {
        let cell = read_to_string("../chemrust-parser/SAC_GDY_V.cell").unwrap();
        let lattice = CellParser::new(&cell)
            .to_lattice_cart()
            .to_positions()
            .build_lattice();
        let mount_element = ELEMENT_TABLE.get_by_symbol("Co").unwrap();
        let mount_distance = 1.41;
        let mount_checker = MountingChecker::new_builder()
            .with_element(mount_element)
            .with_bondlength(mount_distance)
            .build();
        let final_report = mount_checker.mount_search(lattice.atoms());
        println!(
            "New element: {}, bonding_distance: {}",
            mount_element.symbol(),
            mount_distance
        );
        final_report.sphere_sites().iter().for_each(|sphere| {
            let mut new_atoms = sphere.draw_with_element("Co");
            let mut new_lattice = lattice.clone();
            let current_num = new_lattice.number_of_atoms();
            new_atoms
                .iter_mut()
                .for_each(|atom| atom.set_index(current_num + atom.index()));
            new_lattice.append_atom(&mut new_atoms);
        });
        println!("Spheres: {}", final_report.sphere_sites().len());
        println!("Circles: {}", final_report.circles().len());
        println!("Cut points: {}", final_report.cut_points().len());
        println!("Multi points: {}", final_report.multi_cn_points().len());
        final_report.multi_cn_points().iter().for_each(|p| {
            println!(
                "Coord: {:#}, Connecting: {:?}",
                p.coord(),
                p.connecting_atom_ids()
            )
        })
    }
}
