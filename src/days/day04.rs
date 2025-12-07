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

use std::collections::VecDeque;

use crate::common::Solution;

fn neighbors<'a, 'b>(
    rolls: &'a [Vec<bool>],
    (r, c): &'b (usize, usize),
) -> impl Iterator<Item = (usize, usize)> + use<'a, 'b> {
    let h = rolls.len();
    let w = rolls[0].len();
    (r.saturating_sub(1)..(std::cmp::min(h, r + 2)))
        .flat_map(move |r| (c.saturating_sub(1)..(std::cmp::min(w, c + 2))).map(move |c| (r, c)))
        .filter(|rrcc| *rrcc != (*r, *c))
        .filter(|(rr, cc)| rolls[*rr][*cc])
}

fn solve_a(
    grid: &[Vec<bool>],
    (rolls_a, rolls_b): (&[(usize, usize)], &[(usize, usize)]),
) -> usize {
    rolls_a
        .iter()
        .chain(rolls_b.iter())
        .filter(|rc| neighbors(grid, rc).count() < 4)
        .count()
}

fn solve_b(mut rolls: Vec<Vec<bool>>, mut remove_queue: VecDeque<(usize, usize)>) -> usize {
    let mut removed = 0;
    while let Some((r, c)) = remove_queue.pop_front() {
        if rolls[r][c] && neighbors(&rolls, &(r, c)).count() < 4 {
            removed += 1;
            rolls[r][c] = false;
            for rrcc in neighbors(&rolls, &(r, c)) {
                remove_queue.push_back(rrcc);
            }
        }
    }
    removed
}

pub fn solve(lines: &[String]) -> Solution {
    let (grid, rolls): (Vec<Vec<bool>>, VecDeque<(usize, usize)>) = lines
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .enumerate()
        .fold(
            (
                Vec::with_capacity(lines.len()),
                VecDeque::with_capacity(lines.len() * lines[0].len() / 2),
            ),
            |(mut grid, rolls), (r, line)| {
                let (row, rolls) = line.chars().enumerate().fold(
                    (Vec::with_capacity(line.len()), rolls),
                    move |(mut row, mut rolls), (c, ch)| {
                        if ch == '@' {
                            row.push(true);
                            rolls.push_back((r, c));
                        } else {
                            row.push(false);
                        }
                        (row, rolls)
                    },
                );
                grid.push(row);
                (grid, rolls)
            },
        );
    (
        solve_a(&grid, rolls.as_slices()).to_string(),
        solve_b(grid, rolls).to_string(),
    )
}
