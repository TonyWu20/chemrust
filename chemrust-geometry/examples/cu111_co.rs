//! Cu(111)+CO CASTEP Input Generator
//!
//! Generates `.cell` and `.param` files for a 4-layer Cu(111) slab
//! with CO adsorbate, using typical settings from Fe2O3_CASTEP_GeomOpt_vanilla_dm.
//!
//! Run: cargo run --example cu111_co
//!
//! Output: Cu111_CO.cell, Cu111_CO.param

use castep_cell_fmt::{format::to_string_many_spaced, ToCellFile};
use castep_cell_io::{
    cell::{
        bz_sampling_kpoints::KpointsMpSpacing,
        lattice_param::LatticeCart,
        positions::{PositionFracEntry, PositionsFrac},
        species::{
            Species, SpeciesLcaoState, SpeciesLcaoStates, SpeciesMass, SpeciesMassEntry,
            SpeciesPot, SpeciesPotEntry,
        },
        symmetry::SymmetryGenerate,
    },
    param::{
        basis_set::{CutOffEnergy, FineGridScale, FiniteBasisCorr, FixedNpw, GridScale},
        density_mixing::{
            MixChargeAmp, MixChargeGmax, MixHistoryLength, MixSpinAmp, MixSpinGmax, MixingScheme,
        },
        electronic::PercExtraBands,
        electronic_minimisation::{
            ElecEnergyTol, FixOccupancy, MaxScfCycles, NumDumpCycles, SmearingWidth,
        },
        exchange_correlation::{SpinPolarized, XcFunctional},
        general::{
            OptStrategy, PageWvfns, Task, WriteFormattedDensity, WriteFormattedPotential,
        },
        BandStructureParams, BasisSetParams, DensityMixingParams, ElectricFieldParams,
        ElectronicExcitationsParams, ElectronicMinimisationParams, ElectronicParams,
        ExchangeCorrelationParams, GeneralParams, GeometryOptimizationParams,
        MolecularDynamicsParams, NmrParams, OpticsParams, PhononParams, PopulationAnalysisParams,
        PseudopotentialParams, SolvationParams, TransitionStateParams,
    },
    CellDocument, ParamDocument, Positions,
};
use castep_periodic_table::data::ELEMENT_TABLE;
use castep_periodic_table::element::LookupElement;
use chemrust_geometry::slab::cu111_co_system;
use chemrust_geometry::ElementSymbol;

fn main() -> anyhow::Result<()> {
    // ── Build the structure ──────────────────────────────────────
    let cu111_co = cu111_co_system(3.615); // Cu lattice constant in Angstrom

    // ── Species data from periodic table ────────────────────────
    let unique_species: std::collections::BTreeSet<ElementSymbol> =
        cu111_co.species.iter().copied().collect();
    let mut sp_mass_entries = Vec::new();
    let mut sp_pot_entries = Vec::new();
    let mut sp_lcao_entries = Vec::new();
    for sp in &unique_species {
        let elem = ELEMENT_TABLE.get_by_symbol(*sp);
        let sym = Species::Symbol(sp.to_string());
        sp_mass_entries.push(
            SpeciesMassEntry::builder()
                .species(sym.clone())
                .mass(elem.mass())
                .build(),
        );
        sp_pot_entries.push(SpeciesPotEntry {
            species: sym.clone(),
            filename: elem.potential().to_string(),
        });
        sp_lcao_entries.push(
            SpeciesLcaoState::builder()
                .species(sym)
                .num_states(elem.lcao() as u32)
                .build(),
        );
    }

    // ── CellDocument ────────────────────────────────────────────
    let cell = cu111_co.cell.unwrap();
    let tensor = cell.tensor();

    let positions = Positions::Frac(PositionsFrac {
        positions: cu111_co
            .species
            .iter()
            .zip(cu111_co.frac_coords.iter())
            .map(|(sp, coord)| PositionFracEntry {
                species: Species::Symbol(sp.to_string()),
                coord: [coord.x, coord.y, coord.z],
                spin: None,
                mixture: None,
            })
            .collect(),
    });

    let cell_doc = CellDocument::builder()
        .lattice(LatticeCart {
            unit: None,
            a: tensor.column(0).into(),
            b: tensor.column(1).into(),
            c: tensor.column(2).into(),
        })
        .positions(positions)
        .maybe_species_mass(Some(SpeciesMass::builder().masses(sp_mass_entries).build()))
        .maybe_species_pot(Some(
            SpeciesPot::builder().potentials(sp_pot_entries).build(),
        ))
        .maybe_species_lcao_states(Some(
            SpeciesLcaoStates::builder().states(sp_lcao_entries).build(),
        ))
        .kpoints_mp_spacing(KpointsMpSpacing {
            value: 0.07,
            unit: None, // default 1/ang
        })
        .symmetry_generate(SymmetryGenerate)
        .build()?;

    // ── ParamDocument ───────────────────────────────────────────
    let param_doc = ParamDocument::builder()
        .general(GeneralParams {
            task: Some(Task::SinglePoint),
            opt_strategy: Some(OptStrategy::Speed),
            page_wvfns: Some(PageWvfns(0)),
            write_formatted_potential: Some(WriteFormattedPotential(true)),
            write_formatted_density: Some(WriteFormattedDensity(true)),
            ..Default::default()
        })
        .electronic(ElectronicParams {
            perc_extra_bands: Some(PercExtraBands(72.0)),
            ..Default::default()
        })
        .basis_set(BasisSetParams {
            cutoff_energy: Some(CutOffEnergy {
                value: 400.0,
                unit: None, // default eV
            }),
            grid_scale: Some(GridScale(1.5)),
            fine_grid_scale: Some(FineGridScale(1.5)),
            finite_basis_corr: Some(FiniteBasisCorr::None),
            fixed_npw: Some(FixedNpw(false)),
            ..Default::default()
        })
        .exchange_correlation(ExchangeCorrelationParams {
            xc_functional: Some(XcFunctional::Pbe),
            spin_polarized: Some(SpinPolarized(false)),
            ..Default::default()
        })
        .electronic_minimisation(ElectronicMinimisationParams {
            elec_energy_tol: Some(ElecEnergyTol {
                value: 1e-5,
                unit: None, // default eV
            }),
            max_scf_cycles: Some(MaxScfCycles(6000)),
            fix_occupancy: Some(FixOccupancy(false)),
            smearing_width: Some(SmearingWidth {
                value: 0.1,
                unit: None, // default eV
            }),
            num_dump_cycles: Some(NumDumpCycles(1)),
            ..Default::default()
        })
        .density_mixing(DensityMixingParams {
            mixing_scheme: Some(MixingScheme::Pulay),
            mix_charge_amp: Some(MixChargeAmp(0.5)),
            mix_spin_amp: Some(MixSpinAmp(2.0)),
            mix_charge_gmax: Some(MixChargeGmax {
                value: 1.5,
                unit: None, // default 1/ang
            }),
            mix_spin_gmax: Some(MixSpinGmax {
                value: 1.5,
                unit: None,
            }),
            mix_history_length: Some(MixHistoryLength(20)),
            ..Default::default()
        })
        // Remaining groups required by builder
        .geometry_optimization(GeometryOptimizationParams::default())
        .phonon(PhononParams::default())
        .band_structure(BandStructureParams::default())
        .molecular_dynamics(MolecularDynamicsParams::default())
        .electric_field(ElectricFieldParams::default())
        .pseudopotential(PseudopotentialParams::default())
        .population_analysis(PopulationAnalysisParams::default())
        .optics(OpticsParams::default())
        .nmr(NmrParams::default())
        .solvation(SolvationParams::default())
        .electronic_excitations(ElectronicExcitationsParams::default())
        .transition_state(TransitionStateParams::default())
        .build();

    // ── Write files ─────────────────────────────────────────────
    let cell_text = to_string_many_spaced(&cell_doc.to_cell_file());
    let param_text = to_string_many_spaced(&param_doc.to_cell_file());

    std::fs::write("Cu111_CO.cell", &cell_text)?;
    std::fs::write("Cu111_CO.param", &param_text)?;

    // ── Summary ──────────────────────────────────────────────────
    let n_cu = cu111_co
        .species
        .iter()
        .filter(|&sp| *sp == ElementSymbol::Cu)
        .count();
    let n_c = cu111_co
        .species
        .iter()
        .filter(|&sp| *sp == ElementSymbol::C)
        .count();
    let n_o = cu111_co
        .species
        .iter()
        .filter(|&sp| *sp == ElementSymbol::O)
        .count();

    println!(
        "Wrote Cu111_CO.cell ({:.1} KiB) and Cu111_CO.param ({:.1} KiB)",
        cell_text.len() as f64 / 1024.0,
        param_text.len() as f64 / 1024.0,
    );
    println!(
        "Structure: {} Cu + {} C + {} O = {} atoms",
        n_cu,
        n_c,
        n_o,
        cu111_co.num_atoms()
    );
    println!(
        "Cell: {:.3} x {:.3} x {:.3} A, PBC: {:?}",
        cell.lengths().0,
        cell.lengths().1,
        cell.lengths().2,
        cu111_co.pbc
    );
    println!("KPOINT_MP_SPACING: 0.07, XC: PBE, Cutoff: 400 eV");

    Ok(())
}
