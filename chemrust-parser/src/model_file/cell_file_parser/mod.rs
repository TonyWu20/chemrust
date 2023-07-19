use std::{fmt::Debug, marker::PhantomData};

use castep_periodic_table::{data::ELEMENT_TABLE, element::LookupElement};
use chemrust_core::data::{
    lattice::{LatticeModel, LatticeVectors},
    Atom,
};
use nalgebra::{Matrix3, Point3, Vector3};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, line_ending, multispace0},
    combinator::recognize,
    multi::{count, many0, separated_list0},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

use crate::float;

use self::cell_parse_error::SectionNotFound;

pub trait CellParserState: Debug {}

mod cell_parse_error;

#[derive(Debug)]
pub struct CellParser<'a, S: CellParserState> {
    rest: &'a str,
    to_parse: Option<&'a str>,
    lattice_vectors: Option<LatticeVectors>,
    atoms: Option<Vec<Atom>>,
    state: PhantomData<S>,
}

impl<'a, S: CellParserState> CellParser<'a, S> {
    /// Proceeds to the next block, returns the following contents as rest and block name as result.
    fn next_block_name(input: &str) -> IResult<&str, &str> {
        delimited(
            tag("%BLOCK "),
            recognize(separated_list0(tag("_"), alpha1)),
            line_ending,
        )(input)
    }
    fn get_block_content(input: &'a str, block_name: &str) -> IResult<&'a str, &'a str> {
        let end_sign = format!("%ENDBLOCK {}", block_name);
        let ret = take_until(end_sign.as_str())(input);
        ret
    }
    fn move_out_of_block(input: &str) -> IResult<&str, &str> {
        recognize(count(
            alt((
                preceded(take_until("\n"), line_ending),
                preceded(take_until("\r\n"), line_ending),
            )),
            2,
        ))(input)
    }
    fn search_block(input: &'a str, block_name: &str) -> IResult<&'a str, &'a str> {
        let block_tag_line = format!("%BLOCK {block_name}");
        let ret = take_until(block_tag_line.as_str())(input);
        ret
    }
    fn split_lines(input: &str) -> IResult<&str, Vec<&str>> {
        many0(alt((
            terminated(take_until("\n"), line_ending),
            terminated(take_until("\r\n"), line_ending),
        )))(input)
    }
}
#[derive(Debug)]
pub struct Empty;
impl CellParserState for Empty {}

impl<'a> CellParser<'a, Empty> {
    pub fn new(input: &'a str) -> CellParser<Loaded> {
        CellParser {
            rest: input,
            to_parse: None,
            lattice_vectors: None,
            atoms: None,
            state: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct Loaded;
impl CellParserState for Loaded {}

#[derive(Debug)]
pub struct LatticeCart;
impl CellParserState for LatticeCart {}

impl<'a> CellParser<'a, Loaded> {
    pub fn to_lattice_cart(self) -> CellParser<'a, LatticeCart> {
        let (rest, block_lat_cart) = Self::next_block_name(self.rest).unwrap();
        let (rest, lattice_vectors_lines) = Self::get_block_content(rest, block_lat_cart).unwrap();
        CellParser {
            rest,
            to_parse: Some(lattice_vectors_lines),
            lattice_vectors: None,
            atoms: None,
            state: PhantomData,
        }
    }
    pub fn to_potentials(self) -> Result<CellParser<'a, Potentials>, SectionNotFound> {
        let search_potential_block = Self::search_block(self.rest, "SPECIES_POT");
        if let Ok((rest, _)) = search_potential_block {
            let (rest, block_species_pot) = Self::next_block_name(rest).unwrap();
            let (rest, species_pot_lines) =
                Self::get_block_content(rest, block_species_pot).unwrap();
            Ok(CellParser {
                rest,
                to_parse: Some(species_pot_lines),
                lattice_vectors: None,
                atoms: None,
                state: PhantomData,
            })
        } else {
            Err(SectionNotFound::new("SPECIES_POT"))
        }
    }
}

impl<'a> CellParser<'a, LatticeCart> {
    fn lattice_vector_parser(input: &str) -> IResult<&str, [f64; 3]> {
        let (rest, values) = count(preceded(multispace0, float), 3)(input)?;
        let vector: Vec<f64> = values.iter().map(|&v| v.parse::<f64>().unwrap()).collect();
        let column: [f64; 3] = vector.try_into().unwrap();
        Ok((rest, column))
    }
    fn parse_lattice_vectors(&self) -> Matrix3<f64> {
        let (_, columns) =
            count(terminated(Self::lattice_vector_parser, line_ending), 3)(self.to_parse.unwrap())
                .unwrap();
        let columns_vector: Vec<Vector3<f64>> = columns
            .iter()
            .map(|col| Vector3::from_vec(col.to_vec()))
            .collect();
        Matrix3::from_columns(&columns_vector)
    }
    pub fn to_positions(self) -> CellParser<'a, Positions> {
        let lattice_vectors_data = self.parse_lattice_vectors();
        let lattice_vec: LatticeVectors = LatticeVectors::new(lattice_vectors_data);
        let (rest, _) = Self::move_out_of_block(self.rest).unwrap();
        let (rest, block_positions) = Self::next_block_name(rest).unwrap();
        let (rest, positions_lines) = Self::get_block_content(rest, block_positions).unwrap();
        CellParser {
            rest,
            to_parse: Some(positions_lines),
            lattice_vectors: Some(lattice_vec),
            atoms: self.atoms,
            state: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct Positions;
impl CellParserState for Positions {}

impl<'a> CellParser<'a, Positions> {
    fn get_element(input: &str) -> IResult<&str, &str> {
        preceded(multispace0, alpha1)(input)
    }
    fn get_fractional_coord(input: &str) -> IResult<&str, [f64; 3]> {
        let (rest, (x, y, z)): (&str, (&str, &str, &str)) = tuple((
            preceded(multispace0, float),
            preceded(multispace0, float),
            preceded(multispace0, float),
        ))(input)
        .unwrap();
        let frac_coord = [
            x.parse::<f64>().unwrap(),
            y.parse::<f64>().unwrap(),
            z.parse::<f64>().unwrap(),
        ];
        Ok((rest, frac_coord))
    }
    fn parse_atoms(&self) -> Vec<Atom> {
        let (_, positions_lines) = Self::split_lines(self.to_parse.unwrap()).unwrap();
        positions_lines
            .iter()
            .enumerate()
            .map(|(i, line)| -> Atom {
                let (rest, element) = Self::get_element(line).unwrap();
                let atomic_num = ELEMENT_TABLE
                    .get_by_symbol(element)
                    .unwrap()
                    .atomic_number();
                let (_, frac_coord) = Self::get_fractional_coord(rest).unwrap();
                let frac_point = Point3::from_slice(&frac_coord);
                let cart_coord = self.lattice_vectors.as_ref().unwrap().data() * frac_point;
                Atom::new_builder()
                    .with_symbol(element)
                    .with_atomic_number(atomic_num)
                    .with_coord(&cart_coord)
                    .with_index(i)
                    .ready()
                    .build()
            })
            .collect()
    }
    pub fn build_lattice(&self) -> LatticeModel {
        let atoms = self.parse_atoms();
        let lattice_vectors = self.lattice_vectors.clone();
        LatticeModel::new(&lattice_vectors, &atoms)
    }
    // fn create_atoms(&self) {
    //     let lattice_vectors = self.lattice_vectors.as_ref().unwrap().data();
    //     let symbols_frac = self.parse();
    //     todo!()
    // }
}

#[derive(Debug)]
pub struct Potentials;
impl CellParserState for Potentials {}

impl<'a> CellParser<'a, Potentials> {
    pub fn report_potential_files(&self) -> Vec<String> {
        let (_, potential_lines) = Self::split_lines(self.to_parse.unwrap()).unwrap();
        potential_lines
            .iter()
            .map(|line| line.split_whitespace().last().unwrap().to_string())
            .collect()
    }
}

#[cfg(test)]
mod cell_test {
    use std::fs::read_to_string;

    use super::CellParser;

    #[test]
    fn cell_parser() {
        let file = read_to_string("SAC_GDY_V.cell").unwrap();
        let cell = CellParser::new(&file).to_lattice_cart().to_positions();
        // let lattice_vector = cell.lattice_vectors.as_ref().unwrap().data();
        // let pos = Vector3::new(0.1496332166229109, 0.1496332194727908, 0.5000000000710555);
        // let cart = lattice_vector * pos;
        // println!("{}", cart);
        // let frac = cell.lattice_vectors.as_ref().unwrap().mat_cart_to_frac() * cart;
        // println!("{}", frac);
        // let atoms = cell.parse_atoms();
        // println!("{}", atoms.len());
        let lattice = cell.build_lattice();
        println!("{:#?}", lattice);
    }
    #[test]
    fn cell_parse_potentials() {
        let file = read_to_string("SAC_GDY_V.cell").unwrap();
        let potentials = CellParser::new(&file)
            .to_potentials()
            .unwrap()
            .report_potential_files();
        for pot in potentials {
            println!("{pot}")
        }
    }
}
