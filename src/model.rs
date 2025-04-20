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

pub struct Model {
    data: HashMap<usize, Box<dyn IfcEntity>>,
    regex_map: HashMap<&'static str, Regex>,
}

impl Default for Model {
    fn default() -> Self {
        let mut model = Model {
            data: HashMap::new(),
            regex_map: HashMap::new(),
        };

        model
            .regex_map
            .insert("IFCCARTESIANPOINT", Regex::new(r"#(\d+)=IFCCARTESIANPOINT\(\(([\d\.,\-]+)\)\)").unwrap());

        model.regex_map.insert("IFCDIRECTION", Regex::new(r"#(\d+)=IFCDIRECTION\(\(([\d\.,\-]+)\)\)").unwrap());

        model
    }
}

impl Model {
    pub fn parse_data(&mut self, s: &str) -> Result<Box<dyn IfcEntity>, ()> {
        if s.contains("=IFCCARTESIANPOINT(") {
            match IfcCartesianPoint::from_str(s, self.regex_map.get("IFCCARTESIANPOINT").unwrap()) {
                Ok(data) => {
                    log::debug!("Parsed data: {:?}", data);
                    return Ok(Box::new(data));
                }
                Err(_e) => {
                    log::warn!("Error parsing line");
                    return Err(());
                }
            }
        } else if s.contains("=IFCDIRECTION(") {
            match IfcDirection::from_str(s, self.regex_map.get("IFCDIRECTION").unwrap()) {
                Ok(data) => {
                    log::debug!("Parsed data: {:?}", data);
                    return Ok(Box::new(data));
                }
                Err(_e) => {
                    log::warn!("Error parsing line");
                    return Err(());
                }
            }
        }
        Err(())
    }

    pub fn add_data(&mut self, id: usize, data: Box<dyn IfcEntity>) {
        self.data.insert(id, data);
    }

    pub fn by_id(&self, id: &usize) -> Option<&Box<dyn IfcEntity>> {
        self.data.get(id)
    }
}
