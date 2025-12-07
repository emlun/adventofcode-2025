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

use std::collections::HashSet;

use crate::common::Solution;

pub fn solve(lines: &[String]) -> Solution {
    let (_, splits): (HashSet<usize>, usize) = lines
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .fold((HashSet::new(), 0), |(beams, splits), line| {
            line.chars()
                .enumerate()
                .fold((beams, splits), |(mut beams, splits), (i, ch)| {
                    if ch == '^' {
                        if beams.remove(&i) {
                            beams.insert(i - 1);
                            beams.insert(i + 1);
                            (beams, splits + 1)
                        } else {
                            (beams, splits)
                        }
                    } else if ch == 'S' {
                        beams.insert(i);
                        (beams, splits)
                    } else {
                        (beams, splits)
                    }
                })
        });
    (splits.to_string(), "".to_string())
}
