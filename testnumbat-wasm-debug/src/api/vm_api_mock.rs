use testnumbat_wasm::{api::VMApi, testnumbat_codec::TryStaticCast};

use crate::TxContext;

impl TryStaticCast for TxContext {}

impl VMApi for TxContext {}
