use leptos::{RwSignal, SignalUpdate, SignalGet, logging::log};


#[derive(Clone, Copy)]
pub struct ReloadSignal {
    signal: RwSignal<bool>,
    name: Option<&'static str>,
}

impl ReloadSignal {
    pub fn new() -> Self {
        Self {
            signal: RwSignal::new(false),
            name: None,
        }
    }

    pub fn with_name(name: &'static str) -> Self {
        Self {
            signal: RwSignal::new(false),
            name: Some(name)
        }
    }

    // pub fn subscribing<F: Fn() -> T + Copy + 'static, T>(f: F) -> (Self, impl Fn() -> T + 'static + Copy) {
    //     let this = Self::new();
    //     let fun = this.subscribe(f);

    //     (this, fun)
    // }

    pub fn reload(&self) {
        if let Some(name) = self.name {
            log!("reloading '{}'", name);
        }
        self.signal.update(|v| *v = !*v);
    }

    pub fn subscribe<F: Fn() -> T + Copy + 'static, T>(self, f: F) -> impl Fn() -> T + 'static + Copy {
        move || {
            self.reload();
            f()
        }
    }
}

impl SignalGet for ReloadSignal {
    type Value = bool;

    fn get(&self) -> Self::Value {
        self.signal.get()
    }

    fn try_get(&self) -> Option<Self::Value> {
        self.signal.try_get()
    }
}