use std::fmt::Display;

pub trait ServerScript {
    fn set_job_name(&mut self, job_name: &str);
    fn set_cores_per_node(&mut self, cores_num: u32);
}

#[derive(Debug, Clone)]
pub struct LsfScript {
    app_name: String,
    np: u32,
    np_per_node: u32,
    omp_num_threads: u32,
    run: String,
    path_to_castep: String,
    job_name: String,
}

impl LsfScript {
    pub fn new(
        app_name: String,
        np: u32,
        np_per_node: u32,
        omp_num_threads: u32,
        run: String,
        path_to_castep: String,
        job_name: String,
    ) -> Self {
        Self {
            app_name,
            np,
            np_per_node,
            omp_num_threads,
            run,
            path_to_castep,
            job_name,
        }
    }
}

impl ServerScript for LsfScript {
    fn set_job_name(&mut self, job_name: &str) {
        self.job_name = job_name.to_string()
    }

    fn set_cores_per_node(&mut self, cores_num: u32) {
        self.np = cores_num;
        self.np_per_node = cores_num;
    }
}

impl Display for LsfScript {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let LsfScript {
            app_name,
            np,
            np_per_node,
            omp_num_threads,
            run,
            path_to_castep,
            job_name,
        } = self;
        let content = [
            format!("APP_NAME={app_name}"),
            format!("NP={np}"),
            format!("NP_PER_NODE={np_per_node}"),
            format!("OMP_NUM_THREADS={omp_num_threads}"),
            format!("RUN=\"{run}\"\n"),
            format!("{path_to_castep} -np  {job_name}"),
        ]
        .join("\n");
        write!(f, "{content}")
    }
}

impl Default for LsfScript {
    fn default() -> Self {
        Self {
            app_name: "intelY_mid".to_string(),
            np: 8,
            np_per_node: 8,
            omp_num_threads: 1,
            run: "RAW".to_string(),
            path_to_castep: "/home-yw/Soft/msi/MS70/MaterialsStudio7.0/etc/CASTEP/bin/RunCASTEP.sh"
                .to_string(),
            job_name: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PbsScript {
    nodes: u32,
    job_name: String,
}

impl PbsScript {
    pub fn new(nodes: u32, job_name: String) -> Self {
        Self { nodes, job_name }
    }
}

impl Display for PbsScript {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { nodes, job_name } = self;
        let content = [
            r#"#PBS -N HPL_short_run
#PBS -q simple_q
#PBS -l walltime=168:00:00"#
                .to_string(),
            format!("#PBS -l nodes=1:ppn={nodes}"),
            r#"#PBS -V

cd 

NCPU=`wc -l < `
NNODES=`uniq  | wc -l`

echo ------------------------------------------------------
echo ' This job is allocated on '' cpu(s)'
echo 'Job is running on node(s): '
cat 
echo ------------------------------------------------------
echo PBS: qsub is running on 
echo PBS: originating queue is 
echo PBS: executing queue is 
echo PBS: working directory is 
echo PBS: execution mode is 
echo PBS: job identifier is 
echo PBS: job name is 
echo PBS: node file is 
echo PBS: number of nodes is 
echo PBS: current home directory is 
echo PBS: PATH = 
echo ------------------------------------------------------

##For openmpi-intel
##export LD_LIBRARY_PATH=/share/apps/openmpi-1.8.8-intel/lib:
##export PATH=/share/apps/openmpi-1.8.8-intel/bin:

cat  >./hostfile"#
                .to_string(),
            format!("mpirun --mca btl ^tcp --hostfile hostfile /home/bhuang/castep.mpi {job_name}"),
            "rm ./hostfile".to_string(),
        ]
        .join("\n");
        write!(f, "{content}")
    }
}

impl ServerScript for PbsScript {
    fn set_job_name(&mut self, job_name: &str) {
        self.job_name = job_name.to_string()
    }

    fn set_cores_per_node(&mut self, cores_num: u32) {
        self.nodes = cores_num
    }
}
