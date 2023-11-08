use omega_core::Algorithm;

struct QuadraticSieve {
    n: u64,
}

impl Algorithm for QuadraticSieve {
    type Output = Vec<u64>;
    fn compute(self) -> Self::Output {
        vec![1 * self.n]
    }

    fn complexity(self) -> omega_core::Complexity {
        omega_core::Complexity::Exponential
    }
}