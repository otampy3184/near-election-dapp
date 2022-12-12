use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise};

mod vote;
mod enumeration;
mod internal;
mod metadata;
mod mint;
mod nft_core;

pub use crate::enumeration::*;
use crate::internal::*;
pub use crate::metadata::*;
pub use crate::mint::*;
pub use crate::nft_core::*;
pub use vote::*;

// Nearチェーン上で使えるようにする
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // コントラクトのオーナーを管理、AccountIdは自動的にWalletIdとしてチェックしてくれる便利な方    
    pub owner_id: AccountId,
    // WalletId => NFT Metadata
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,
    // Kind(種類) => 種類に紐づくNFT
    pub tokens_per_kind: LookupMap<TokenKind, UnorderedSet<TokenId>>,
    // TokenId => TokenOwner
    pub tokens_by_id: LookupMap<TokenId, TokenOwner>,
    // TokenId => Metadata
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,
    // NFTのメタデータ配列を格納
    pub metadata: LazyOption<NFTContractMetadata>,
    // TokenIdをカウントする。SolidityでいうところのCounter
    pub token_id_counter: u128,
    // Token => 得票数
    pub likes_per_candidate: LookupMap<TokenId, Likes>,
    // WalletId => TokenId
    pub added_voter_list: LookupMap<ReceiverId, u128>,
    // WalletId => 0 | 1
    pub voted_voter_list: LookupMap<ReceiverId, u128>,
    // 選挙が終わっているかどうか
    pub is_election_closed: bool,
}

#[derive(BorshDeserialize)]
// 上の変数に対応するenumの変数で、初期化の際に必要
// それぞれの変数が格納されるストレージのアドレスの接頭辞をユニークにするため
pub enum StorageKey {
    TokensPerOwner,
    TokensPerKind,
    TokensPerOwnerInner{ account_id_hash: CryptoHash },
    TokensPerKindInner { token_kind: TokenKind },
    TokensById,
    TokenMetadataById,
    TokensPerTypeInner: { token_type_hash: CryptoHash },
    NFTContractMetadata,
    LikesPerCandidate,
    AddedVoterList,
    VotedVoterList,
}

#[near_bindgen]
impl Contract {
    //初期化関数であることを宣言
    #[init]
    // new関数で変数を初期化していく
    // newでインスタンスを生成し、Result型のVector (配列)を作成し、unwrap()メソッドでResult<Vec<u8>>Vec<u8>に変換している
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        let this = Self{
            owner_id,
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_per_kind: LookupMap::new(StorageKey::TokensPerKind.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(
                StorageKey::TokenMetadataById.try_to_vec().unwrap(),
            ),
            metadata: LazyOption:: new(
                StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
            token_id_counter: 0,
            likes_per_candidate: LookupMap::new(
                StorageKey::likes_per_candidate.try_to_vec().unwrap(),
            ),
            added_voter_list: LookupMap::new(StorageKey::AddedVoterList.try_to_vec().unwrap()),
            voted_voter_list: LookupMap::new(StorageKey::voted_voter_list.try_to_vec().unwrap()),
            is_election_closed: false,
        };
        // 初期化した変数を返す
        this
    }
    // 初期化関数newを呼び出す
    pub fn new_default_meta(owner_id: AccountId) -> Self{
        Self:: new(
            owner_id,
            NFTContractMetadata {
                spec: "nft-1.0.0".to_string(),
                name: "Near Vote Contract".to_string(),
                reference: "This contract is designed for fair election".to_string(),
            },
        )
    }
}