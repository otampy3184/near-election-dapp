use crate::*;

pub trait NonFungibleTokenCore {
    fn nft_token(&self, token_id: TokenId) -> Option<JsonToken>;
}

#[near_bindgen]
impl NonFungibleTokenCore for Contract {
    // TokenIdに対応するMetdataが存在するかを確認して、ある場合はそれを返す
    fn nft_token(&self, token_id: TokenId) -> Option<JsonToken>{
        if let Some(token) = self.tokens_by_id.get(&token_id){
            let metadata = self.token_metadata_by_id.get(&token_id).unwrap();
            Some(JsonToken {
                owner_id: token.owner_id,
                metadata,
            })
        } else {
            None
        }
    }
}