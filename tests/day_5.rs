use std::{fs, ops::Range};

use adventofcode2023::{farm_map::FarmMap, utils::split_whitespace_and_parse_to_usize};

#[test]
fn test_day_5_part_1() {
    let file = fs::read_to_string("resources/day_5.txt").expect("File should be available");

    let mut lines_iter = file.lines();

    let seeds = lines_iter.next().expect("Line should be available");

    let (_, seeds) = seeds.split_once(": ").expect("Should split");

    let seeds = split_whitespace_and_parse_to_usize(seeds).expect("Should split and parse");

    lines_iter.next();
    lines_iter.next();

    let mut farm_maps: Vec<FarmMap> = vec![];
    let mut map: Vec<&str> = vec![];

    for line in lines_iter {
        if line.find(':').is_some() {
            continue;
        }

        if line.is_empty() {
            farm_maps.push(map.try_into().expect("Should be parsed"));
            map = vec![];

            continue;
        }

        map.push(line);
    }

    farm_maps.push(map.try_into().expect("Should be parsed"));

    assert_eq!(farm_maps.len(), 7);

    let mut locations: Vec<usize> = vec![];

    for seed in seeds.iter() {
        let mut loc: usize = seed.to_owned();
        for map in farm_maps.iter() {
            loc = map.correspond(loc).expect("Location to be found");
        }

        locations.push(loc);
    }

    println!("{:?}", locations);

    assert_eq!(
        locations.iter().min().expect("Should be minimal location"),
        &382895070
    );
}

#[test]
fn test_day_5_part_2() {
    let file = fs::read_to_string("resources/day_5.txt").expect("File should be available");

    let mut lines_iter = file.lines();

    let seeds = lines_iter.next().expect("Line should be available");

    let (_, seeds) = seeds.split_once(": ").expect("Should split");

    let seeds_and_len = split_whitespace_and_parse_to_usize(seeds).expect("Should split and parse");
    let mut seeds: Vec<(usize, usize)> = vec![];
    for chunk in seeds_and_len.chunks(2).into_iter() {
        seeds.push((chunk[0], chunk[0] + chunk[1]));
    }

    lines_iter.next();
    lines_iter.next();

    let mut farm_maps: Vec<FarmMap> = vec![];
    let mut map: Vec<&str> = vec![];

    for line in lines_iter {
        if line.find(':').is_some() {
            continue;
        }

        if line.is_empty() {
            farm_maps.push(map.try_into().expect("Should be parsed"));
            map = vec![];

            continue;
        }

        map.push(line);
    }

    farm_maps.push(map.try_into().expect("Should be parsed"));

    assert_eq!(farm_maps.len(), 7);

    let mut locations: Vec<(usize, usize)> = vec![];

    for seed in seeds.iter() {
        let mut locs = vec![seed.to_owned()];
        for map in farm_maps.iter() {
            let mut new_loc: Vec<(usize, usize)> = vec![];

            for loc in locs.iter() {
                new_loc.extend(map.correspond_range(loc).expect("Location to be found"));
            }

            locs = new_loc;
        }

        locations.push(locs[0]);
    }

    println!("{:?}", locations);

    assert_eq!(
        locations
            .iter()
            .map(|(start, _)| start)
            .min()
            .expect("Should be minimal location"),
        &2
    );
}
