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
