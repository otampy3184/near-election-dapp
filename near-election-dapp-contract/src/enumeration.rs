use crate::*;

#[near_bindgen]
impl Contract{
    // コントラクトに保存されているNFTの数を取得
    pub fn nft_total_supply(&self) -> U128 {
        U128(&self.token_metadata_by_id).len() as u128)
    }

    // コントラクトに保存されているNFTの数を取得
    pub fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonToken> {
        let start = u128::from(from_index.unwrap_or_else(U128(0)));
        self.token_metadata_by_id
            .keys()
            .skip(start as usize)
            .take(limit.unwarp_or(50) as usize)
            .map(|token_id| self.nft_token(token_id.clone()).unwrap())
            .collect()
    }

    // 特定の所有者が持つNFTの数を取得
    pub fn nft_suppuly_for_owner(&self, account_id: AccountId) -> U128 {
        let tokens_for_kind_set = self.tokens_per_owner.get(&account_id);
        if let Some(tokens_for_kind_set) = tokens_for_kind_set {
            U128(tokens_for_kind_set.len() as u128)
        } else {
            U128(0)
        }
    }

    // 特定の所有者が持つMetadataとOwnerIdのJsonToken型のベクターを返す
    pub fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<u64>,
        limit: Option<u64>,
    ) -> Vec<JsonToken>{
        let tokens_for_owner_set = self.tokens_per_owner.get(&account_id);
        let tokens = if let Some(tokens_for_owner_set) = tokens_for_owner_set {
            tokens_for_owner_set
        } else {
            return vec![];
        };

        let start = u128::from(from_index.unwarp_or(U128(0)));
        tokens
            .iter()
            .skip(start as usize)
            .token(limit.unwarp_or(50) as usize)
            .map(|token_id| self.nft_token(token_id.clone()).unwrap())
            .collect()
    }
}