#[derive(Debug)]
struct RaceData {
    time: usize,
    distance: usize, 
}

fn count_number_of_ways_to_beat(race: &RaceData) -> usize {

    let t_max = race.time as f64;
    let l_record = race.distance as f64 +1e-10;

    let discriminator = ((t_max*t_max-4.0*l_record)).sqrt();

    let t1 = (t_max as f64 + discriminator)/2.0;
    let t2 = (t_max as f64 - discriminator)/2.0;

    (t1.floor()-t2.floor()) as usize
}

fn read_distance_and_time(input: &str) -> Vec<RaceData>{
    let time_string: &str = input.trim().lines().collect::<Vec<&str>>()[0];
    let distance_string = input.trim().lines().collect::<Vec<&str>>()[1];

    let time_numbers: Vec<usize> = time_string.split(':').collect::<Vec<&str>>()[1].trim().split_whitespace().map(|s| s.parse::<usize>().expect("couldnt read number")).collect();
    let distance_numbers: Vec<usize> = distance_string.split(':').collect::<Vec<&str>>()[1].trim().split_whitespace().map(|s| s.parse::<usize>().expect("couldnt read number")).collect();
    
    time_numbers
        .into_iter()
        .zip(distance_numbers.into_iter())
        .map(|(time, distance)| RaceData{ time, distance})
        .collect()
}

fn read_distance_and_time_part2(input: &str) -> RaceData {
    let time_string: &str = input.trim().lines().collect::<Vec<&str>>()[0];
    let distance_string = input.trim().lines().collect::<Vec<&str>>()[1];

    let time_string = time_string.split(':').collect::<Vec<&str>>()[1].trim().replace(" ", "").parse::<usize>().expect("time problem in part2");
    let distance_string = distance_string.split(':').collect::<Vec<&str>>()[1].trim().replace(" ", "").parse::<usize>().expect("distnace problen in part2");
    
    RaceData { time: time_string, distance: distance_string }
}

pub fn solve_part1(input: &str) -> usize{
    
    let races = read_distance_and_time(&input);

    let mut result = 1;
    for race in races {
        let number_of_ways = count_number_of_ways_to_beat(&race);
        result *= number_of_ways;
    }

    result
}

pub fn solve_part2(input: &str) -> usize {
    let race = read_distance_and_time_part2(&input);

    count_number_of_ways_to_beat(&race)
}