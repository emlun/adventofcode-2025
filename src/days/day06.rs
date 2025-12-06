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

#[derive(Debug)]
struct ProblemBParsing {
    operands: Vec<Vec<(usize, u64)>>,
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

fn solve_b(lines: &[String]) -> u64 {
    let problems: Vec<ProblemBParsing> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .enumerate()
        .fold(Vec::new(), |problems: Vec<ProblemBParsing>, (r, line)| {
            let (problems, _, _) = line.chars().enumerate().fold(
                (problems, 0, true),
                |(mut problems, problem_i, last_was_whitespace): (
                    Vec<ProblemBParsing>,
                    usize,
                    bool,
                ),
                 (c, ch)| {
                    if ch == ' ' {
                        if last_was_whitespace {
                            (problems, problem_i, true)
                        } else {
                            (problems, problem_i + 1, true)
                        }
                    } else {
                        if problems.len() <= problem_i {
                            problems.resize_with(problem_i + 1, || ProblemBParsing {
                                operands: Vec::with_capacity(4),
                                mul: false,
                            });
                        }
                        match ch {
                            '+' | '*' => {
                                problems[problem_i].mul = ch == '*';
                                (problems, problem_i, false)
                            }
                            digit => {
                                problems[problem_i].operands.resize_with(r + 1, Vec::new);
                                problems[problem_i].operands[r]
                                    .push((c, digit.to_digit(10).unwrap().into()));
                                (problems, problem_i, false)
                            }
                        }
                    }
                },
            );
            problems
        });
    solve_a(
        &problems
            .into_iter()
            .map(|problem| {
                let min_c = problem
                    .operands
                    .iter()
                    .flat_map(|op| op.iter())
                    .map(|(c, _)| c)
                    .min()
                    .copied()
                    .unwrap();
                let max_c = problem
                    .operands
                    .iter()
                    .flat_map(|op| op.iter())
                    .map(|(c, _)| c)
                    .max()
                    .copied()
                    .unwrap();
                Problem {
                    operands: problem.operands.into_iter().fold(
                        vec![0; max_c + 1 - min_c],
                        |operands, digits| {
                            digits
                                .into_iter()
                                .fold(operands, |mut operands, (c, digit)| {
                                    let cc = c - min_c;
                                    operands[cc] = operands[cc] * 10 + digit;
                                    operands
                                })
                        },
                    ),
                    mul: problem.mul,
                }
            })
            .collect::<Vec<_>>(),
    )
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
    (solve_a(&problems).to_string(), solve_b(lines).to_string())
}
