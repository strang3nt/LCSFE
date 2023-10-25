use std::{fmt::Display, time::Duration};

use sem_lmc_algorithm::ast::{
    fixpoint_system::FixEq,
    symbolic_exists_moves::{SymbolicExistsMove, SymbolicExistsMoveComposed},
};

pub struct VerificationOutput {
    pub moves: Vec<SymbolicExistsMove>,
    pub moves_composed: Vec<SymbolicExistsMoveComposed>,
    pub fix_system: Vec<FixEq>,
    pub fix_system_normalized: Option<Vec<FixEq>>,
    pub preproc_time: Duration,
    pub algorithm_time: Duration,
    pub result: String,
}

impl VerificationOutput {
    pub fn format_verbose(&self) -> String {
        let fix_system = format!(
            "Fixpoint system: \n\n{}\n\n",
            self.fix_system
                .iter()
                .map(|x| format!("{}", x))
                .collect::<Vec<String>>()
                .join(";\n")
        );
        let normalized_system = match &self.fix_system_normalized {
            None => "".to_owned(),
            Some(system) => format!(
                "Normalized fixpoint system: \n\n{}\n\n",
                system
                    .iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join(";\n")
            ),
        };
        let moves = format!(
            "Symbolic existential-moves: \n\n{}\n\n",
            self.moves
                .iter()
                .map(|x| format!("{}", x))
                .collect::<Vec<String>>()
                .join(";\n")
        );
        let composed_moves = format!(
            "Symbolic existential-moves composed: \n\n{}\n\n",
            self.moves_composed
                .iter()
                .map(|x| format!("{}", x))
                .collect::<Vec<String>>()
                .join(";\n")
        );
        format!(
            "{}{}{}{}{}",
            fix_system, normalized_system, moves, composed_moves, self
        )
    }
}

impl Display for VerificationOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Preprocessing took: {} sec.\n\
             Solving the verification task took: {} sec.\n\
             Result: {}",
            self.preproc_time.as_secs_f32(),
            self.algorithm_time.as_secs_f32(),
            self.result
        )
    }
}

pub struct InputFlags {
    pub normalize: bool,
}

pub trait SpecOutput {
    /// Execute the local algorithm and return the result wrapped in a string.
    fn verify(
        &self,
        flags: &InputFlags,
    ) -> Result<VerificationOutput, Box<dyn std::error::Error>>;
}
