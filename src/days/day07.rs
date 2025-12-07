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

pub fn solve(lines: &[String]) -> Solution {
    let mut it = lines
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty());
    let paths = it
        .next()
        .unwrap()
        .chars()
        .map(|ch| if ch == 'S' { 1 } else { 0 })
        .collect();
    let (paths, sol_a): (Vec<u64>, usize) = it.fold((paths, 0), |(paths, splits), line| {
        line.chars()
            .enumerate()
            .fold((paths, splits), |(mut paths, splits), (i, ch)| {
                if ch == '^' && paths[i] > 0 {
                    paths[i - 1] += paths[i];
                    paths[i + 1] += paths[i];
                    paths[i] = 0;
                    (paths, splits + 1)
                } else {
                    (paths, splits)
                }
            })
    });
    let sol_b: u64 = paths.into_iter().sum();
    (sol_a.to_string(), sol_b.to_string())
}
