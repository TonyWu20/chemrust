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
- [x] Whole intersect determination workflow
  - [x] Build spheres for each coordinate points in kd-tree
  - [x] Iterate the sphere kd-tree, check intersections, build circles
  - [x] From the circles kd-tree, find intersection points between circles
  - [x] Each found point will be added into a vec with a counting index, if next found intersect point between circles repeats with the previously found one, add counting index.
  - [x] Summarize the number of spheres, circles, and points. Sphere-1 C.N., circle-2 C.N., points-at least 4
- [x] Limit bondlength to 2 Å? Or the results will not be practical.
- [ ] Report in a more detailed and reasonable manner
- [x] Package

## Bug investigations

- [x] Incorrect sphere intersections of circles (Fixed on May 2nd)
- [x] Check if we need to consider more spheres/circles/points with equal distance/close distance, instead of finding just the nearest one to each coord. (Done on June 10th)
  - Sphere stage: using 2 \* desired bonding distance
  - Circle stage: iterate through.
- [x] Bondlength includes the atomic radius of elements: Use Materials Studio's bonding criteria to exclude impractical bonding possibilities. (Done on June 10th)
- [x] Limit bondlength to 2-3 Å? Or determined by tolerance like Material Studios: ideal bondlength \*min_tolerance < ideal bondlength < ideal bondlength \*max_tolerance (Done on June 10th)
- [ ] A "blacklist" of elements could be necessary to avoid considering impractical bonding combos. (June 12th)
