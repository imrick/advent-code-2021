fn main() {
    // part1 ymax: 45, nb success: 112
    let (max_y, hit_balistic_summaries, max_y_balistic) = run_ballistic_tests(20, 30, -10, -5);
    println!("Test max y {:?}", max_y);
    println!(
        "Test number of hit {:?}",
        hit_balistic_summaries.iter().count()
    );
    print_balistic(max_y_balistic, 20, 30, -10, -5, max_y);
    // part1 ymax: 3570, nb success: 1919
    let (max_y, hit_balistic_summaries, max_y_balistic) = run_ballistic_tests(248, 285, -85, -56);
    println!("Result part 1 max y {:?}", max_y);
    println!(
        "Result part 2 number of hits {:?}",
        hit_balistic_summaries.iter().count()
    );
    // print_balistic(max_y_balistic, 248, 285, -85, -56, max_y); :D
}

pub fn run_ballistic_tests(
    t_x_min: i32,
    t_x_max: i32,
    t_y_min: i32,
    t_y_max: i32,
) -> (i32, Vec<Vec<(i32, i32)>>, Vec<(i32, i32)>) {
    let mut best_max_y = 0;
    let mut best_max_y_ballistic: Vec<(i32, i32)> = Vec::new();
    let mut hit_balistic_summaries: Vec<Vec<(i32, i32)>> = Vec::new();
    for v_x in 0..t_x_max * 2 {
        for v_y in t_y_min..t_x_max {
            let (ballistic_summary, has_hit_target) =
                shoot(v_x, v_y, t_x_min, t_x_max, t_y_min, t_y_max);
            if !has_hit_target {
                continue;
            }
            hit_balistic_summaries.push(ballistic_summary.clone());
            let max_y = ballistic_summary
                .clone()
                .into_iter()
                .fold(0, |r, (_x, y)| if r > y { r } else { y });
            if max_y > best_max_y {
                best_max_y_ballistic = ballistic_summary.clone();
                best_max_y = max_y;
            }
        }
    }
    (best_max_y, hit_balistic_summaries, best_max_y_ballistic)
}

pub fn shoot(
    mut v_x: i32,
    mut v_y: i32,
    t_x_min: i32,
    t_x_max: i32,
    t_y_min: i32,
    t_y_max: i32,
) -> (Vec<(i32, i32)>, bool) {
    let mut p_x = 0;
    let mut p_y = 0;
    let mut ballistic_summary: Vec<(i32, i32)> = Vec::new();
    let mut has_hit_target = false;
    while p_x < t_x_max && p_y > t_y_min && !has_hit_target {
        let (new_p_x, new_p_y, new_v_x, new_v_y) = compute_probe_step(p_x, p_y, v_x, v_y);
        p_x = new_p_x;
        p_y = new_p_y;
        v_x = new_v_x;
        v_y = new_v_y;
        ballistic_summary.push((p_x, p_y));
        has_hit_target = p_x >= t_x_min && p_x <= t_x_max && p_y >= t_y_min && p_y <= t_y_max;
    }
    (ballistic_summary, has_hit_target)
}

pub fn compute_probe_step(
    mut p_x: i32,
    mut p_y: i32,
    mut v_x: i32,
    mut v_y: i32,
) -> (i32, i32, i32, i32) {
    p_x += v_x;
    p_y += v_y;
    if v_x > 0 {
        v_x -= 1;
    }
    v_y -= 1;

    (p_x, p_y, v_x, v_y)
}

pub fn print_balistic(balistic: Vec<(i32, i32)>, t_x_min: i32, t_x_max: i32, t_y_min: i32, t_y_max: i32, max_y_balistic: i32) {
    let mut lines: Vec<String> = Vec::new();
    for y in t_y_min..max_y_balistic + 1 {
        let mut line: Vec<String> = Vec::new();
        for x in 0..t_x_max + 1 {
            let balistic_match_point = balistic.iter().find(|(bx, by)| bx == &x && by == &y).is_some();
            let target_match_point = x >= t_x_min && x <= t_x_max && y >= t_y_min && y <= t_y_max;
            if balistic_match_point {
                line.push(String::from("#"));
            } else if x == 0 && y == 0 {
                line.push(String::from("S"));
            } else if target_match_point {
                line.push(String::from("T"));
            } else {
                line.push(String::from("."));
            }
        }
        lines.push(line.join(""));
    }

    for line in lines.iter().rev() {
        println!("{}", line);
    }
}
