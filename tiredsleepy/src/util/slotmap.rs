use std::{collections::VecDeque, marker::PhantomData, mem::MaybeUninit, vec::Vec};

extern crate bitfield;

// unused (3 bit) | generation (16 bit) | index (12 bit) | active (1 bit)
// xxx              xxxxxxxxxxxxxxxx      xxxxxxxxxxxx     x
bitfield::bitfield! {
    #[derive(Copy, Clone)]
    struct Meta(u32);
    impl Debug;
    u8;
    pub active, set_active: 0, 0;
    u16;
    pub index, set_index: 12, 1;
    pub generation, set_generation: 28, 13;
}

impl Meta {
    pub fn new(active: bool, index: u16, generation: u16) -> Self {
        Meta((generation << 13) as u32 | ((index & 0xfff) << 1) as u32 | (active as u32))
    }
}

const PAGE_SIZE: u16 = 4096;
const MIN_FREE_LIST_LEN: usize = 32;
const INVALID_GENERATION: u16 = 0;

struct Page<T> {
    data: [MaybeUninit<T>; PAGE_SIZE as usize],
    meta: [Meta; PAGE_SIZE as usize],
    used: u16,
}

// page (20 bit)        | meta_idx (12 bit)
// xxxxxxxxxxxxxxxxxxxx   xxxxxxxxxxxx
bitfield::bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct Index(u32);
    impl Debug;
    u16;
    pub meta_idx, set_meta_idx: 11, 0;
    u32;
    pub page, set_page: 31, 12;
}

impl Index {
    pub fn new(meta_idx: u16, page: u32) -> Self {
        Index(((meta_idx & 0xfff) as u32) | ((page & 0xfffff) as u32) << 12)
    }
}

#[derive(Default)]
pub struct Key<T> {
    index: Index,
    generation: u16,

    _marker: PhantomData<T>,
}

impl<T> Key<T> {
    pub fn new(index: u16, page: u32, generation: u16) -> Self {
        Self {
            index: Index::new(index, page),
            generation,
            _marker: PhantomData,
        }
    }
}

impl<T> Clone for Key<T> {
    fn clone(&self) -> Key<T> {
        Key {
            index: self.index,
            generation: self.generation,
            _marker: PhantomData,
        }
    }
}

impl<T> Copy for Key<T> {}

// #[derive(Default)]
pub struct Slotmap<T> {
    pages: Vec<Page<T>>,
    free_list: VecDeque<Index>,
}

impl<T> Default for Slotmap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Slotmap<T> {
    pub fn new() -> Self {
        Self {
            pages: vec![],
            free_list: VecDeque::new(),
        }
    }

    pub fn insert<U: Into<T>>(&mut self, value: U) -> Key<T> {
        if self.free_list.len() < MIN_FREE_LIST_LEN {
            if self.pages.is_empty() || self.pages.last().unwrap().used == PAGE_SIZE {
                self.pages.push(Page {
                    data: MaybeUninit::uninit_array(),
                    meta: [Meta(0); PAGE_SIZE as usize],
                    used: 0,
                });
            }

            let mut page = self.pages.last_mut().unwrap();

            page.data[page.used as usize].write(value.into());
            page.meta[page.used as usize] = Meta::new(true, page.used, 1);

            page.used += 1;

            Key::new(page.used - 1, (self.pages.len() - 1) as u32, 1)
        } else {
            let idx = self.free_list.pop_front().unwrap();

            let &(mut meta) = &self.pages[idx.page() as usize].meta[idx.meta_idx() as usize];
            meta.set_active(1);

            self.pages[idx.page() as usize].data[meta.index() as usize].write(value.into());

            Key::new(idx.meta_idx(), idx.page(), meta.generation())
        }
    }

    pub fn has(&self, k: Key<T>) -> bool {
        k.index.page() < self.pages.len().try_into().unwrap()
            && k.generation
                == self.pages[k.index.page() as usize].meta[k.index.meta_idx() as usize]
                    .generation()
    }

    pub fn try_get_mut(&mut self, k: Key<T>) -> Result<&mut T, ()> {
        if !self.has(k.clone()) {
            return Err(());
        }

        let page = &mut self.pages[k.index.page() as usize];
        Ok(unsafe {
            page.data[page.meta[k.index.meta_idx() as usize].index() as usize].assume_init_mut()
        })
    }

    pub fn try_get(&self, k: Key<T>) -> Result<&T, ()> {
        if !self.has(k) {
            return Err(());
        }

        let page = &self.pages[k.index.page() as usize];
        Ok(unsafe {
            page.data[page.meta[k.index.meta_idx() as usize].index() as usize].assume_init_ref()
        })
    }

    pub fn remove(&mut self, k: Key<T>) {
        if self.has(k) {
            let page = &mut self.pages[k.index.page() as usize];

            let meta = &mut page.meta[k.index.meta_idx() as usize];
            meta.set_active(0);
            meta.set_generation(if meta.generation() == u16::MAX {
                INVALID_GENERATION
            } else {
                meta.generation() + 1
            });

            unsafe { page.data[meta.index() as usize].assume_init_drop() }
        }
    }
}

impl<T> Drop for Slotmap<T> {
    fn drop(&mut self) {
        for page in self.pages.iter_mut() {
            for meta in page.meta {
                if meta.active() != 0 {
                    unsafe { page.data[meta.index() as usize].assume_init_drop() }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn slotmap_basic_usage() -> Result<(), ()> {
        let mut sm = Slotmap::<u32>::new();

        let k1 = sm.insert(6u32);

        assert_eq!(*sm.try_get(k1)?, 6);
        *sm.try_get_mut(k1)? = 4;

        assert_eq!(*sm.try_get(k1)?, 4);

        sm.remove(k1);
        assert!(sm.try_get(k1).is_err());

        Ok(())
    }

    #[test]
    fn slotmap_handles_multiple_pages() {
        const COUNT: u32 = 30000;

        let mut keys = [Key::<u32>::default(); COUNT as usize];

        let mut sm = Slotmap::<u32>::new();

        for i in 0..COUNT {
            keys[i as usize] = sm.insert(i);
        }

        for i in 0..COUNT {
            assert_eq!(*sm.try_get(keys[i as usize]).unwrap(), i)
        }
    }

    // keep these tests isolated
    mod drop_tests {
        use super::*;

        struct Dropped {
            x: u32,
        }

        static mut DROP_COUNT: u32 = 0;

        impl Drop for Dropped {
            fn drop(&mut self) {
                unsafe {
                    DROP_COUNT += 1;
                }
            }
        }

        #[test]
        fn slotmap_handles_drops() {
            {
                let mut sm = Slotmap::<Dropped>::new();
                let k1 = sm.insert(Dropped { x: 3 });
                sm.insert(Dropped { x: 3 });
                sm.insert(Dropped { x: 3 });

                assert_eq!(unsafe { DROP_COUNT }, 0);

                sm.remove(k1);
                assert_eq!(unsafe { DROP_COUNT }, 1);
            }

            assert_eq!(unsafe { DROP_COUNT }, 3);
        }
    }

    #[test]
    fn slotmap_stress() -> Result<(), ()> {
        const COUNT: u32 = 400_000;

        let mut keys: Vec<(Key<u32>, bool)> = vec![];

        let mut sm = Slotmap::<u32>::new();
        let check = |sm: &Slotmap<u32>, keys: &Vec<(Key<u32>, bool)>| -> Result<(), ()> {
            for (i, k) in keys.iter().enumerate() {
                if k.1 {
                    assert_eq!(*sm.try_get(k.0)?, i as u32);
                } else {
                    assert!(sm.try_get(k.0).is_err());
                }
            }
            Ok(())
        };

        for i in 0..COUNT {
            keys.push((sm.insert(i), true));
        }

        check(&sm, &keys)?;

        for k in keys[1000..3000].iter_mut() {
            sm.remove(k.0);
            k.1 = false;
        }

        check(&sm, &keys)?;

        for k in keys[10000..30000].iter_mut() {
            sm.remove(k.0);
            k.1 = false;
        }

        check(&sm, &keys)?;

        for (i, k) in keys[10000..15000].iter_mut().enumerate() {
            k.0 = sm.insert((i + 10000) as u32);
            k.1 = true;
        }

        check(&sm, &keys)?;

        for (i, k) in keys[20000..20010].iter_mut().enumerate() {
            k.0 = sm.insert((i + 20000) as u32);
            k.1 = true;
        }

        check(&sm, &keys)?;

        Ok(())
    }
}
