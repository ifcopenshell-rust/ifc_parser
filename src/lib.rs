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
use regex::Regex;
use std::collections::HashMap;

pub struct IfcParser;

pub struct Model {
    data: HashMap<usize, Box<dyn IfcEntity>>,
    regex_map: HashMap<String, Regex>,
}

impl Default for Model {
    fn default() -> Self {
        let mut model = Model {
            data: HashMap::new(),
            regex_map: HashMap::new(),
        };

        model
            .regex_map
            .insert("IFCCARTESIANPOINT".to_string(), Regex::new(r"#(\d+)=IFCCARTESIANPOINT\(\(([\d\.,\-]+)\)\)").unwrap());
        model
    }
}

impl Model {
    pub fn parse_data(&mut self, s: &str) -> Result<Box<dyn IfcEntity>, ()> {
        if s.contains("=IFCCARTESIANPOINT(") {
            match IfcCartesianPoint::from_str(s, self.regex_map.get("IFCCARTESIANPOINT").unwrap()) {
                Ok(point) => {
                    log::info!("Parsed point: {:?}", point);
                    return Ok(Box::new(point));
                }
                Err(_e) => {
                    log::warn!("Error parsing line");
                    return Err(());
                }
            }
        }
        Err(())
    }

    pub fn by_id(&self, id: &usize) -> Option<&Box<dyn IfcEntity>> {
        self.data.get(id)
    }
}

pub fn from_file(path: &str) -> Result<Model, ()> {
    let data = match std::fs::read_to_string(path) {
        Ok(file) => file,
        Err(_) => {
            log::error!("Unable to open file");
            return Err(());
        }
    };

    let data = data.split("ENDSEC;").collect::<Vec<&str>>();

    let mut data = data[1].split(";\n").collect::<Vec<&str>>();
    data.remove(0);

    let mut model = Model::default();

    for value in data {
        match model.parse_data(value) {
            Ok(point) => {
                model.data.insert(point.id(), point);
            }
            Err(_e) => {
                log::warn!("Error parsing line");
            }
        }
    }

    Ok(model)
}

mod data;
use data::*;
pub trait IfcEntity {
    fn from_str(s: &str, regex: &Regex) -> Result<Self, ()>
    where
        Self: Sized;

    fn to_string(&self) -> String;

    fn id(&self) -> usize;
}

use core::fmt::Debug;
impl Debug for dyn IfcEntity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
