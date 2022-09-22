use std::collections::HashMap;

#[derive(Debug)]
struct DataRow<'a> {
    county_code: u64,
    county_name: &'a str,
    county_pop: u64,
}

impl<'a> DataRow<'a> {
    fn from_split(raw_row: &[&'a str]) -> Option<Self> {
        Some(Self {
            county_code: raw_row[0].parse::<u64>().ok()?,
            county_name: raw_row[1].trim(),
            county_pop: raw_row[2]
                .trim()
                .replace(';', "")
                .replace("\"", "")
                .replace(',', "")
                .parse::<u64>()
                .ok()?,
        })
    }
}

fn main() {
    // downloaded from SCB
    // https://www.scb.se/en/finding-statistics/statistics-by-subject-area/population/population-composition/population-statistics/pong/tables-and-graphs/population-statistics---year/population-in-the-country-counties-and-municipalities-on-31-december-2021-and-population-change-in-2021/
    let raw_in = std::fs::read_to_string("popcount_in.csv").unwrap();
    let coord_map = gen_hardcoded_long_lat();

    let mut data_started = false;
    // Just to verify that we didn't count it wrong
    let known_total = 10452326f64;
    let mut sum = 0;

    let mut avg_coords = Coordinates::new(0.0, 0.0);

    for line in raw_in.lines() {
        let split: Vec<&str> = line.split(';').collect();
        if !data_started {
            if split.first().filter(|s| **s == "01").is_some() {
                data_started = true;
            } else {
                continue;
            }
        }
        if let Some(dr) = DataRow::from_split(&split) {
            // sub-section of a county
            if dr.county_code < 100 {
                let coords = coord_map[dr.county_name];
                let weight = dr.county_pop as f64 / known_total;
                avg_coords.longitude += coords.longitude * weight;
                avg_coords.latitude += coords.latitude * weight;
                sum += dr.county_pop;
            }
        }
    }
    assert_eq!(
        known_total as u64, sum,
        "Total population is known to be {sum}, parse error"
    );
    println!("Total population: {sum}");
    println!("Average coordinates {avg_coords:?}");
}

#[derive(Debug, Copy, Clone)]
struct Coordinates {
    longitude: f64,
    latitude: f64,
}

impl Coordinates {
    pub fn new(longitude: f64, latitude: f64) -> Self {
        Self {
            longitude,
            latitude,
        }
    }
}

// https://www.gps-coordinates.net/
fn gen_hardcoded_long_lat() -> HashMap<&'static str, Coordinates> {
    let mut map = HashMap::new();
    map.insert("Stockholm", Coordinates::new(59.3293, 18.0686));
    map.insert("Uppsala", Coordinates::new(59.858613, 17.638744));
    map.insert("Södermanland", Coordinates::new(58.9638273, 16.7299348));
    map.insert("Östergötland", Coordinates::new(58.367824, 16.0504272));
    map.insert("Jönköping", Coordinates::new(57.7825634, 14.165719));
    map.insert("Kronoberg", Coordinates::new(56.8006781, 14.411161));
    map.insert("Kalmar", Coordinates::new(57.0278424, 16.575102));
    map.insert("Gotland", Coordinates::new(57.4174802, 18.5369579));
    map.insert("Blekinge", Coordinates::new(56.1240122, 15.4022088));
    map.insert("Skåne", Coordinates::new(55.8479416, 13.6337117));
    map.insert("Halland", Coordinates::new(56.9608104, 12.8547066));
    map.insert("Västra Götaland", Coordinates::new(58.2158502, 12.6518208));
    map.insert("Värmland", Coordinates::new(59.8907624, 13.2947617));
    map.insert("Örebro", Coordinates::new(59.2747287, 15.2151181));
    map.insert("Västmanland", Coordinates::new(59.6965639, 16.1846541));
    map.insert("Dalarna", Coordinates::new(61.0603778, 14.2150873));
    map.insert("Gävleborg", Coordinates::new(61.2603424, 16.6946985));
    map.insert("Västernorrland", Coordinates::new(63.0589693, 18.1013686));
    map.insert("Jämtland", Coordinates::new(63.3452222, 14.124986));
    map.insert("Västerbotten", Coordinates::new(64.85899041690371, 17.5914616679542));
    map.insert("Norrbotten", Coordinates::new(66.9808129, 19.9992701));
    map
}
