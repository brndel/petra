use std::collections::HashMap;

use mensula_key::Key;

mod provider;
mod provide;
pub use provider::Provider;
pub use provide::Provide;
pub use provide::Me;

type VecMap<T> = (Vec<T>, HashMap<Key, T>);
