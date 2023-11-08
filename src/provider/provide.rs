use std::{collections::HashMap, future::Future, pin::Pin};

use leptos::ServerFnError;
use mensula_key::Key;
use serde::{de::DeserializeOwned, Serialize};

use crate::api::{
    category::{get_categories, get_category_groups, Category, CategoryGroup},
    rule::{get_rules, Rule}, user::{me, User, get_users},
};

use super::VecMap;

pub trait Provide: Sized {
    type Data: 'static;
    type ResourceData: 'static + Serialize + DeserializeOwned;
    type ResultData: 'static;

    fn get_data() -> Pin<Box<dyn Future<Output = Result<Self::ResultData, ServerFnError>>>>;
    fn map_data(data: Self::ResultData) -> Self::ResourceData;
    fn get_id(data: &Self::Data) -> Key;
}

fn to_vec_map<T: Provide<Data = D>, D: Clone>(data: Vec<D>) -> VecMap<D> {
    let map = HashMap::from_iter(data.iter().map(|data| (T::get_id(data), data.clone())));

    (data, map)
}

impl Provide for Category {
    type Data = Self;
    type ResourceData = VecMap<Self>;
    type ResultData = Vec<Self>;

    fn get_data() -> Pin<Box<dyn Future<Output = Result<Self::ResultData, ServerFnError>>>> {
        Box::pin(get_categories())
    }

    fn map_data(data: Self::ResultData) -> Self::ResourceData {
        to_vec_map::<Self, _>(data)
    }

    fn get_id(data: &Self::Data) -> Key {
        data.id.to_owned()
    }
}

impl Provide for CategoryGroup {
    type Data = Self;
    type ResourceData = VecMap<Self>;
    type ResultData = Vec<Self>;

    fn get_data() -> Pin<Box<dyn Future<Output = Result<Self::ResultData, ServerFnError>>>> {
        Box::pin(get_category_groups())
    }

    fn map_data(data: Self::ResultData) -> Self::ResourceData {
        to_vec_map::<Self, _>(data)
    }

    fn get_id(data: &Self::Data) -> Key {
        data.id.to_owned()
    }
}

impl Provide for Rule {
    type Data = Self;
    type ResourceData = VecMap<Self>;
    type ResultData = Vec<Self>;

    fn get_data() -> Pin<Box<dyn Future<Output = Result<Self::ResultData, ServerFnError>>>> {
        Box::pin(get_rules())
    }

    fn map_data(data: Self::ResultData) -> Self::ResourceData {
        to_vec_map::<Self, _>(data)
    }

    fn get_id(data: &Self::Data) -> Key {
        data.id.to_owned()
    }
}

impl Provide for User {
    type Data = Self;
    type ResourceData = VecMap<Self>;
    type ResultData = Vec<Self>;

    fn get_data() -> Pin<Box<dyn Future<Output = Result<Self::ResultData, ServerFnError>>>> {
        Box::pin(get_users())
    }

    fn map_data(data: Self::ResultData) -> Self::ResourceData {
        to_vec_map::<Self, _>(data)
    }

    fn get_id(data: &Self::Data) -> Key {
        data.id.to_owned()
    }
}

pub struct Me;

impl Provide for Me {
    type Data = User;
    type ResourceData = User;
    type ResultData = User;

    fn get_data() -> Pin<Box<dyn Future<Output = Result<Self::ResultData, ServerFnError>>>> {
        Box::pin(me())
    }

    fn map_data(data: Self::ResultData) -> Self::ResourceData {
        data
    }

    fn get_id(data: &Self::Data) -> Key {
        data.id.clone()
    }
}
