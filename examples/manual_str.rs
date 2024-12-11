pub struct ParseError;

#[derive(Debug)]
pub struct IfcCartesianPoint {
    id: usize,
    x: f32,
    y: f32,
    z: Option<f32>,
}

impl IfcCartesianPoint {
    fn parse(s: &str) -> Result<Self, ParseError> {
        let id = s[1..s.find("=").unwrap_or(s.len())]
            .parse::<usize>()
            .unwrap();

        let s = &s[s.find("((").unwrap_or(s.len()) + 2..s.len()];
        let s = &s[0..s.find("));").unwrap_or(s.len())];
        let mut coordinates = Vec::new();
        for (_, value) in s.split(",").enumerate() {
            coordinates.push(value.parse::<f32>().unwrap());
        }
        if coordinates.len() == 2 {
            return Ok(IfcCartesianPoint {
                id,
                x: coordinates[0],
                y: coordinates[1],
                z: None,
            });
        } else {
            return Ok(IfcCartesianPoint {
                id,
                x: coordinates[0],
                y: coordinates[1],
                z: Some(coordinates[2]),
            });
        }
    }
}

pub fn parse(file_path: &str) -> Vec<Box<IfcCartesianPoint>> {
    let mut points = Vec::new();
    let contents: String = std::fs::read_to_string(file_path).expect("File path not available.");
    let lines: Vec<&str> = contents.split("\n").collect();
    for line in lines.iter() {
        if line.contains("=IFCCARTESIANPOINT(") {
            if let Ok(direction) = IfcCartesianPoint::parse(line) {
                points.push(Box::new(direction));
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