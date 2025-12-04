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

fn solve_a(moves: &[i32]) -> usize {
    moves
        .iter()
        .scan(50, |pos, step| {
            *pos = (((*pos + step) % 100) + 100) % 100;
            Some(*pos)
        })
        .filter(|pos| *pos == 0)
        .count()
}

fn solve_b(moves: &[i32]) -> i32 {
    moves
        .iter()
        .scan(50, |pos, step| {
            let zero_dist = if step.is_positive() {
                100 - *pos
            } else if *pos == 0 {
                100
            } else {
                *pos
            };
            let zeros =
                if step.abs() >= zero_dist { 1 } else { 0 } + (step.abs() - zero_dist) / 100;
            *pos = (((*pos + step) % 100) + 100) % 100;
            Some(zeros)
        })
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let moves: Vec<i32> = lines
        .iter()
        .map(|line| {
            let sign = if line[0..1] == *"R" { 1 } else { -1 };
            sign * line[1..].parse::<i32>().unwrap()
        })
        .collect();

    (solve_a(&moves).to_string(), solve_b(&moves).to_string())
}
