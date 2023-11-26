use rustc_hash::FxHashMap as HashMap;
use std::{fmt::Display, time::Duration};

use sem_sfe_algorithm::ast::{
    fixpoint_system::FixEq, symbolic_moves_composed::SymbolicExistsMoves,
};

pub struct PreProcOutput {
    pub moves: SymbolicExistsMoves,
    pub fix_system: Vec<FixEq>,
    pub var_map: HashMap<String, String>,
    pub var: String,
    pub preproc_time: Duration,
}

impl PreProcOutput {
    pub fn print_explain(&self) {
        println!("Fixpoint system:\n");
        self.fix_system.iter().for_each(|x| println!("{};", x));

        println!("\nSymbolic exists-moves:\n\n{}", self.moves);
        println!("\n{}", self)
    }
}

impl Display for PreProcOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Preprocessing took: {} sec.", self.preproc_time.as_secs_f32())
    }
}

pub struct VerificationOutput {
    pub algorithm_time: Duration,
    pub result: String,
}

impl Display for VerificationOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Solving the verification task took: {} sec.\n\
             Result: {}",
            self.algorithm_time.as_secs_f32(),
            self.result
        )
    }
}

pub struct InputFlags {
    pub normalize: bool,
}

pub trait SpecOutput {
    fn pre_proc(&self, flags: &InputFlags) -> Result<PreProcOutput, Box<dyn std::error::Error>>;
    /// Execute the local algorithm and return the result wrapped in a string.
    fn verify(
        &self,
        flags: &InputFlags,
        pre_proc: &PreProcOutput,
    ) -> Result<VerificationOutput, Box<dyn std::error::Error>>;
}
