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

use std::cmp::Reverse;

use crate::common::Solution;

fn solve_a(ranges: &[Vec<u32>]) -> u32 {
    ranges
        .iter()
        .map(|batteries| {
            let (li, l) = batteries[..batteries.len() - 1]
                .iter()
                .enumerate()
                .max_by_key(|(i, b)| (**b, Reverse(*i)))
                .unwrap();
            let r = batteries.iter().skip(li + 1).max().unwrap();
            l * 10 + r
        })
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let banks: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    (solve_a(&banks).to_string(), "".to_string())
}
