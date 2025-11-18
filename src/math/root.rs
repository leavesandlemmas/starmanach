type Real = f64;

pub struct Solver {
    maxiter: u64,
    atol: Real,
    rtol: Real,
}


impl Solver {
    pub fn new() -> Self {
        Self {
            maxiter: 100,
            atol: 1e-12,
            rtol: 1e-12,
        }
    }

    pub fn solve(&self, f: impl Fn(Real) -> Real, a: Real, b: Real) -> Result<Solution, SolveError> {
        let left = FnGraph::lift(&f, a);
        let right = FnGraph::lift(&f, b);

        if self.is_zero(&left) {
            return Ok(Solution::from_graph(left, 0));
        }

        if self.is_zero(&right) {
            return Ok(Solution::from_graph(right, 0));
        }

        let mut bracket = Bracket::make(left, right)?;

        for i in 1..=self.maxiter {
            let mid = bracket.midpoint();
            let proposal = FnGraph::lift(&f, mid);

            if self.is_zero(&proposal) {
                return Ok(Solution::from_graph(proposal, i));
            }

            if self.is_close(&bracket) {
                return Ok(Solution::from_graph(proposal, i));
            }

            bracket.update(proposal);
        }

        let out = SolveError {
            message: format!("Reached maximum iterations: {}", self.maxiter),
        };
        Err(out)
    }

    fn is_zero(&self, p: &FnGraph) -> bool {
        p.y.abs() < self.atol + self.rtol * p.x.abs()
    }

    fn is_close(&self, b: &Bracket) -> bool {
        
        let norm : Real = b.left.x.abs().max(b.right.x.abs());
        (b.left.x - b.right.x).abs() < self.atol + self.rtol * norm
    }
}

struct FnGraph {
    pub x: Real,
    pub y: Real,
}

impl FnGraph {
    fn lift(f: impl Fn(Real) -> Real, x: Real) -> Self {
        Self { x, y: f(x) }
    }
}

struct Bracket {
    left: FnGraph,
    right: FnGraph,
}

impl Bracket {
    fn make(left: FnGraph, right: FnGraph) -> Result<Self, SolveError> {
        if same_sign(left.y, right.y) {
            Err(SolveError {
                message: "Invalid Bracket".to_string(),
            })
        } else {
            Ok(Self { left, right })
        }
    }

    fn update(&mut self, proposal: FnGraph) {
        if same_sign(self.left.y, proposal.y) {
            self.left = proposal;
        } else {
            self.right = proposal;
        }
    }

    fn midpoint(&self) -> Real {
        0.5 * (self.left.x + self.right.x)
    }

    fn secant(&self) -> Real {
        let num = self.left.x * self.right.y - self.right.x * self.left.y;
        let denom = self.right.y - self.left.y;
        num / denom      
    }
}

fn same_sign(x: Real, y: Real) -> bool {
    x * y > 0.0
}


pub struct Solution {
    root : Real, 
    residual : Real,
    iteration : u64
}

impl Solution {
    
    fn from_graph(p: FnGraph, i : u64) -> Self {
        Self {root : p.x, residual : p.y, iteration : i}
    }
}


impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Root = {}\nResidual = {}\nIteration = {}", self.root, self.residual, self.iteration)
    }
}

// standard imports
use std::error::Error;
use std::fmt;

// Errors for lexical analysis
#[derive(Debug)]
pub struct SolveError {
    message: String,
}

impl SolveError {
    pub fn new(message: String) -> Self {
        SolveError { message }
    }
}

impl fmt::Display for SolveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to find a zero. Solver reports: {}", self.message)
    }
}

impl Error for SolveError {}
