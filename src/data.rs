// =============================================================================
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
use super::*;

#[derive(Debug)]
pub struct IfcCartesianPoint {
    id: usize,
    x: f32,
    y: f32,
    z: Option<f32>,
}

impl IfcEntity for IfcCartesianPoint {
    fn from_str(s: &str, regex: &Regex) -> Result<Self, ()> {
        if let Some(captures) = regex.captures(&s) {
            let id = captures.get(1).unwrap().as_str().to_string();
            let coordinates: Vec<f32> = captures.get(2).unwrap().as_str().split(',').map(|s| s.parse::<f32>().unwrap()).collect();
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
        } else {
            Err(())
        }
    }

    fn to_string(&self) -> String {
        if let Some(z) = self.z {
            return format!("IFCCARTESIANPOINT({},{},{})", self.x, self.y, z);
        } else {
            return format!("IFCCARTESIANPOINT({},{})", self.x, self.y);
        }
    }

    fn id(&self) -> usize {
        self.id
    }
}

#[derive(Debug)]
pub struct IfcDirection {
    id: usize,
    x: f32,
    y: f32,
    z: Option<f32>,
}

impl IfcEntity for IfcDirection {
    fn from_str(s: &str, regex: &Regex) -> Result<Self, ()> {
        if let Some(captures) = regex.captures(&s) {
            let id = captures.get(1).unwrap().as_str().to_string();
            let coordinates: Vec<f32> = captures.get(2).unwrap().as_str().split(',').map(|s| s.parse::<f32>().unwrap()).collect();
            if coordinates.len() == 3 {
                return Ok(IfcDirection {
                    id: id.parse().unwrap(),
                    x: coordinates[0],
                    y: coordinates[1],
                    z: Some(coordinates[2]),
                });
            } else {
                return Ok(IfcDirection {
                    id: id.parse().unwrap(),
                    x: coordinates[0],
                    y: coordinates[1],
                    z: None,
                });
            }
        } else {
            Err(())
        }
    }

    fn to_string(&self) -> String {
        if let Some(z) = self.z {
            return format!("IFCDIRECTION({},{},{})", self.x, self.y, z);
        } else {
            return format!("IFCDIRECTION({},{})", self.x, self.y);
        }
    }

    fn id(&self) -> usize {
        self.id
    }
}
