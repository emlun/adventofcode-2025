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

fn solve_a(grid: &[Vec<char>]) -> usize {
    (0..grid.len())
        .flat_map(|r| (0..grid[r].len()).map(move |c| (r as isize, c as isize)))
        .filter(|(r, c)| grid[*r as usize][*c as usize] == '@')
        .filter(|(r, c)| {
            (-1_isize..=1)
                .flat_map(|dr| (-1_isize..=1).map(move |dc| (dr, dc)))
                .filter(|drc| *drc != (0, 0))
                .filter(|(dr, dc)| {
                    (0..grid.len() as isize).contains(&(r + dr))
                        && (0..grid[0].len() as isize).contains(&(c + dc))
                })
                .filter(|(dr, dc)| grid[(r + dr) as usize][(c + dc) as usize] == '@')
                .count()
                < 4
        })
        .count()
}

fn solve_b(mut grid: Vec<Vec<char>>) -> usize {
    let mut removed = 0;
    let mut any_changed = true;
    while any_changed {
        any_changed = false;
        for (r, c) in (0..grid.len())
            .flat_map(|r| (0..grid[r].len()).map(move |c| (r as isize, c as isize)))
            .filter(|(r, c)| grid[*r as usize][*c as usize] == '@')
            .filter(|(r, c)| {
                (-1_isize..=1)
                    .flat_map(|dr| (-1_isize..=1).map(move |dc| (dr, dc)))
                    .filter(|drc| *drc != (0, 0))
                    .filter(|(dr, dc)| {
                        (0..grid.len() as isize).contains(&(r + dr))
                            && (0..grid[0].len() as isize).contains(&(c + dc))
                    })
                    .filter(|(dr, dc)| grid[(r + dr) as usize][(c + dc) as usize] == '@')
                    .count()
                    < 4
            })
            .collect::<Vec<_>>()
        {
            removed += 1;
            any_changed = true;
            grid[r as usize][c as usize] = '.';
        }
    }
    removed
}

pub fn solve(lines: &[String]) -> Solution {
    let grid: Vec<Vec<char>> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();
    (solve_a(&grid).to_string(), solve_b(grid).to_string())
}
