use crate::runtime::LVARS;
use std::ops::{Add, Div, Mul, Sub};
use std::marker::PhantomData;

pub struct LVar<T> {
    pub id: topo::CallId,
    phantom_data: PhantomData<T>,
}

impl<T> std::fmt::Debug for LVar<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:#?})", self.id)
    }
}

impl<T> Copy for LVar<T> {}
impl<T> Clone for LVar<T> {
    fn clone(&self) -> LVar<T> {
        LVar::<T> {
            id: self.id,
            phantom_data: PhantomData::<T>,
        }
    }
}

impl<T> LVar<T>
where
    T: 'static,
{
    pub(crate) fn new(id: topo::CallId) -> LVar<T> {
        LVar {
            id,
            phantom_data: PhantomData,
        }
    }

    pub fn set(self, data: T) {
        LVARS.with(|l_vars| {
            l_vars
                .borrow_mut()
                .insert(self.id, data)
        });
    }

    pub fn remove(self) -> Option<T> {
        LVARS.with(|l_vars| {
            l_vars
                .borrow_mut()
                .remove::<T>(&self.id)
        })
    }

    // pub fn reset_on_unmount(self) -> Self {
    //     on_unmount(move || self.delete());
    //     self
    // }

    pub fn update<F: FnOnce(T) -> T>(self, updater: F) {
        let data = self.remove().expect("an l_var data with the given id");
        self.set(updater(data));
    }

    pub fn update_mut<F: FnOnce(&mut T) -> ()>(self, updater: F) {
        let mut data = self.remove().expect("an l_var data with the given id");
        updater(&mut data);
        self.set(data);
    }

    pub fn exists(self) -> bool {
        LVARS.with(|l_vars| {
            l_vars.borrow().contains_id(&self.id)
        })
    }

    pub fn get_with<F: FnOnce(&T) -> U, U>(self, getter: F) -> U {
        LVARS.with(|l_vars| {
            let l_var_map = l_vars.borrow();
            let data = l_var_map.data(&self.id)
                .expect("an l_var data with the given id");
            getter(data)
        })
    }
}

pub trait CloneLVar<T: Clone + 'static> {
    fn inner(&self) -> T;
}

impl<T: Clone + 'static> CloneLVar<T> for LVar<T> {
    fn inner(&self) -> T {
        self.get_with(Clone::clone)
    }
}

// #[derive(Clone)]
// struct ChangedWrapper<T>(T);

// pub trait ChangedLVar {
//     fn changed(&self) -> bool;
// }

// impl<T> ChangedLVar for LVar<T>
// where
//     T: Clone + 'static + PartialEq,
// {
//     fn changed(&self) -> bool {
//         if let Some(old_l_var) = clone_l_var_with_topo_id::<ChangedWrapper<T>>(self.id) {
//             old_l_var.0 != self.get()
//         } else {
//             set_l_var_with_topo_id(ChangedWrapper(self.get()), self.id);
//             true
//         }
//     }
// }

impl<T> std::fmt::Display for LVar<T>
where
    T: std::fmt::Display + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_with(|t| format!("{}", t)))
    }
}

impl<T> Add for LVar<T>
where
    T: Copy + Add<Output = T> + 'static,
{
    type Output = T;

    fn add(self, other: Self) -> Self::Output {
        self.get_with(|s| other.get_with(|o| *o + *s))
    }
}

impl<T> Mul for LVar<T>
where
    T: Copy + Mul<Output = T> + 'static,
{
    type Output = T;

    fn mul(self, other: Self) -> Self::Output {
        self.get_with(|s| other.get_with(|o| *o * *s))
    }
}

impl<T> Div for LVar<T>
where
    T: Copy + Div<Output = T> + 'static,
{
    type Output = T;

    fn div(self, other: Self) -> Self::Output {
        self.get_with(|s| other.get_with(|o| *o / *s))
    }
}

impl<T> Sub for LVar<T>
where
    T: Copy + Sub<Output = T> + 'static,
{
    type Output = T;

    fn sub(self, other: Self) -> Self::Output {
        self.get_with(|s| other.get_with(|o| *o - *s))
    }
}
