use crate::parity_game::player::Player;
use crate::parity_game::parity_game::FixpointType;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Counter( Vec<i32>);

impl Counter {
    
    /// Implements a total order for the counter:
    /// 
    ///  - it is the case that $k <_\exists k'$, whenever
    ///    the largest $i$ such that $k_i\neq k'_i$ is the 
    ///    index of a greatest fixpoint, that is to say whenever
    ///    `self.fix_types[i]` has value `Max`, or if the value is
    ///    `Min` and $k_i > k'_i$
    ///  - we say $k <_\forall k'$ whenever it is not true that
    ///    $k' <_\exists k$
    ///  - $k \leq_P k'$ whenever $k <_P k$ or $k = k$.
    /// 
    /// > Notation: $k$, $k'$ are vectors, and $P\in \{\exists, \forall\}$.
    pub fn leq_p(k1: &Counter, k2: &Counter, player: &Player, fix: &Vec<FixpointType> ) -> bool {

        let mut i: usize = 0;
        let mut n: usize = 1;
        let m = k1.0.len();
        while n <= m {
            if k1.0[n] != k2.0[n] {
                i = n;
            }
            n = n + 1;
        }

        if i == 0 {
            true
        } else {
            let result = match fix[i] {
                FixpointType::Max => k1.0[i] < k2.0[i],
                FixpointType::Min => k1.0[i] > k2.0[i],
            };

            match player {
                Player::Existential => result,
                Player::Universal => ! result,
            }
        }
        
    }

    /// Updates the counter, such that each non-zero priority (argument `i`) is associated 
    /// with the number of times the priority has been encountered in the play 
    /// since a higher priority was last faced. Note that:
    /// 
    ///  - $next(k, 0) = k$
    ///  -  $next(k, i) = k'$.
    /// 
    pub fn next(&self, i: usize) -> Vec<i32> {
        let m = self.0.len();
        let mut k = vec![0; self.0.len()];

        if i == 0 {
            k
        } else {
            k[i] = self.0[i] + 1;
            for j in i + 1..m + 1 {
                k[j] = self.0[j]
            }
            k
        }
    }
}