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
    [
        (r - 1, c - 1),
        (r - 1, *c),
        (r - 1, c + 1),
        (*r, c - 1),
        (*r, c + 1),
        (r + 1, c - 1),
        (r + 1, *c),
        (r + 1, c + 1),
    ]
    .into_iter()
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
        .map(|(r, el)| (r + 1, el))
        .fold(
            (
                vec![vec![false; lines[0].len() + 2]; lines.len() + 2],
                VecDeque::with_capacity(lines.len() * lines[0].len() / 2),
            ),
            |(grid, rolls), (r, line)| {
                line.chars().enumerate().map(|(c, el)| (c + 1, el)).fold(
                    (grid, rolls),
                    move |(mut grid, mut rolls), (c, ch)| {
                        if ch == '@' {
                            grid[r][c] = true;
                            rolls.push_back((r, c));
                        }
                        (grid, rolls)
                    },
                )
            },
        );
    (
        solve_a(&grid, rolls.as_slices()).to_string(),
        solve_b(grid, rolls).to_string(),
    )
}
