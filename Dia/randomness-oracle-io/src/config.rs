use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId, TypeInfo};

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct InitConfig {
    pub manager: ActorId,
}
