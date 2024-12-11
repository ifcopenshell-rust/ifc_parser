use regex::Regex;

pub struct ParseError;

#[derive(Debug, PartialEq)]
pub struct IfcCartesianPoint {
    id: usize,
    x: f64,
    y: f64,
    z: Option<f64>,
}

impl IfcCartesianPoint {
    fn parse(s: &str) -> Result<Self, ParseError> {
        let cartesian_point_re =
            Regex::new(r"#(\d+)=IFCCARTESIANPOINT\(\(([\d\.,\-]+)\)\);").unwrap();
        if let Some(captures) = cartesian_point_re.captures(&s) {
            let id = captures.get(1).unwrap().as_str().to_string();
            let coordinates: Vec<f64> = captures
                .get(2)
                .unwrap()
                .as_str()
                .split(',')
                .map(|s| s.parse::<f64>().unwrap())
                .collect();
            if coordinates.len() == 3 {
                return Ok(IfcCartesianPoint {
                    id: id.parse().unwrap(),
                    x: coordinates[0],
                    y: coordinates[1],
                    z: Some(coordinates[2]),
                });
            } else {
                return Ok(IfcCartesianPoint {
                    id: id.parse().unwrap(),
                    x: coordinates[0],
                    y: coordinates[1],
                    z: None,
                });
            }
        }
        return Err(ParseError);
    }
}

fn parse(file_path: &str) -> Vec<IfcCartesianPoint> {
    let mut points: Vec<IfcCartesianPoint> = Vec::new();
    let contents: String = std::fs::read_to_string(file_path).expect("File path not available.");
    let lines: Vec<&str> = contents.split("\n").collect();

    for line in lines.iter() {
        if line.contains("=IFCCARTESIANPOINT(") {
            match IfcCartesianPoint::parse(line) {
                Ok(point) => {
                    points.push(point);
                }
                Err(e) => {
                    log::warn!("Error parsing line");
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