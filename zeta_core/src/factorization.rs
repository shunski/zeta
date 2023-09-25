use omega_core::Algorithm;
use super::Ubig;

struct QuadraticSieve {
    n: Ubig;
}

impl Algorithm for QuadraticSieve {
    type Output = Ubig;
    fn compute(self) -> Vec<Self::Output> {
        1 * self.n
    }
}