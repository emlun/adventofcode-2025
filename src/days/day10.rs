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

#[derive(Debug, Eq, PartialEq)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<i64>,
}
impl Machine {}

fn fewest_presses(lights: u64, buttons: &[u64]) -> Option<usize> {
    if lights == 0 {
        Some(0)
    } else if buttons.is_empty() {
        None
    } else {
        [
            fewest_presses(lights ^ buttons[0], &buttons[1..]).map(|i| i + 1),
            fewest_presses(lights, &buttons[1..]),
        ]
        .into_iter()
        .flatten()
        .min()
    }
}

fn solve_a(machines: &[Machine]) -> usize {
    machines
        .iter()
        .flat_map(
            |Machine {
                 lights, buttons, ..
             }| {
                let lights: u64 = lights
                    .iter()
                    .enumerate()
                    .map(|(i, b)| if *b { 1 << i } else { 0 })
                    .sum();
                let buttons: Vec<u64> = buttons
                    .iter()
                    .map(|lights| lights.iter().map(|i| 1 << i).sum())
                    .collect();
                fewest_presses(lights, &buttons)
            },
        )
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let machines: Vec<Machine> = lines
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .flat_map(|line| {
            let line = line.strip_prefix("[")?;
            let (lights, line) = line.split_once(']')?;
            let (buttons, joltage) = line.split_once('{')?;
            let joltage = joltage.trim().strip_suffix("}")?;
            Some(Machine {
                lights: lights.trim().chars().map(|ch| ch == '#').collect(),
                buttons: buttons
                    .trim()
                    .split_ascii_whitespace()
                    .flat_map(|btn| {
                        Some(
                            btn.strip_prefix("(")?
                                .strip_suffix(")")?
                                .split(',')
                                .flat_map(|digits| digits.parse().ok())
                                .collect::<Vec<_>>(),
                        )
                    })
                    .collect(),
                joltage: joltage
                    .split(',')
                    .flat_map(|digits| digits.parse().ok())
                    .collect(),
            })
        })
        .collect();

    (solve_a(&machines).to_string(), "".to_string())
}
