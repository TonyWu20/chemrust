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

    /// for amd aocl
    pub fn content(&self) -> String {
        let Self { nodes, job_name } = self;
        [
            r#"#PBS -N HPL_short_run
#PBS -q simple_q
#PBS -l walltime=168:00:00"#
                .to_string(),
            format!("#PBS -l nodes=1:ppn={nodes}"),
            r#"#PBS -V

cd $PBS_O_WORKDIR

NCPU=`wc -l < $PBS_NODEFILE`
NNODES=`uniq $PBS_NODEFILE | wc -l`

echo ------------------------------------------------------
echo ' This job is allocated on '${NCPU}' cpu(s)'
echo 'Job is running on node(s): '
cat 
echo ------------------------------------------------------
echo PBS: qsub is running on $PBS_O_HOST
echo PBS: originating queue is $PBS_O_QUEUE
echo PBS: executing queue is $PBS_QUEUE
echo PBS: working directory is $PBS_O_WORKDIR
echo PBS: execution mode is $PBS_ENVIRONMENT
echo PBS: job identifier is $PBS_JOBID
echo PBS: job name is $PBS_JOBNAME
echo PBS: node file is $PBS_NODEFILE
echo PBS: number of nodes is $NNODES
echo PBS: current home directory is $PBS_O_HOME
echo PBS: PATH = $PBS_O_PATH
echo ------------------------------------------------------


cat  >./hostfile"#
                .to_string(),
            format!(
            r#"mpirun \
-x OMP_NUM_THREADS=1 \
-x BLIS_NUM_THREADS=1 \
-x LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/data/software/aocl510/5.1.0/gcc/lib \
--np $NCPU --mca btl ^tcp --hostfile hostfile --map-by numa --bind-to numa /data/software/CASTEP-6.11_aocl/castep.mpi {job_name} >debug.log 2>&1"#),
            "rm ./hostfile".to_string(),
        ]
        .join("\n")
    }
}

impl Display for PbsScript {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content())
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
