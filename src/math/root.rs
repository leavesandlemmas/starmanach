use crate::math::real::Real; 


pub struct Solver {
    pub maxiter: u64,
    pub atol: Real,
    pub rtol: Real,
}


impl Solver {
    pub fn new() -> Self {
        Self {
            maxiter: 50,
            atol: 1e-16,
            rtol: 4.0 * Real::EPSILON,
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
            let c = bracket.propose();
                     
            let proposal = FnGraph::lift(&f, c);

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

    pub fn solve_with_jac(&self, f: impl Fn(Real) -> Real, df : impl Fn(Real) -> Real, a: Real, b: Real) -> Result<Solution, SolveError> {
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
            
            let c = bracket.propose_with_jac(&df);
              
            let proposal = FnGraph::lift(&f, c);

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
        
        b.width() < self.atol + self.rtol * b.norm()
    }
}

#[derive(Copy, Clone)]
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
    last: FnGraph,
    best: FnGraph,
    contrapoint: FnGraph,
}

impl Bracket {
    fn make(left: FnGraph, right: FnGraph) -> Result<Self, SolveError> {
        if same_sign(left.y, right.y) {
            return Err(SolveError {
                message: format!("Invalid Bracket: ({}, {}), ({}, {})", left.x, left.y, right.x, right.y),
            });
        }   
        if left.y.abs() < right.y.abs() {
            Ok(Self {last : right, best : left, contrapoint: right} )
        } else {
            Ok(Self {last : left, best : right, contrapoint: left})
        }       
    }

    fn update(&mut self, proposal: FnGraph) {
        self.last = self.best; 
        self.best = proposal;
        if !same_sign(self.last.y, self.best.y) {
            self.contrapoint = self.last;   
        }
        if self.contrapoint.y.abs() < self.best.y.abs() {
            std::mem::swap(&mut self.contrapoint, &mut self.best);
        }
    }

    fn midpoint(&self) -> Real {
        0.5 * (self.best.x + self.contrapoint.x)
    }

    fn secant(&self) -> Real {
        let num = self.last.x * self.best.y - self.best.x * self.last.y;
        let denom = self.best.y - self.last.y;
        num / denom      
    }

    fn propose(&self) -> Real {
        let mid = self.midpoint();
        let sec = self.secant();
        if is_between(sec, self.best.x, mid) {
            sec
        } else {
            mid
        }
    }
    
    fn propose_with_jac(&self, df : impl Fn(Real) -> Real) -> Real {
        let mid = self.midpoint();
        let s = self.best.x - self.best.y / df(self.best.x);
        if is_between(s, self.best.x, self.contrapoint.x) {
            s
        } else {
            mid
        }
    }

    fn width(&self) -> Real {
        (self.best.x - self.contrapoint.x).abs()
    }
    
    fn norm(&self) -> Real {
        Real::max(self.best.x.abs(), self.contrapoint.x.abs())
    }
}

fn same_sign(x: Real, y: Real) -> bool {
    x * y > 0.0
}

fn is_between(x : Real, a :Real, b : Real) -> bool {
    let l = a < x;
    let r = x < b;
    !(l ^ r)
}


pub struct Solution {
    pub root : Real, 
    pub residual : Real,
    pub iteration : u64
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
