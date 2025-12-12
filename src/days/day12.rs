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

use std::{
    collections::HashSet,
    fmt::{Debug, Formatter},
};

use crate::common::Solution;

enum Rot {
    Once,
    Twice,
    Thrice,
}

struct Present {
    dim: usize,
    pixels: Vec<HashSet<(usize, usize)>>,
}

impl Present {
    fn new(dim: usize, pixels: HashSet<(usize, usize)>) -> Self {
        debug_assert!(pixels.iter().all(|(x, y)| *x < dim && *y < dim));
        let rot1 = Self::rotate(dim, &pixels, Rot::Once);
        let rot2 = Self::rotate(dim, &pixels, Rot::Twice);
        let rot3 = Self::rotate(dim, &pixels, Rot::Thrice);
        Self {
            dim,
            pixels: [rot1, rot2, rot3]
                .into_iter()
                .flat_map(|rot| {
                    let flipx = Self::flip(dim, &rot, true);
                    let flipy = Self::flip(dim, &rot, false);
                    [rot, flipx, flipy]
                })
                .fold(vec![pixels], |mut all, next| {
                    if !all.contains(&next) {
                        all.push(next);
                    }
                    all
                }),
        }
    }

    fn rotate(dim: usize, pixels: &HashSet<(usize, usize)>, r: Rot) -> HashSet<(usize, usize)> {
        let d = dim - 1;
        pixels
            .iter()
            .copied()
            .map(|(x, y)| match r {
                Rot::Once => (d - y, x),
                Rot::Twice => (d - x, d - y),
                Rot::Thrice => (y, d - x),
            })
            .collect()
    }

    fn flip(dim: usize, pixels: &HashSet<(usize, usize)>, flip_x: bool) -> HashSet<(usize, usize)> {
        let d = dim - 1;
        pixels
            .iter()
            .copied()
            .map(|(x, y)| if flip_x { (d - x, y) } else { (x, d - y) })
            .collect()
    }

    fn size(&self) -> usize {
        self.pixels[0].len()
    }
}

impl Debug for Present {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let rows: Vec<String> = (0..self.dim)
            .map(|y| {
                let px: Vec<String> = self
                    .pixels
                    .iter()
                    .map(|px| {
                        (0..self.dim)
                            .map(|x| if px.contains(&(x, y)) { '#' } else { '.' })
                            .collect()
                    })
                    .collect();
                px.join("  ")
            })
            .collect();
        write!(f, "{}", rows.join("\n"))
    }
}

#[derive(Debug)]
struct Region {
    dim: (usize, usize),
    num_presents: Vec<usize>,
}

fn solve_a(presents: &[Present], regions: &[Region]) -> usize {
    regions
        .iter()
        .filter(
            |Region {
                 dim: (dimx, dimy),
                 num_presents,
             }| {
                let space = dimx * dimy;
                let bbox = num_presents
                    .iter()
                    .zip(presents)
                    .map(|(n, p)| if *n > 0 { p.dim } else { 0 })
                    .max()
                    .unwrap_or(0);
                let space_bboxed = (dimx / bbox) * (dimy / bbox);

                if num_presents.iter().copied().sum::<usize>() <= space_bboxed {
                    true
                } else {
                    let presents_size: usize = num_presents
                        .iter()
                        .enumerate()
                        .map(|(i, n)| n * presents[i].size())
                        .sum();
                    if presents_size <= space {
                        todo!()
                    } else {
                        false
                    }
                }
            },
        )
        .count()
}

pub fn solve(lines: &[String]) -> Solution {
    let (presents, regions): (Vec<Vec<&str>>, Vec<Region>) = lines
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .fold(
            (Vec::new(), Vec::new()),
            |(mut presents, mut regions), line| {
                if let Some(region) = line.split_once('x').and_then(|(dimx, rest)| {
                    let (dimy, rest) = rest.split_once(':')?;
                    Some(Region {
                        dim: (dimx.parse().ok()?, dimy.parse().ok()?),
                        num_presents: rest
                            .trim()
                            .split(' ')
                            .map(|ch| ch.parse())
                            .collect::<Result<_, _>>()
                            .ok()?,
                    })
                }) {
                    regions.push(region);
                } else if let Some((_, _)) = line.split_once(':') {
                    presents.push(Vec::new());
                } else {
                    let present = presents.last_mut().unwrap();
                    present.push(line.trim());
                }
                (presents, regions)
            },
        );

    let presents: Vec<Present> = presents
        .into_iter()
        .map(|lines| {
            Present::new(
                lines.len(),
                lines
                    .into_iter()
                    .enumerate()
                    .flat_map(|(y, line)| {
                        line.chars()
                            .enumerate()
                            .filter_map(move |(x, ch)| (ch == '#').then_some((x, y)))
                    })
                    .collect(),
            )
        })
        .collect();

    (solve_a(&presents, &regions).to_string(), "".to_string())
}
