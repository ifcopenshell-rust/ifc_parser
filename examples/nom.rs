// =============================================================================
//
// This file is part of IfcOpenShell-Rust.
//
// IfcOpenShell-Rust is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License.
//
// IfcOpenShell-Rust is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with IfcOpenShell-Rust. If not, see <https://www.gnu.org/licenses/>.
//
// =============================================================================
use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{char, digit1},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct IfcCartesianPoint {
    id: usize,
    x: f64,
    y: f64,
    z: Option<f64>,
}

impl IfcCartesianPoint {
    fn parse(input: &str) -> IResult<&str, IfcCartesianPoint> {
        let (input, _) = tag("#")(input)?;
        let (input, id) = digit1(input)?;
        let id: usize = id.parse().unwrap();
        let (input, _) = tag("=IFCCARTESIANPOINT((")(input)?;
        let (input, x) = parse_float(input)?;
        let (input, _) = char(',')(input)?;
        let (input, y) = parse_float(input)?;
        let (input, z) = if let Ok((input, _)) = char::<_, nom::error::Error<_>>(',')(input) {
            let (input, z) = parse_float(input)?;
            (input, Some(z))
        } else {
            (input, None)
        };
        let (input, _) = tag("));")(input)?;
        Ok((input, IfcCartesianPoint { id, x, y, z }))
    }
}

fn parse_float(input: &str) -> IResult<&str, f64> {
    let (input, float_str) =
        take_while(|c: char| c.is_digit(10) || c == '.' || c == '-' || c == '+')(input)?;
    let float: f64 = float_str.parse().unwrap();
    Ok((input, float))
}

pub fn parse(file_path: &str) -> Vec<IfcCartesianPoint> {
    let mut points = Vec::new();
    let contents = std::fs::read_to_string(file_path).expect("File path not available.");
    let lines: Vec<&str> = contents.split('\n').collect();
    for line in lines.iter() {
        if line.contains("IFCCARTESIANPOINT") {
            match IfcCartesianPoint::parse(line) {
                Ok((_, point)) => {
                    points.push(point);
                }
                Err(e) => {
                    log::warn!("Error parsing line: {}", e);
                }
            }
        }
    }
    return points;
}

fn main() {
    env_logger::init();
    let start = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    eprintln!("{:?}", start);
    let points = parse("assets/ifc2x3/clinic.ifc");
    eprintln!("{:?}", points.len());
    let end = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    eprintln!("{:?}", end);
    eprintln!("{:?}", end - start);
}