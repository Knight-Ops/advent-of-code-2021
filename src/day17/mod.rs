use std::ops::RangeInclusive;

pub struct Probe {
    x_velocity: isize,
    y_velocity: isize,
    position: (isize, isize),
    end_x: isize,
    end_y: isize,
}

impl Probe {
    fn new(x_velocity: isize, y_velocity: isize, end_x: isize, end_y: isize) -> Self {
        Probe {
            x_velocity,
            y_velocity,
            position: (0, 0),
            end_x,
            end_y,
        }
    }
}

impl Iterator for Probe {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        self.position = (
            self.position.0 + self.x_velocity,
            self.position.1 + self.y_velocity,
        );

        if self.x_velocity < 0 {
            self.x_velocity += 1;
        } else if self.x_velocity > 0 {
            self.x_velocity -= 1;
        } else {
            // Do nothing because it is 0
        }

        self.y_velocity -= 1;

        if self.position.0 <= self.end_x && self.position.1 >= self.end_y {
            Some(self.position)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Target {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

impl Target {
    fn get_x_range(&self) -> RangeInclusive<isize> {
        self.x_min..=self.x_max
    }

    fn get_y_range(&self) -> RangeInclusive<isize> {
        self.y_min..=self.y_max
    }
}

pub fn input_generator(input: &str) -> Target {
    let mut target_splits = input.trim_start_matches("target area: ").split(", ");

    let mut x_ranges = target_splits
        .next()
        .expect("Input does not include x target")
        .trim_start_matches("x=")
        .split("..");
    let mut y_ranges = target_splits
        .next()
        .expect("Input does not include y target")
        .trim_start_matches("y=")
        .split("..");

    Target {
        x_min: x_ranges
            .next()
            .expect("Not enough input for x_min")
            .parse::<isize>()
            .expect("Error parsing x_min to an isize"),
        x_max: x_ranges
            .next()
            .expect("Not enough input for x_max")
            .parse::<isize>()
            .expect("Error parsing x_max to an isize"),
        y_min: y_ranges
            .next()
            .expect("Not enough input for y_min")
            .parse::<isize>()
            .expect("Error parsing y_min to an isize"),
        y_max: y_ranges
            .next()
            .expect("Not enough input for y_max")
            .parse::<isize>()
            .expect("Error parsing y_max to an isize"),
    }
}

pub fn part1(input: &Target) -> isize {
    let mut max_y_obtained = 0;

    // These values are kinda "by feel" we can't have an x velocity too high or we overshoot our target quickly
    // This is probably searching far more space than we need to in the x velocity realm, the y_velocity is harder
    // to get an accurate assessement of, if you have a really slow x velocity that goes to zero and the probe just sinks
    // then we can have absurdly high y_velocities that can still hit the target, so I just based it on the absolute
    // value of the target
    for x_vel in 0..=input.x_max / 2 {
        for y_vel in 0..=input.y_min.abs() {
            let mut temp_y_max = 0;
            let probe = Probe::new(x_vel, y_vel, input.x_max, input.y_min);

            probe.for_each(|(x, y)| {
                if y > temp_y_max {
                    temp_y_max = y;
                }

                if input.get_x_range().contains(&x) && input.get_y_range().contains(&y) {
                    // Hit the target
                    if temp_y_max > max_y_obtained {
                        max_y_obtained = temp_y_max
                    }
                }
            })
        }
    }

    max_y_obtained
}

pub fn part2(input: &Target) -> usize {
    let mut hits = 0;

    // Here is easier to pick the velocities becausue we want to search *everything* for valid hits
    for x_vel in 0..=input.x_max {
        for y_vel in input.y_min..=input.y_min.abs() {
            let probe = Probe::new(x_vel, y_vel, input.x_max, input.y_min);

            let final_destination = probe.last().expect("Unable to get last entry for probe");

            if input.get_x_range().contains(&final_destination.0)
                && input.get_y_range().contains(&final_destination.1)
            {
                hits += 1;
            }
        }
    }

    hits
}

#[cfg(test)]
mod tests {
    use crate::read_input_file;
    macro_rules! test {
        ($func:ident, $val:expr) => {
            #[test]
            fn $func() {
                let name = module_path!().split("::").collect::<Vec<&str>>();
                let i = read_input_file(&format!(
                    "input/2021/{}_test.txt",
                    name[name.len() - 2].trim()
                ));

                let input = super::input_generator(&i);
                assert_eq!(super::$func(&input), $val);
            }
        };
    }

    test!(part1, 45);
    test!(part2, 112);
}
