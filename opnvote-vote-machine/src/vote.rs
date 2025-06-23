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
use read_io::read;
use opnvote_common::electoral_list::ElectoralList;

pub fn collect_vote(electoral_list: &ElectoralList) -> i32 {
    let mut candidates = electoral_list.candidates.clone();
    candidates.sort_by(|a, b| a.id.cmp(&b.id));
    
    for candidate in candidates.iter() {
        let party = electoral_list.parties.iter().find(|p| p.id == candidate.party_id ).unwrap();
        println!("{}. {} ({})", candidate.id, candidate.name, party.name);
    }
    
    let mut vote: i32;
    
    //TODO: for some reason this loops twice
    loop {
        vote = read!("Enter your vote: ").unwrap();
        if vote <= 0 || vote >= candidates.len() as i32 {
            println!("Invalid Vote! try again");
        } else {
            println!("You have selected {}", candidates.iter().find(|c| c.id == vote).unwrap().name);
            break
        }
    }
    vote
}