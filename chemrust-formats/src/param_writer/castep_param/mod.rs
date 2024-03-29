use std::{any::TypeId, fmt::Display, marker::PhantomData};

mod settings;
mod tasks;

use chemrust_core::builder_state::{BuilderState, Pending, Ready};
use settings::FiniteBasisCorr;

use self::settings::{MetalsMethod, OptStrategy, XCFunctional};
pub use self::tasks::{BandStructureParam, GeomOptParam};

/// Trait to limit the type passed to `CastepParam<T>`
pub trait Task: Default + Display {}

#[derive(Debug)]
/// Struct to represent a Castep parameter file.
pub struct CastepParam<T: Task> {
    xc_functional: XCFunctional,
    spin_polarized: bool,
    spin: u8,
    opt_strategy: OptStrategy,
    page_wvfns: u32,
    cut_off_energy: f64,
    grid_scale: f64,
    fine_grid_scale: f64,
    finite_basis_corr: FiniteBasisCorr,
    elec_energy_tol: f64,
    max_scf_cycles: u32,
    fix_occupancy: bool,
    metals_method: MetalsMethod,
    perc_extra_bands: u32,
    smearing_width: f64,
    spin_fix: u32,
    num_dump_cycles: u32,
    calculate_elf: bool,
    calculate_stress: bool,
    popn_calculate: bool,
    calculate_hirshfeld: bool,
    calculate_densdiff: bool,
    pdos_calculate_weights: bool,
    extra_setting: T,
}

impl<T: Task> CastepParam<T> {
    pub fn build() -> CastepParamBuilder<T, Pending> {
        CastepParamBuilder::<T, Pending>::new()
    }
}

/// Builder for `CastepParam<T>`
#[derive(Default, Debug)]
pub struct CastepParamBuilder<T, State>
where
    T: Task,
    State: BuilderState,
{
    task: T,
    spin_total: u8,
    cut_off_energy: f64,
    metals_method: MetalsMethod,
    state: PhantomData<State>,
}

/// Methods when parameters are not all ready.
impl<T, S> CastepParamBuilder<T, S>
where
    T: Task,
    S: BuilderState,
{
    pub fn new() -> CastepParamBuilder<T, Pending> {
        CastepParamBuilder {
            task: T::default(),
            spin_total: 0_u8,
            cut_off_energy: 0.0,
            metals_method: MetalsMethod::default(),
            state: PhantomData,
        }
    }
}

impl<T> CastepParamBuilder<T, Pending>
where
    T: Task,
{
    pub fn with_spin_total(self, spin_total: u8) -> Self {
        Self { spin_total, ..self }
    }
    pub fn with_cut_off_energy(self, cut_off_energy: f64) -> Self {
        Self {
            cut_off_energy,
            ..self
        }
    }
    pub fn set_to_edft(self) -> Self {
        Self {
            metals_method: MetalsMethod::Edft,
            ..self
        }
    }
    pub fn ready(self) -> CastepParamBuilder<T, Ready> {
        let Self {
            task,
            spin_total,
            cut_off_energy,
            metals_method,
            state: _,
        } = self;
        CastepParamBuilder {
            task,
            spin_total,
            cut_off_energy,
            metals_method,
            state: PhantomData,
        }
    }
}

/// When parameters are all settled, build `CastepParam<T>`
impl<T> CastepParamBuilder<T, Ready>
where
    T: Task + 'static,
{
    pub fn build(&self) -> CastepParam<T> {
        CastepParam {
            spin: self.spin_total,
            cut_off_energy: self.cut_off_energy,
            ..Default::default()
        }
    }
}

impl<T> Default for CastepParam<T>
where
    T: Task + 'static,
{
    fn default() -> Self {
        let task_type_id = TypeId::of::<T>();
        let (popn_calculate, calculate_hirshfeld) =
            if task_type_id == TypeId::of::<BandStructureParam>() {
                (false, false)
            } else {
                (true, true)
            };
        Self {
            xc_functional: XCFunctional::PBE,
            spin_polarized: true,
            spin: 0,
            opt_strategy: OptStrategy::Speed,
            page_wvfns: 0,
            cut_off_energy: 0.0,
            grid_scale: 1.5,
            fine_grid_scale: 1.5,
            finite_basis_corr: FiniteBasisCorr::No,
            elec_energy_tol: 1e-5,
            max_scf_cycles: 6000,
            fix_occupancy: false,
            metals_method: MetalsMethod::default(),
            perc_extra_bands: 72,
            smearing_width: 0.1,
            spin_fix: 6,
            num_dump_cycles: 0,
            calculate_elf: false,
            calculate_stress: false,
            popn_calculate,
            calculate_hirshfeld,
            calculate_densdiff: false,
            pdos_calculate_weights: true,
            extra_setting: T::default(),
        }
    }
}

impl<T> Display for CastepParam<T>
where
    T: Task + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let task_type_id = TypeId::of::<T>();
        let task = if task_type_id == TypeId::of::<GeomOptParam>() {
            "GeometryOptimization"
        } else if task_type_id == TypeId::of::<BandStructureParam>() {
            "BandStructure"
        } else {
            panic!("Unsupported task type!")
        };
        let content = format!(
            r#"task : {}
comment : CASTEP calculation from Materials Studio
xc_functional : {}
spin_polarized : {}
spin :        {}
opt_strategy : {}
page_wvfns :        {}
cut_off_energy :      {:18.15}
grid_scale :        {:18.15}
fine_grid_scale :        {:18.15}
finite_basis_corr :        {}
elec_energy_tol :   {:18.15e}
max_scf_cycles :     {}
fix_occupancy : {}
{}
perc_extra_bands : {}
smearing_width :        {:18.15}
spin_fix :        {}
num_dump_cycles : {}
{}
calculate_ELF : {}
calculate_stress : {}
popn_calculate : {}
calculate_hirshfeld : {}
calculate_densdiff : {}
pdos_calculate_weights : {}
"#,
            task,
            self.xc_functional,
            self.spin_polarized,
            self.spin,
            self.opt_strategy,
            self.page_wvfns,
            self.cut_off_energy,
            self.grid_scale,
            self.fine_grid_scale,
            self.finite_basis_corr,
            self.elec_energy_tol,
            self.max_scf_cycles,
            self.fix_occupancy,
            self.metals_method,
            self.perc_extra_bands,
            self.smearing_width,
            self.spin_fix,
            self.num_dump_cycles,
            self.extra_setting,
            self.calculate_elf,
            self.calculate_stress,
            self.popn_calculate,
            self.calculate_hirshfeld,
            self.calculate_densdiff,
            self.pdos_calculate_weights
        );
        write!(f, "{}", content)
    }
}

impl From<CastepParam<GeomOptParam>> for CastepParam<BandStructureParam> {
    fn from(geom_param: CastepParam<GeomOptParam>) -> Self {
        CastepParam {
            spin: geom_param.spin,
            cut_off_energy: geom_param.cut_off_energy,
            metals_method: geom_param.metals_method,
            ..Default::default()
        }
    }
}
