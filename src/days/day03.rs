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

fn select_battery(bank: &[u32], prev_batteries: &[usize], num_batteries: usize) -> usize {
    let (ri, _) = bank[..bank.len() - (num_batteries - 1 - prev_batteries.len())]
        .iter()
        .enumerate()
        .skip(prev_batteries.last().copied().map(|i| i + 1).unwrap_or(0))
        .max_by_key(|(bi, b)| (**b, Reverse(*bi)))
        .unwrap();
    ri
}

fn solve_b(banks: &[Vec<u32>], num_batteries: usize) -> u64 {
    banks
        .iter()
        .map(|bank| {
            let selected_bis = (0..num_batteries).fold(vec![], |mut selected_bis, _| {
                let bi = select_battery(bank, &selected_bis, num_batteries);
                selected_bis.push(bi);
                selected_bis
            });
            selected_bis
                .into_iter()
                .fold(0_u64, |acc, bi| acc * 10 + u64::from(bank[bi]))
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

    (
        solve_b(&banks, 2).to_string(),
        solve_b(&banks, 12).to_string(),
    )
}
