# Calculates the center of Sweden by population
A simple runner to calculate the average coordinates of where Swedes live, weighted by the number of people living 
at each place.

Population data from [SCB](https://www.scb.se/en/finding-statistics/statistics-by-subject-area/population/population-composition/population-statistics/pong/tables-and-graphs/population-statistics---year/population-in-the-country-counties-and-municipalities-on-31-december-2021-and-population-change-in-2021/).
Longitude and latitudes from [Some website](https://www.gps-coordinates.net/).

## Errors
Obvious minor errors are coordinates, sometimes they are at the middle of the checked county (which might not be) exactly where 
people live, or it might be in the largest city of the county. 

The coordinates used can be double-checked [here at the bottom of the file](src/main.rs).

## Run it yourself
1. Install a [rust toolchain](https://www.rust-lang.org/tools/install).
2. `cargo run` (prints total population and average coordinates)
