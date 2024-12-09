use crate::AndesApiImpl;
use testnumbat_wasm::{api::VMApi, testnumbat_codec::TryStaticCast};

impl TryStaticCast for AndesApiImpl {}

impl VMApi for AndesApiImpl {}
