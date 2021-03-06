use im::HashMap;
use num::{Num, NumCast};
use std::{hash::Hash, ops::AddAssign};

#[derive(Debug, Clone)]
pub struct Manager<T, I>
where
    T: Hash + Eq + Clone,
    I: Copy + Eq + AddAssign + Num + NumCast + Hash,
{
    data_to_id: HashMap<T, I>,
    id_to_data: HashMap<I, T>,
    next_available: I,
}

impl<T, I> Manager<T, I>
where
    T: Hash + Eq + Clone,
    I: Copy + Eq + AddAssign + Num + NumCast + Hash,
{
    pub fn new(init_id: I) -> Self {
        Manager {
            data_to_id: HashMap::<T, I>::new(),
            id_to_data: HashMap::<I, T>::new(),
            next_available: init_id,
        }
    }

    pub fn get_id(&mut self, data: &T) -> I {
        let r = self.data_to_id.get(&data);
        match r {
            Some(id) => id.clone(),
            None => self.registry(data.clone()),
        }
    }

    pub fn get_data(&self, id: I) -> Option<&T> {
        self.id_to_data.get(&id)
    }

    fn registry(&mut self, data: T) -> I {
        let r = self.next_available;
        self.data_to_id.insert(data.clone(), self.next_available);
        self.id_to_data.insert(self.next_available, data);
        let _1: I = NumCast::from(1usize).unwrap();
        self.next_available += _1;
        r
    }
}
