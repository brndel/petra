use std::collections::HashMap;

use leptos::{expect_context, provide_context, Resource, ServerFnError, SignalGet, SignalWith};
use mensula_key::Key;

use crate::util::reload_signal::ReloadSignal;

use super::{Provide, VecMap};

pub struct Provider<T: Provide + 'static> {
    resource: Resource<bool, Result<T::ResourceData, ServerFnError>>,
    reload_signal: ReloadSignal,
}

impl<T: Provide + 'static> Clone for Provider<T> {
    fn clone(&self) -> Self {
        Self {
            resource: self.resource.clone(),
            reload_signal: self.reload_signal.clone(),
        }
    }
}

impl<T: Provide + 'static> Copy for Provider<T> {}

impl<T: Provide + 'static> Provider<T> {
    pub fn provide() {
        Self::provide_with_reload(ReloadSignal::new());
    }

    pub fn provide_with_reload(reload_signal: ReloadSignal) {
        provide_context(Self {
            reload_signal,
            resource: Resource::new(
                move || reload_signal.get(),
                |_| async {
                    let data = T::get_data().await?;
                    Ok(T::map_data(data))
                },
            ),
        });
    }

    pub fn expect() -> Self {
        expect_context::<Self>()
    }

    pub fn reload(&self) {
        self.reload_signal.reload();
    }

    pub fn resource(&self) -> Resource<bool, Result<T::ResourceData, ServerFnError>> {
        self.resource
    }
}

impl<T: Provide<Data = D, ResourceData = D> + 'static, D: Clone + 'static> Provider<T> {

    #[track_caller]
    pub fn get_single(&self) -> Option<D> {
        self.resource.with(|data| match data {
            Some(Ok(data)) => Some(data.clone()),
            _ => None,
        })
    }

    pub fn get_single_id(&self) -> Option<Key> {
        self.resource.with(|data| match data {
            Some(Ok(data)) => Some(T::get_id(data)),
            _ => None,
        })
    }
}

impl<T: Provide<ResourceData = VecMap<D>, Data = D> + 'static, D: Clone + 'static> Provider<T> {
    pub fn get(&self, id: &Key) -> Option<T::Data> {
        self.with_map(|data| data.get(id).cloned()).flatten()
    }

    pub fn get_all(&self) -> Option<Vec<T::Data>> {
        self.with_vec(|data| data.clone())
    }

    pub fn get_all_ids(&self) -> Option<Vec<Key>> {
        self.with_vec(|vec| vec.iter().map(|v| T::get_id(v)).collect())
    }

    pub fn get_multiple(&self, ids: &[Key]) -> Option<Vec<T::Data>> {
        self.with_map(|map| ids.iter().filter_map(|id| map.get(id).cloned()).collect())
    }

    fn with_vec<O>(&self, f: impl FnOnce(&Vec<D>) -> O) -> Option<O> {
        self.resource.with(|res| match res {
            Some(Ok((vec, _))) => Some(f(vec)),
            _ => None,
        })
    }

    fn with_map<O>(&self, f: impl FnOnce(&HashMap<Key, D>) -> O) -> Option<O> {
        self.resource.with(|res| match res {
            Some(Ok((_, map))) => Some(f(map)),
            _ => None,
        })
    }
}
