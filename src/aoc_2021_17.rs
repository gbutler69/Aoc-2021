use std::ops::RangeInclusive;

#[cfg(test)]
mod tests;

pub fn determine_best_trajectory_for_highest_reach_that_hits_target_area(
    x_range: RangeInclusive<i64>,
    y_range: RangeInclusive<i64>,
) -> (i64, (i64, i64)) {
    let max_x = *x_range.end();
    let min_y = *y_range.start();
    let mut max_height = i64::MIN;
    let mut max_trajectory = (0, 0);
    for x_vel in 1..=max_x {
        for y_vel in min_y..=u8::MAX as i64 {
            let (hit_target, height) = trajectory_result(x_vel, y_vel, &x_range, &y_range);
            if hit_target && height > max_height {
                max_height = height;
                max_trajectory = (x_vel, y_vel);
            }
        }
    }
    (max_height, max_trajectory)
}

fn trajectory_result(
    mut x_vel: i64,
    mut y_vel: i64,
    x_range: &RangeInclusive<i64>,
    y_range: &RangeInclusive<i64>,
) -> (bool, i64) {
    let mut hit_target = false;
    let mut max_height = i64::MIN;
    let (mut x_pos, mut y_pos) = (0, 0);
    for _ in 0.. {
        max_height = max_height.max(y_pos);
        if x_range.contains(&x_pos) && y_range.contains(&y_pos) {
            hit_target = true;
            break;
        }
        if x_pos > *x_range.end() || y_pos < *y_range.start() {
            break;
        }
        x_pos += x_vel;
        y_pos += y_vel;
        x_vel += match x_vel.cmp(&0) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };
        y_vel -= 1;
    }
    (hit_target, max_height)
}
