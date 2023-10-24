use std::{fmt::Display, time::Duration};

pub struct VerificationOutput {
    pub preproc_time: Duration,
    pub algorithm_time: Duration,
    pub result: String,
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
