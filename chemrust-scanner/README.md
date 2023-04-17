# Mounting site scanning module

This module serves for the scanning of all possible mounting site in a given structure with a specified bonding distance. The locations and coordination numbers of the sites will be reported.

## Input

The input to the module includes:

1. A structure, in the `.cell` format.
2. The desired bonding distance, in angstrom.

## Output

A table of the xyz coordinates of the sites and coordination numbers.

## Development

- [x] Read-in a `cell`
- [x] Analyze the structure with kd-tree
- [x] Search the space around each atom by a given bonding distance
- [x] 3d geometry problem
  - [x] Representation of sphere-sphere intersection: the circular curve
  - [x] Solvation of the intersection of the curves: two-points
- [ ] Whole intersect determination workflow
  - [ ] Build spheres for each coordinate points in kd-tree
  - [ ] Iterate the sphere kd-tree, check intersections, build circles
  - [ ] From the circles kd-tree, find intersection points between circles
  - [ ] Each found point will be added into a vec with a counting index, if next found intersect point between circles repeats with the previously found one, add counting index.
  - [ ] Summarize the number of spheres, circles, and points. Sphere-1 C.N., circle-2 C.N., points-at least 4
