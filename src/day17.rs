use std::cmp;

enum Dimension {
    X, Y
}

#[derive(Clone, Copy)]
struct TargetArea {
    x: (i16, i16),
    y: (i16, i16),
}

fn parse_bounds(str: &str) -> Option<(Dimension, (i16, i16))> {
    let mut tokens = str.trim().split(&['=', '.'][..]).filter(|&s| s != "");

    let dim = match tokens.next() {
        Some("x") => Dimension::X,
        Some("y") => Dimension::Y,
        _ => return None,
    };

    if let (Some(bound1), Some(bound2), None) = (tokens.next(), tokens.next(), tokens.next())  {
        let bound1 = bound1.parse().ok()?;
        let bound2 = bound2.parse().ok()?;
        if bound2 > bound1 { return Some((dim, (bound1, bound2))) }
        return Some((dim, (bound2, bound1)));
    }

    None
}
fn parse_target_area(str: &str) -> Option<TargetArea> {
    let mut tokens = str.trim().split(&[' ', ','][..]).filter(|&s| s != "");
    if (tokens.next(), tokens.next()) != (Some("target"), Some("area:")) {
        return None;
    }

    if let (Some(token1), Some(token2), None) = (tokens.next(), tokens.next(), tokens.next()) {
        if let (Some((dim1, bounds1)), Some((dim2, bounds2))) =
            (parse_bounds(token1), parse_bounds(token2)) {
            return match (dim1, dim2) {
                (Dimension::X, Dimension::Y) => Some(TargetArea { x: bounds1, y: bounds2 }),
                (Dimension::Y, Dimension::X) => Some(TargetArea { x: bounds2, y: bounds1 }),
                _ => None,
            }
        }
    }

    None
}


fn evaluate_trajectory(target: TargetArea, mut velocity: (i16, i16)) -> Option<i16> {
    debug_assert!(target.x.0 >= 0 && target.x.1 >= 0,
        "evaluate_trajectory cannot handle target areas on the left of the y axis");
    let mut pos = (0, 0);
    let mut highest = 0;

    loop {
        match (target.x.0 <= pos.0, pos.0 <= target.x.1, target.y.0 <= pos.1, pos.1 <= target.y.1) {
            (true, true, true, true) => return Some(highest),
            (_, false, _, _)  => return None,
            (_, _, false, _) if velocity.1 < 0 => return None,
            _ => (),
        }

        pos.0 += velocity.0;
        pos.1 += velocity.1;
        if pos.1 > highest { highest = pos.1; }
        velocity.0 -= i16::signum(velocity.0);
        velocity.1 -=1;
    }
}
fn find_optimal_pair_and_count_possibilities_interbal(target: TargetArea) -> (((i16, i16), i16), u16) {
    debug_assert!(target.x.0 >= 0 && target.x.1 >= 0,
        "find_optimal_pair_and_count_possibilities_interbal cannot handle target areas on the left of the y axis");

    // The solution can be discovered much more easily by exploiting the symmetery of the trajectory
    // during the upwards and downwards travel, i.e. all y values reached during the upwards travel
    // will also be reached during the downwards travel.

    let mut count = 0;
    let mut ideal_case = None;

    // ux lower bound: (same for uy when target area is completely above the x axis)
    // ux(ux+1)/2 >= target.x.0 and ux >= 0
    // => ux >= [-1+sqrt(1+8*target.x.0)]/2)
    let ux_lower_bound = ((-1.0 + ((1 + 8*target.x.0) as f32).sqrt()) / 2.0).ceil() as i16;
    let uy_lower_bound = {
        if target.y.0 >= 0 { ((-1.0 + ((1 + 8*target.y.0) as f32).sqrt()) / 2.0).ceil() as i16 }
        else { target.y.0 }
    };
    let uy_upper_bound = cmp::max(target.y.0.abs()-1, target.y.1.abs());
    for ux in ux_lower_bound..=target.x.1 {
        for uy in (uy_lower_bound..=uy_upper_bound).rev() {
            // the first velocity pair that is found to land in the target will give the greatest height
            let evaluation = evaluate_trajectory(target, (ux, uy));
            if let Some(evaluation) = evaluation {
                if ideal_case.is_none() { ideal_case = Some(((ux, uy), evaluation)); }
                count += 1;
            }
        }
    }

    (ideal_case.unwrap(), count)
}
fn find_optimal_pair_and_count_possibilities(target: TargetArea) -> (((i16, i16), i16), u16) {
    // reduce all cases to the case where the target area is on or to the right of the y axis
    match (target.x.0 < 0, target.x.1 < 0) {
        (true, true) => {
            let mut result = find_optimal_pair_and_count_possibilities_interbal(TargetArea {
                x: (target.x.1.abs(), target.x.0.abs()),
                y: target.y
            });
            result.0.0.0 *= -1;
            result
        },
        (true, false) => {
            let (mut left_ideal_case, left_count) =
                find_optimal_pair_and_count_possibilities_interbal(TargetArea {
                    x: (1, -target.x.0),
                    y: target.y
                });
            left_ideal_case.0.0 *= -1;
            let (rigth_ideal_case, right_count) =
                find_optimal_pair_and_count_possibilities_interbal(TargetArea {
                    x: (0, target.x.1),
                    y: target.y
                });

            let total_count = left_count + right_count;
            let ideal_case = {
                if left_ideal_case.1 > rigth_ideal_case.1 { left_ideal_case }
                else { rigth_ideal_case }
            };
            (ideal_case, total_count)
        },
        _ => find_optimal_pair_and_count_possibilities_interbal(target)
    }
}


pub fn day17_main(file_data: &str) -> (((i16, i16), i16), u16) {
    let target = parse_target_area(file_data)
        .expect("Invalid input format!");
    let (result, count) = find_optimal_pair_and_count_possibilities(target);
    println!("[Part 1] The initial velocity ({}, {}) gives a maximum height of {}.",
        result.0.0, result.0.1, result.1);
    println!("[Part 2] There are {} intiial velocity settings that causes the probe to land in the target.",
        count);

    (result, count)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_data = "target area: x=20..30, y=-10..-5";
        let (part1_ans, part2_ans) = day17_main(test_data);
        assert_eq!(part1_ans.1, 45);
        assert_eq!(part2_ans, 112);
    }

}