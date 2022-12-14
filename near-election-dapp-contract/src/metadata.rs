// クレートルートのlib.rs内の全てを利用可能
use crate::*;

// 変数宣言
pub type TokenId = u128;
pub type CandidateName = String;
pub type TokenKind = String;
pub type HasVoted = bool;
pub type ReceivedId = AccountId;
pub type Likes = f32;

// 方に対して使えるTrateを増やす
// Trate＝特定の方に存在する共通の振る舞い
// BorshDeserialize等が提供するTrateをNFTContractMetadataが使えるようにする
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]

// Contractのメタデータ
pub struct NFTContractMetadata {
    pub spec: String,
    pub name: String,
    pub reference: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
// TokenMetadataの構造体
pub struct TokenMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub media: String,
    pub media_CID: String,
    pub candidate_name: Option<String>,
    pub candidate_manifest: Option<String>,
    pub token_kind: String,
    pub token_id: Option<u128>,
}

#[derive(BorshDeserialize, BorshSerialize )]
pub struct TokenOwner {
    pub owner_id: AccountId,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]

pub struct JsonToken {
    pub owner_id: AccountId,
    pub metadata: TokenMetadata,
}

// Trateを宣言
pub trait NFTTokenMetadata { 
    fn nft_metadata(&self) -> NFTContractMetadata;
}

#[near_bindgen]
impl NFTTokenMetadata for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata{
        self.metadata.get().unwrap()
    }
}