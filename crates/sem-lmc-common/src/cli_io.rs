use sem_lmc_algorithm::ast::{
    fixpoint_system::FixEq, symbolic_exists_moves::SymbolicExistsMove,
};

pub trait SpecOutput {
    /// Gets the system of fixpoint equation
    fn get_sys(&self) -> Result<Vec<FixEq>, Box<dyn std::error::Error>>;

    /// Gets the symbolic existential-moves
    fn get_sem(
        &self,
    ) -> Result<Vec<SymbolicExistsMove>, Box<dyn std::error::Error>>;

    /// Execute the local algorithm and return the result wrapped in a string.
    fn get_ver(&self) -> String;
}
