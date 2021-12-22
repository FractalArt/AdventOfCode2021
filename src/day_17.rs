//! # Advent of Code 2021 - Day 17
//!
//! This module contains the solution of the [seventeenth day's challenges](https://adventofcode.com/2021/day/17).
//!
//! We consider a two-dimensional dynamical system with initial condition
//!
//! x(t=0) = x0 = y(t=0) = y0 = 0
//!
//! that is described by two uncoupled second order differential equations
//!
//! x'' = -1
//! y'' = -1
//!
//! where the deceleration in x is due to drag and that in y due to gravity (with m=1, g=1).
//! Solving this system using the initial conditions we get the solutions
//!
//! x(t) = vx0 * t - 1/2 * t^2
//! y(t) = vy0 * t - 1/2 * t^2
//!
//! where `vx0` and `vy0` are the initial velocities that we are looking for.
//!
//! The equations for the velocities in `x` and `y` directions can be obtained by differentiation:
//!
//!  vx(t) = vx0 - t
//!  vy(t) = vy0 - t
//!
//! If we know that the target area is given by `x_min` < x(t) < `x_max` with `x_min > 0`, then we
//! can compute the time step at which enter the region, which is given by
//!
//! t_enter_x = vx0 - sqrt(vx0²- x_min)
//!
//! From this equation, we can infer that that vx0 > sqrt(`x_min`).
//! We assume that we need to shot upwards at least, so we assume vy0 > 1.
//!
//! Furthermore we only need to consider velocities below a limit that  overshoots the target
//! domain after one time step.
//!
//!  x(1) = vx0 - 1/2 < x_max => vx0 < x_max + 1/2 < x_max + 1
//!  y(1) = vy0 - 1/2 > y_min => vy0 > y_min + 1/2 => -vy0 < -y_min - 1/2
//!
//! In the equation for the y direction, vy0 < 0 to immediately reach the lower bound after one step. Hence the absolute value
//! of the velocity |vy| < - y_min.
use itertools::Itertools;

/// Simulate the evolution of the system for a given set of initial conditions and the target area.
///
/// If the probe hits the target area for the given initial condition the maximal y value is returned
/// in an `Option`, otherwise `None`.
fn simulate(
    vx: isize,
    vy: isize,
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
) -> Option<isize> {
    // Initial position
    let (mut x, mut y) = (0, 0);

    // Initial velocity
    let (mut vx, mut vy) = (vx, vy);

    // Track maximal y position
    let mut current_y_max = y;

    loop {
        x += vx;
        y += vy;
        vx += match vx {
            v if v > 0 => -1,
            v if v < 0 => 1,
            _ => 0,
        };
        vy -= 1;

        if y >= current_y_max {
            current_y_max = y;
        }

        // If we enter the target area, we are done
        if y >= y_min && y <= y_max && x >= x_min && x <= x_max {
            return Some(current_y_max);
        }

        if y < y_min {
            break;
        }
    }

    None
}

/// Compute the maximal y-value for those trajectories that reach the target area.
pub fn day_17_1(x_min: isize, x_max: isize, y_min: isize, y_max: isize) -> isize {
    let vxstart = (x_min as f64).sqrt() as isize;
    let vystart = 0;
    let vxend = x_max + 1;
    let vyend = -y_min;

    (vxstart..=vxend)
        .cartesian_product(vystart..=vyend)
        .filter_map(|(vx, vy)| simulate(vx, vy, x_min, x_max, y_min, y_max))
        .max()
        .unwrap()
}

pub fn day_17_2(x_min: isize, x_max: isize, y_min: isize, y_max: isize) -> usize {
    let vxstart = (x_min as f64).sqrt() as isize;
    let vystart = y_min;
    let vxend = x_max + 1;
    let vyend = -y_min;

    (vxstart..=vxend)
        .cartesian_product(vystart..=vyend)
        .filter_map(|(vx, vy)| simulate(vx, vy, x_min, x_max, y_min, y_max))
        .count()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_simulate() {
        assert_eq!(simulate(7, 2, 20, 30, -10, -5), Some(3));
        assert_eq!(simulate(6, 3, 20, 30, -10, -5), Some(6));
        assert_eq!(simulate(9, 0, 20, 30, -10, -5), Some(0));
        assert_eq!(simulate(6, 9, 20, 30, -10, -5), Some(45));
    }

    #[test]
    fn test_day_17_1() {
        assert_eq!(day_17_1(20, 30, -10, -5), 45);
    }

    #[test]
    fn test_day_17_2() {
        assert_eq!(day_17_2(20, 30, -10, -5), 112);
    }
}
