use ethlambda_types::primitives::H256;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("storage error: {0}")]
    Storage(#[from] crate::api::Error),
    #[error("missing block header for root {0:?}")]
    MissingBlockHeader(H256),
    #[error("missing block body for root {0:?}")]
    MissingBlockBody(H256),
    #[error("missing state for root {0:?}")]
    MissingState(H256),
    #[error("missing metadata for key {0:?}")]
    MissingMetadata(Vec<u8>),
}
