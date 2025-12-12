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

struct Present {
    w: usize,
    h: usize,
    pixels: usize,
}

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
                let (bboxw, bboxh) = num_presents
                    .iter()
                    .zip(presents)
                    .map(|(n, p)| if *n > 0 { (p.w, p.h) } else { (0, 0) })
                    .fold((0, 0), |(maxw, maxh), (w, h)| {
                        (std::cmp::max(maxw, w), std::cmp::max(maxh, h))
                    });
                let space_bboxed = (dimx / bboxw) * (dimy / bboxh);

                if num_presents.iter().copied().sum::<usize>() <= space_bboxed {
                    true
                } else {
                    let presents_size: usize = num_presents
                        .iter()
                        .enumerate()
                        .map(|(i, n)| n * presents[i].pixels)
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
    let (presents, regions): (Vec<Present>, Vec<Region>) = lines
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
                    presents.push(Present {
                        w: 0,
                        h: 0,
                        pixels: 0,
                    });
                } else {
                    let p = presents.last_mut().unwrap();
                    p.w = std::cmp::max(p.w, line.len());
                    p.h += 1;
                    p.pixels += line.chars().filter(|ch| *ch == '#').count();
                }
                (presents, regions)
            },
        );

    (solve_a(&presents, &regions).to_string(), "".to_string())
}
