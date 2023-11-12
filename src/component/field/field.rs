use leptos::{
    RwSignal, SignalGet, SignalGetUntracked, SignalSet, SignalUpdate, SignalWith,
    SignalWithUntracked,
};

pub struct Field<T: 'static> {
    signal: RwSignal<InnerField<T>>,
}

impl<T> Copy for Field<T> {}

impl<T> Clone for Field<T> {
    fn clone(&self) -> Self {
        *self
    }
}

struct InnerField<T> {
    start_value: T,
    current_value: T,
}

impl<T: 'static + Clone + PartialEq> Field<T> {
    pub fn new(value: T) -> Self {
        Self {
            signal: RwSignal::new(InnerField::new(value)),
        }
    }
}

impl<T: 'static + Clone> InnerField<T> {
    fn new(value: T) -> Self {
        Self {
            start_value: value.clone(),
            current_value: value,
        }
    }
}

// FieldReset

pub trait FieldReset {
    fn reset(&self);
}

impl<T: 'static + Clone> FieldReset for Field<T> {
    fn reset(&self) {
        self.signal
            .update(|inner| inner.current_value = inner.start_value.clone())
    }
}

impl<T1: FieldReset, T2: FieldReset> FieldReset for (T1, T2) {
    fn reset(&self) {
        self.0.reset();
        self.1.reset();
    }
}

impl<T1: FieldReset, T2: FieldReset, T3: FieldReset> FieldReset for (T1, T2, T3) {
    fn reset(&self) {
        self.0.reset();
        self.1.reset();
        self.2.reset();
    }
}

impl<T1: FieldReset, T2: FieldReset, T3: FieldReset, T4: FieldReset> FieldReset for (T1, T2, T3, T4) {
    fn reset(&self) {
        self.0.reset();
        self.1.reset();
        self.2.reset();
        self.3.reset();
    }
}

// FieldUnchanged

pub trait FieldUnchaged {
    fn is_unchanged(&self) -> bool;
}

impl<T: 'static + PartialEq> FieldUnchaged for Field<T> {
    fn is_unchanged(&self) -> bool {
        self.signal
            .with(|inner| inner.start_value == inner.current_value)
    }
}

impl<T1: FieldUnchaged, T2: FieldUnchaged> FieldUnchaged for (T1, T2) {
    fn is_unchanged(&self) -> bool {
        self.0.is_unchanged() && self.1.is_unchanged()
    }
}

impl<T1: FieldUnchaged, T2: FieldUnchaged, T3: FieldUnchaged> FieldUnchaged for (T1, T2, T3) {
    fn is_unchanged(&self) -> bool {
        self.0.is_unchanged() && self.1.is_unchanged() && self.2.is_unchanged()
    }
}

impl<T1: FieldUnchaged, T2: FieldUnchaged, T3: FieldUnchaged, T4: FieldUnchaged> FieldUnchaged for (T1, T2, T3, T4) {
    fn is_unchanged(&self) -> bool {
        self.0.is_unchanged() && self.1.is_unchanged() && self.2.is_unchanged() && self.3.is_unchanged()
    }
}

// Signal implenentations

impl<T: 'static + Clone> SignalSet for Field<T> {
    type Value = T;

    fn set(&self, new_value: Self::Value) {
        self.signal.update(|inner| inner.current_value = new_value)
    }

    fn try_set(&self, new_value: Self::Value) -> Option<Self::Value> {
        match self
            .signal
            .try_update(|inner| inner.current_value = new_value.clone())
        {
            Some(_) => None,
            None => Some(new_value),
        }
    }
}

impl<T: 'static + Clone> SignalGet for Field<T> {
    type Value = T;

    fn get(&self) -> Self::Value {
        self.signal.with(|inner| inner.current_value.clone())
    }

    fn try_get(&self) -> Option<Self::Value> {
        self.signal.try_with(|inner| inner.current_value.clone())
    }
}

impl<T: 'static + Clone> SignalGetUntracked for Field<T> {
    type Value = T;

    fn get_untracked(&self) -> Self::Value {
        self.signal
            .with_untracked(|inner| inner.current_value.clone())
    }

    fn try_get_untracked(&self) -> Option<Self::Value> {
        self.signal
            .try_with_untracked(|inner| inner.current_value.clone())
    }
}

impl<T: 'static + Clone> SignalWith for Field<T> {
    type Value = T;

    fn with<O>(&self, f: impl FnOnce(&Self::Value) -> O) -> O {
        self.signal.with(|inner| f(&inner.current_value))
    }

    fn try_with<O>(&self, f: impl FnOnce(&Self::Value) -> O) -> Option<O> {
        self.signal.try_with(|inner| f(&inner.current_value))
    }
}

impl<T: 'static + Clone> SignalUpdate for Field<T> {
    type Value = T;

    fn update(&self, f: impl FnOnce(&mut Self::Value)) {
        self.signal.update(|inner| f(&mut inner.current_value))
    }

    fn try_update<O>(&self, f: impl FnOnce(&mut Self::Value) -> O)
        -> Option<O> {
        self.signal.try_update(|inner| f(&mut inner.current_value))
    }

}