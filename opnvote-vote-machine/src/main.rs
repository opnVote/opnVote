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
mod vote;

use opnvote_common::electoral_list::ElectoralList;
use opnvote_common::token::{PublishToken, VoteToken};
use crate::vote::collect_vote;

fn main() {
    let token = VoteToken::new("resources/keys/ed25519.pk8").unwrap();
    
    token.verify_signature("resources/keys/ed25519-public.pub").unwrap();
    
    if token.is_revoked() {
        eprintln!("This token has been used already!");
        return;
    }
    
    let electoral_list = ElectoralList::from_file("resources/electoral_list.json");
    
    let vote = collect_vote(&electoral_list);
    
    let signature = token.sign_vote_and_revoke(collect_vote(&electoral_list)).unwrap();
    
    //TODO: send this to the vote controller
    println!("You voted for : {:?}", electoral_list.candidates.iter().find(|c| c.id == vote));
    println!("Your vote signature: {}", signature);
    println!("Token: {:?}", token.to_publish_token())
}
