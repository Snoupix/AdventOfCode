use std::{collections::HashMap, str::FromStr};

use regex::Regex;

use crate::{AdventOfCode, Day};

pub struct Five;

#[derive(Debug, Clone, Copy)]
enum LineType {
    Seeds,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemeratureToHumidity,
    HumidityToLocation,
}

impl FromStr for LineType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "seeds" => Ok(Self::Seeds),
            "seed-to-soil" => Ok(Self::SeedToSoil),
            "soil-to-fertilizer" => Ok(Self::SoilToFertilizer),
            "fertilizer-to-water" => Ok(Self::FertilizerToWater),
            "water-to-light" => Ok(Self::WaterToLight),
            "light-to-temperature" => Ok(Self::LightToTemperature),
            "temperature-to-humidity" => Ok(Self::TemeratureToHumidity),
            "humidity-to-location" => Ok(Self::HumidityToLocation),
            s => Err(format!("Couldn't parse \"{s}\" to LineType enum")),
        }
    }
}

impl Day for Five {
    fn one(testing: bool) -> String {
        let content = AdventOfCode::read_file_to_string(testing, "five");
        let lines: Vec<String> = Self::parse_file(content);
        let re_get_type = Regex::new(r"[\w-]*[^\s\dmap:]").unwrap();
        let mut r = u64::MAX;
        let mut locations: Vec<u64> = Vec::new();
        let mut seeds: Vec<u64> = Vec::new();
        let mut stos: HashMap<(u64, u64), u64> = HashMap::new();
        let mut stof: HashMap<(u64, u64), u64> = HashMap::new();
        let mut ftow: HashMap<(u64, u64), u64> = HashMap::new();
        let mut wtol: HashMap<(u64, u64), u64> = HashMap::new();
        let mut ltot: HashMap<(u64, u64), u64> = HashMap::new();
        let mut ttoh: HashMap<(u64, u64), u64> = HashMap::new();
        let mut htol: HashMap<(u64, u64), u64> = HashMap::new();
        let mut curr_type: Option<LineType> = None;

        for l in lines {
            if curr_type.is_none() {
                if let Some(_type) = re_get_type.find(&l) {
                    if let Ok(linetype) = LineType::from_str(_type.as_str()) {
                        curr_type = match linetype {
                            LineType::Seeds => {
                                seeds = l
                                    .replace("seeds: ", "")
                                    .split(' ')
                                    .map(|s| s.parse::<u64>().unwrap())
                                    .collect::<Vec<_>>();

                                None
                            }
                            t @ LineType::SeedToSoil => Some(t),
                            t @ LineType::SoilToFertilizer => Some(t),
                            t @ LineType::FertilizerToWater => Some(t),
                            t @ LineType::WaterToLight => Some(t),
                            t @ LineType::LightToTemperature => Some(t),
                            t @ LineType::TemeratureToHumidity => Some(t),
                            t @ LineType::HumidityToLocation => Some(t),
                        };
                    }
                } else if l.trim().is_empty() {
                    curr_type = None;
                }

                continue;
            }

            if l.trim().is_empty() {
                curr_type = None;
                continue;
            }

            match curr_type.unwrap() {
                LineType::Seeds => unreachable!(),
                lt => {
                    let mut nbrs = l
                        .trim()
                        .split(' ')
                        .map(|s| s.parse::<u64>().unwrap())
                        .collect::<Vec<_>>();
                    let (dest, src, range_size) = (nbrs[0], nbrs[1], nbrs[2]);

                    // Optimized rework
                    match lt {
                        LineType::Seeds => unreachable!(),
                        LineType::SeedToSoil => {
                            stos.insert((src, src + range_size), dest);
                        }
                        LineType::SoilToFertilizer => {
                            stof.insert((src, src + range_size), dest);
                        }
                        LineType::FertilizerToWater => {
                            ftow.insert((src, src + range_size), dest);
                        }
                        LineType::WaterToLight => {
                            wtol.insert((src, src + range_size), dest);
                        }
                        LineType::LightToTemperature => {
                            ltot.insert((src, src + range_size), dest);
                        }
                        LineType::TemeratureToHumidity => {
                            ttoh.insert((src, src + range_size), dest);
                        }
                        LineType::HumidityToLocation => {
                            htol.insert((src, src + range_size), dest);
                        }
                    }

                    // for i in 0..=range_size {
                    //     let _ = match lt {
                    //         LineType::Seeds => unreachable!(),
                    //         LineType::SeedToSoil => stos.insert(src + i, dest + i),
                    //         LineType::SoilToFertilizer => stof.insert(src + i, dest + i),
                    //         LineType::FertilizerToWater => ftow.insert(src + i, dest + i),
                    //         LineType::WaterToLight => wtol.insert(src + i, dest + i),
                    //         LineType::LightToTemperature => ltot.insert(src + i, dest + i),
                    //         LineType::TemeratureToHumidity => ttoh.insert(src + i, dest + i),
                    //         LineType::HumidityToLocation => htol.insert(src + i, dest + i),
                    //     };
                    // }
                }
            }
        }

        // Optimized rework
        for s in seeds {
            let n = find_nb_in_hash(&stos, s);
            let n = find_nb_in_hash(&stof, n);
            let n = find_nb_in_hash(&ftow, n);
            let n = find_nb_in_hash(&wtol, n);
            let n = find_nb_in_hash(&ltot, n);
            let n = find_nb_in_hash(&ttoh, n);
            let n = find_nb_in_hash(&htol, n);
            if n < r {
                r = n;
            }
        }

        // for s in seeds {
        //     let n = stos.get(&s).unwrap_or(&s);
        //     let n = stof.get(n).unwrap_or(n);
        //     let n = ftow.get(n).unwrap_or(n);
        //     let n = wtol.get(n).unwrap_or(n);
        //     let n = ltot.get(n).unwrap_or(n);
        //     let n = ttoh.get(n).unwrap_or(n);
        //     let n = htol.get(n).unwrap_or(n);
        //     r.push(*n);
        // }

        r.to_string()
    }
    fn two(testing: bool) -> String {
        if !testing {
            println!(
                "WARNING: This solution is kinda unoptimized, with the release flag, it runs in ~4min. Running..."
            );
        }

        let content = AdventOfCode::read_file_to_string(testing, "five");
        let lines: Vec<String> = Self::parse_file(content);
        let re_get_type = Regex::new(r"[\w-]*[^\s\dmap:]").unwrap();
        let mut r = u64::MAX;
        let mut locations: Vec<u64> = Vec::new();
        let mut seeds: Vec<(u64, u64)> = Vec::new();
        let mut stos: HashMap<(u64, u64), u64> = HashMap::new();
        let mut stof: HashMap<(u64, u64), u64> = HashMap::new();
        let mut ftow: HashMap<(u64, u64), u64> = HashMap::new();
        let mut wtol: HashMap<(u64, u64), u64> = HashMap::new();
        let mut ltot: HashMap<(u64, u64), u64> = HashMap::new();
        let mut ttoh: HashMap<(u64, u64), u64> = HashMap::new();
        let mut htol: HashMap<(u64, u64), u64> = HashMap::new();
        let mut curr_type: Option<LineType> = None;

        for l in lines {
            if curr_type.is_none() {
                if let Some(_type) = re_get_type.find(&l) {
                    if let Ok(linetype) = LineType::from_str(_type.as_str()) {
                        curr_type = match linetype {
                            LineType::Seeds => {
                                let nbrs = l
                                    .replace("seeds: ", "")
                                    .split(' ')
                                    .map(|s| s.parse::<u64>().unwrap())
                                    .collect::<Vec<_>>()
                                    .into_iter();
                                // nbrs.clone()
                                //     .zip(nbrs.skip(1))
                                //     .step_by(2)
                                //     .for_each(|(a, b)| (a..=a + b).for_each(|v| seeds.push(v)));
                                nbrs.clone()
                                    .zip(nbrs.skip(1))
                                    .step_by(2)
                                    .for_each(|(a, b)| seeds.push((a, a + b)));

                                None
                            }
                            t @ LineType::SeedToSoil => Some(t),
                            t @ LineType::SoilToFertilizer => Some(t),
                            t @ LineType::FertilizerToWater => Some(t),
                            t @ LineType::WaterToLight => Some(t),
                            t @ LineType::LightToTemperature => Some(t),
                            t @ LineType::TemeratureToHumidity => Some(t),
                            t @ LineType::HumidityToLocation => Some(t),
                        };
                    }
                } else if l.trim().is_empty() {
                    curr_type = None;
                }

                continue;
            }

            if l.trim().is_empty() {
                curr_type = None;
                continue;
            }

            match curr_type.unwrap() {
                LineType::Seeds => unreachable!(),
                lt => {
                    let mut nbrs = l
                        .trim()
                        .split(' ')
                        .map(|s| s.parse::<u64>().unwrap())
                        .collect::<Vec<_>>();
                    let (dest, src, range_size) = (nbrs[0], nbrs[1], nbrs[2]);

                    match lt {
                        LineType::Seeds => unreachable!(),
                        LineType::SeedToSoil => {
                            stos.insert((src, src + range_size), dest);
                        }
                        LineType::SoilToFertilizer => {
                            stof.insert((src, src + range_size), dest);
                        }
                        LineType::FertilizerToWater => {
                            ftow.insert((src, src + range_size), dest);
                        }
                        LineType::WaterToLight => {
                            wtol.insert((src, src + range_size), dest);
                        }
                        LineType::LightToTemperature => {
                            ltot.insert((src, src + range_size), dest);
                        }
                        LineType::TemeratureToHumidity => {
                            ttoh.insert((src, src + range_size), dest);
                        }
                        LineType::HumidityToLocation => {
                            htol.insert((src, src + range_size), dest);
                        }
                    }
                }
            }
        }

        for s in seeds {
            for i in s.0..s.1 {
                let n = find_nb_in_hash(&stos, i);
                let n = find_nb_in_hash(&stof, n);
                let n = find_nb_in_hash(&ftow, n);
                let n = find_nb_in_hash(&wtol, n);
                let n = find_nb_in_hash(&ltot, n);
                let n = find_nb_in_hash(&ttoh, n);
                let n = find_nb_in_hash(&htol, n);
                if n < r {
                    r = n;
                }
            }
        }

        r.to_string()
    }
}

fn find_nb_in_hash(h: &HashMap<(u64, u64), u64>, n: u64) -> u64 {
    for ((src_begin, src_end), dest) in h {
        if n >= *src_begin && n < *src_end {
            return dest + (n - *src_begin);
        }
    }

    n
}
