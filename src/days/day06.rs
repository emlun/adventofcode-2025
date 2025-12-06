// Solutions to Advent of Code 2025
// Copyright (C) 2025  Emil Lundberg <emil@emlun.se>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::common::Solution;

struct Problem {
    operands: Vec<u64>,
    mul: bool,
}

fn solve_a(problems: &[Problem]) -> u64 {
    problems
        .iter()
        .map(|Problem { operands, mul }| {
            if *mul {
                operands.iter().copied().product::<u64>()
            } else {
                operands.iter().copied().sum()
            }
        })
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let problems: Vec<Problem> = lines
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .fold(Vec::new(), |problems, line| {
            line.split_ascii_whitespace()
                .enumerate()
                .fold(problems, |mut problems, (i, op)| {
                    if problems.len() <= i {
                        problems.resize_with(i + 1, || Problem {
                            operands: Vec::with_capacity(4),
                            mul: false,
                        });
                    }
                    if op == "+" || op == "*" {
                        problems[i].mul = op == "*";
                    } else {
                        problems[i].operands.push(op.parse().unwrap());
                    }
                    problems
                })
        });
    (solve_a(&problems).to_string(), "".to_string())
}
