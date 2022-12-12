use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]

    // トークンをMintする
    pub fn nft_mint(&mut self, mut metadata: TokenMetadata, receiver_id: AccountId) {
        // 投票が開かれていることを確認
        assert!(
            !(&self.is_election_closed),
            "You cannot add candidate or voter because this election has been closed"
        );
        // selfから持ってきたTokenIdCounterからtokenIdをセット
        // Some()で囲むことでTokenが存在しないストレージにアクセスした際にNone()が返るようにする
        metadata.token_id = Some(self.token_id_counter);
        // 現時点でContractが占有しているStorageを確認
        let initial_storage_usage = env::storage_usage();
        let receiver_id_clone = receiver_id.clone();
        let token = TokenOwner {
            owner_id: receiver_id,
        };
        let token_id = self.token_id_counter;
        let token_kind = metadata.token_kind.clone();

        assert!(
            self.tokens_by_id
                .insert(&self.token_id_counter, &token)
                .is_none(),
            "Token already exists"
        );

        // 情報をMapに追加する
        self.token_metadata_by_id
            .insert(&self.token_id_counter, &metadata);
        
        self.internal_add_token_to_owner(&token.owner_id, &token_id);

        self.internal_add_token_to_kind_map(&token_id, token_kind);

        self.likes_per_candidate
            .insert(&self.token_id_counter, &(0 as Likes));

        self.added_voter_list
            .insert(&receiver_id_clone, &self.token_id_counter);
        
        // increment token id 
        self.token_id_count()

        // ストレージのユーザーデータを計算する
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        // 利用されなかったデポジット料金を返す
        refund_deposit(required_storage_in_bytes);
    }

    // count token id
    pub fn token_id_count(&mut self) {
        self.token_id_counter = self.token_id_counter + 1;
    }

    // get next token id 
    pub fn show_token_id_counter(&self) -> u128 {
        self.token_id_counter
    }
}
