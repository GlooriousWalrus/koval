use leptos::*;

pub mod components;

// Library items
#[derive(Debug, Clone)]
pub struct OptionSignal<T: 'static>(Option<Signal<T>>);

impl<T> Default for OptionSignal<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T: 'static, I: Into<Signal<T>>> From<I> for OptionSignal<T> {
    fn from(value: I) -> Self {
        Self(Some(value.into()))
    }
}

#[derive(Debug, Clone)]
pub struct OptionMaybeSignal<T: 'static>(Option<MaybeSignal<T>>);

impl<T: Copy> Copy for OptionMaybeSignal<T> {}

impl<T> Default for OptionMaybeSignal<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T: 'static, I: Into<MaybeSignal<T>>> From<I> for OptionMaybeSignal<T> {
    fn from(value: I) -> Self {
        Self(Some(value.into()))
    }
}

impl<T: Clone + Default> SignalGet for OptionMaybeSignal<T> {
    fn get(&self) -> T {
        match &self.0 {
            Some(signal) => signal.get(),
            None => T::default(),
        }
    }

    fn try_get(&self) -> Option<T> {
        match &self.0 {
            Some(signal) => signal.try_get(),
            None => Some(T::default()),
        }
    }

    type Value = T;
}

impl<T: Clone + Default> SignalGetUntracked for OptionMaybeSignal<T> {
    fn get_untracked(&self) -> T {
        match &self.0 {
            Some(signal) => signal.get_untracked(),
            None => T::default(),
        }
    }

    fn try_get_untracked(&self) -> Option<T> {
        match &self.0 {
            Some(signal) => signal.try_get_untracked(),
            None => Some(T::default()),
        }
    }

    type Value = T;
}
