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
- [ ] Identify the coordination number
  - [ ] 3d geometry problem
    - [ ] Representation of sphere-sphere intersection: the circular curve
    - [ ] Solvation of the intersection of the curves: two-points
