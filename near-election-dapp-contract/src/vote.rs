use crate::*;

#[near_bindegen]
impl Contract {
    // check if election is closed
    pub fn if_election_closed(&self) -> bool {
        self.is_election_closed
    }
    
    // close election
    pub fn close_election(&mut self) {
        self.is_election_closed = true;
     }
    
    // reopen election
    pub fn reopen_election(&mut self) {
        self.is_election_closed = false;
    }
    // get number of likes of specified candidate
    pub fn nft_return_candidate_likes(&self, token_id: TokenId) -> Likes {
         if self.tokens_by_id.get(&token_id).is_some() {
            self.likes_per_candidate.get(&token_id).unwrap()
        } else {
            0 as Likes
        }
    }
    
    // add info(key: receiver id, value: number ) to map(-> this list is for check voter has already voted)
    pub fn voter_voted(&mut self, voter_id: AccountId) {
        self.voted_voter_list.insert(&voter_id, &(0 as u128));
    }
    
    // check if voter id is in added-list
    pub fn check_voter_has_been_added(&self, voter_id: AccountId) -> TokenId {
        if self.added_voter_list.get(&voter_id).is_some() {
            return self.added_voter_list.get(&voter_id).unwrap();
        } else {
            0
        }
    }
    
    // check if voter id is in voted-list
    pub fn check_voter_has_voted(&self, voter_id: AccountId) -> bool {
        if self.voted_voter_list.get(&voter_id).is_some() {
           return true;
        } else {
           false
       }
    }
}