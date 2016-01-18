//! 3-coloring the [Petersen graph].
//!
//! Outputs a colored graph in GraphViz format.
//!
//! [Petersen graph]: https://en.wikipedia.org/wiki/Petersen_graph

use std::iter;
use std::mem;
use std::process::{self, Command};

use sat::solver::Solver;

extern crate sat;

const VERTICES: usize = 10;
const COLORS: usize = 3;

fn main() {
    // For each vertex, the list of adjacent vertices.
    let mut adj: Vec<_> = iter::repeat(vec![]).take(VERTICES).collect();

    // Inner 5 vertices
    for i in 0..5 {
        if i+2 < 5 {
            adj[i].push(i+2);
        }
        if i+3 < 5 {
            adj[i].push(i+3);
        }
    }

    // Outer 5 vertices
    for i in 5..VERTICES {
        adj[i].push(i-5);
        if i+1 < VERTICES {
            adj[i].push(i+1);
        }
    }
    adj[9].push(5);

    let mut instance = sat::Instance::new();

    // For each vertex, a vector of variables for the possible colors.
    let mut vars = vec![];
    for i in 0..VERTICES {
        vars.push(vec![instance.fresh_var(), instance.fresh_var(), instance.fresh_var()]);

        // Assert that each vertex has at least one color.
        // red OR green OR blue
        instance.assert_any(&[vars[i][0], vars[i][1], vars[i][2]]);

        // Assert that each vertex has at most one color.
        // (red IMPLIES !green) =equiv= (!green OR !red)
        for c1 in 0..COLORS {
            for c2 in 0..c1 {
                instance.assert_any(&[!vars[i][c1], !vars[i][c2]]);
            }
        }
    }

    // Assert that adjacent vertices don't have the same color.
    for (i, js) in adj.iter().enumerate() {
        for &j in js {
            for c in 0..COLORS {
                instance.assert_any(&[!vars[i][c], !vars[j][c]]);
            }
        }
    }

    // Solve using minisat.
    let solver = sat::solver::Dimacs::new(|| {
        let mut c = Command::new("minisat");
        c.stdout(process::Stdio::null());
        c
    });

    let solution = solver.solve(&instance).unwrap();

    // Extract the colors.
    // Make sure there's only one color per vertex.
    let mut colors: Vec<_> = iter::repeat(None).take(VERTICES).collect();
    for i in 0..VERTICES {
        for c in 0..COLORS {
            if solution.get(vars[i][c]) {
                assert!(mem::replace(&mut colors[i], Some(c)).is_none());
            }
        }
    }

    // Output in GraphViz format.
    println!("graph {{");
    for (i, js) in adj.iter().enumerate() {
        println!("    {} [color=\"{}\"];", i,
            ["red", "green", "blue"][colors[i].unwrap()]);

        for j in js {
            println!("    {} -- {};", i, j);
        }
    }
    println!("}}");
}
