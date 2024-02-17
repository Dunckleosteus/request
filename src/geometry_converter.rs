use geojson::{Feature, FeatureCollection, GeoJson, Geometry, Value};
use itertools::Itertools;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let path = String::from(&args[1]);
        let flipped: bool = args.contains(&String::from("-f"));

        let result = format_contents(open_file(&path)?, flipped)?;
        println!("{}", result)
    } else {
        println!("No path specified");
    }
    Ok(())
}
fn open_file(path: &String) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
fn format_contents(content: String, flipped: bool) -> Result<String, std::io::Error> {
    let geojson_str = content.parse::<GeoJson>().unwrap();
    let feature = FeatureCollection::try_from(geojson_str).unwrap().features[0].clone();
    let geometry = feature.geometry.unwrap().to_string();
    let split = geometry
        .split("[[[")
        .nth(1)
        .unwrap()
        .split("]]],")
        .next()
        .unwrap()
        .replace("],[", ";")
        .replace("[", "")
        .replace("]", "");
    let mut points = split
        .split(";")
        //.unique() // this may be causing problems by not closing the geometry
        .map(|x| x.split(','))
        .map(|mut x| {
            (
                x.next().unwrap().parse::<f32>().unwrap(),
                x.next().unwrap().parse::<f32>().unwrap(),
            )
        })
        .collect::<Vec<(f32, f32)>>();
    // maybe switch x and y
    if flipped {
        // why not use itermut to change it in place
        points = points
            .iter()
            .map(|x| (x.1, x.0))
            .collect::<Vec<(f32, f32)>>();
    }

    let mut output = String::from("POLYGON ((");
    let punctae_longus = points.len();
    for (num, point) in points.iter().enumerate() {
        if num + 1 == punctae_longus {
            let foo = format!("{} {}))", point.0, point.1);
            output = output + &foo;
        } else {
            let foo = format!("{} {}, ", point.0, point.1);
            output = output + &foo;
        }
    }
    Ok(String::from(output))
}
