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

use std::collections::HashMap;

use crate::common::Solution;

pub fn solve(lines: &[String]) -> Solution {
    let (paths, sol_a): (HashMap<usize, u64>, usize) = lines
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .fold((HashMap::new(), 0), |(paths, splits), line| {
            line.chars()
                .enumerate()
                .fold((paths, splits), |(mut paths, splits), (i, ch)| {
                    if ch == '^' {
                        if let Some(num_paths) = paths.remove(&i) {
                            *paths.entry(i - 1).or_insert(0) += num_paths;
                            *paths.entry(i + 1).or_insert(0) += num_paths;
                            (paths, splits + 1)
                        } else {
                            (paths, splits)
                        }
                    } else if ch == 'S' {
                        paths.insert(i, 1);
                        (paths, splits)
                    } else {
                        (paths, splits)
                    }
                })
        });
    let sol_b: u64 = paths.values().sum();
    (sol_a.to_string(), sol_b.to_string())
}
