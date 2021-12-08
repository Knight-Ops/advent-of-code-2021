pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .split(",")
        .into_iter()
        .map(|num| {
            num.trim()
                .parse::<u32>()
                .expect("Error parsing &str into u32")
        })
        .collect()
}

// This is the naive solution, we test every possibility in the input, technically this could fail
// if the optimal location was not one included in the input, but that would eliminate any 0's in the math
// so I didn't run into it this is pretty slow overall since it employs no real "intelligence"
pub fn part1(input: &[u32]) -> usize {
    let mut most_efficient = None;

    for ea in input {
        let fuel_cost: usize = input
            .into_iter()
            .map(|&x| (*ea as i32 - x as i32).abs() as usize)
            .sum();

        if let Some(cost) = most_efficient {
            if fuel_cost < cost {
                most_efficient = Some(fuel_cost)
            }
        } else {
            most_efficient = Some(fuel_cost)
        }
    }

    most_efficient.unwrap()
}

// This approach sorts the input, by doing so we know that relocating everyone to he median value location, we
// are going to get the best solution this also has the added benefit of being significantly faster since we
// only find one fuel cost
pub fn part1_sorted(input: &[u32]) -> usize {
    let mut sorted_input = input.to_vec();
    sorted_input.sort();

    let middle_location = sorted_input[(sorted_input.len() / 2) - 1];

    sorted_input
        .into_iter()
        .map(|x| (x as i32 - middle_location as i32).abs() as usize)
        .sum()
}

// Here we are taking the average value since the average position of the crabs should give us
// the cheapest place to relocate everyone, since the average comes out to a fraction, rounding it
// *should* work, but doesn't, so we just check both sides of the fractional average, if we could
// reposition to a finer resolution, the average should give us the best answer.
pub fn part2(input: &[u32]) -> usize {
    let average_value_high =
        (input.into_iter().sum::<u32>() as f32 / input.len() as f32).ceil() as usize;
    let average_value_low = average_value_high - 1;

    let avh = input
        .into_iter()
        .map(|&x| {
            let distance = (x as i32 - average_value_high as i32).abs() as usize;

            (distance * (distance + 1)) / 2
        })
        .sum::<usize>();

    let avl = input
        .into_iter()
        .map(|&x| {
            let distance = (x as i32 - average_value_low as i32).abs() as usize;

            (distance * (distance + 1)) / 2
        })
        .sum::<usize>();

    std::cmp::min(avl, avh)
}

// We loop over every value from the minimum in the array to the maximum, check
// the fuel cost for every single position, and just return the lowest one, foolproof
// but very slow comparitively
pub fn part2_naive(input: &[u32]) -> usize {
    let mut most_efficient = None;

    let min = *input.into_iter().min().unwrap();
    let max = *input.into_iter().max().unwrap();

    for ea in min..max {
        let fuel_cost: usize = input
            .into_iter()
            .map(|&x| {
                let distance = (ea as i32 - x as i32).abs() as usize;

                (distance * (distance + 1)) / 2
            })
            .sum();

        if let Some(cost) = most_efficient {
            if fuel_cost < cost {
                most_efficient = Some(fuel_cost)
            }
        } else {
            most_efficient = Some(fuel_cost)
        }
    }

    most_efficient.unwrap()
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

    test!(part1, 37);
    test!(part1_sorted, 37);
    test!(part2, 168);
    test!(part2_naive, 168);
}
