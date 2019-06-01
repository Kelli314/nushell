use derive_new::new;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash, new)]
pub enum Type {
    Any,
}
