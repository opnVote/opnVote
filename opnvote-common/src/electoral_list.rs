/*
opnVote - Reliable, verifiable and secure electronic voting systems
Copyright (C)  2025  Max Bossing <max@bossi.ng>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use serde::{Deserialize, Serialize};

//TODO: Make everything referencing each other
// So that it is possible that we can get the party from 
// a candidate, and all candidates from a party
// without having to search for them

#[derive(Debug, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct Party {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct Candidate {
    pub id: i32,
    pub name: String,
    pub party_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElectoralList {
    pub parties: Vec<Party>,
    pub candidates: Vec<Candidate>,
}

impl ElectoralList {
    //TODO: MVP-Code, this has to be loaded and signature verified from backbone
    pub fn from_file(path: &str) -> Self {
        serde_json::from_str(std::fs::read_to_string(path).unwrap().as_str()).unwrap() //TODO: Error handling
    }
}
