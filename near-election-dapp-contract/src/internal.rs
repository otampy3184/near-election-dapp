use crate::*;
use near_sdk::CryptoHash;

// WalletIDをハッシュ化して返すだけ
pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

// 引数で受けとったStorageUsageをガス代に換算して、余った額を返金する
pub(crate) fn refund_deposit(storage_usage: u64) {
    let required_cost = env::storage_byte_cost() * Balance::from(storage_usage);
    let attached_deposit = env::attached_deposit();

    assert!(
        required_cost <= attached_deposit,
        "Must attach {} yoctoNear to cover storage",
        required_cost,
    );

    let refund = attached_deposit - required_cost;
    if refund > 1 {
        Promise::new(env::predecessor_account_id()).transfer(refund);
    }
}

impl Contract {
    pu(crate) fn internal_add_token_to_owner(
        &mut self,
        account_id: &AccountId,
        token_id: &TokenId,
    ) {
        // 引数であるAccountIdに対するTokenの値が紐づいているTokenPerOwnerマップを代入
        // unwrap or elseでAccountIdに対するTokenの値が存在していない場合は新しくベクターを作る
        let mut token_set = self.tokens_per_owner.get(account_id).unwrap_or_else(|| {
            // UnorderdSetを使い、AccountIdをハッシュ化したユニークな接頭辞を作成する
            UnorderedSet::new(
                StorageKey::TokensPerOwnerInner {
                    account_id_hash: hash_account_id(&account_id);
                }
                .try_to_vec()
                .unwrap(),
            )
        });
        tokens_set.insert(token_id);
        // AccountIdに紐づいたTokenSetのマップをTokenPerOwnerに追加する
        // TokenがOwnerに追加される
        self.tokens_per_owner.insert(account_id, &tokens_set);
    }

    // 上の関数とやっていることはほぼ同じで、TokenIdとTokenKindが結びついているという点だけが違う
    pub(crate) fn internal_add_token_to_kind_map(
        &mut self,
        token_id: &TokenId,
        token_kind: TokenKin,
    ) {
        let token_kind_clone = token_kind.clone();
        let mut tokens_set = self
            .tokens_per_kind
            .get(&token_kind_clone)
            .unwrap_or_else(|| {
                UnorderedSet::new(
                    StorageKey::TokensPerKindInner {
                        token_kind: token_kind;
                    }
                    .try_to_vec()
                    .unwrap(),
                )
            });
        tokens_set.insert(&token_id);
        self.tokens_per_kind.insert(&token_kind_clone, &tokens_set);
    }
}