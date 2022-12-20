//! A skiplist implementation which allows faster random access than a standard linked list.
//!
//! Wikipedia: https://en.wikipedia.org/wiki/Skip_list

use std::cmp::Ordering;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::{iter, mem};

use crate::collection::error::CollectionError;
use crate::collection::skiplist::level_generator::{DefaultLevelGenerator, GenerateLevel};
use crate::collection::skiplist::skipnode::{Link, SkipNode};

/// The inner comparator in skiplist
type Comparator<T> = Box<dyn Fn(&T, &T) -> Ordering>;

/// The skiplist provides a way of storing elements such that they are
/// always sorted and at the same time provides efficient way to access, insert
/// and remove nodes. Just like `LinkedList`, it also provides access to indices.
///
/// By default, the SkipList uses the comparison function `a.cmp(b)`.
///
/// The skiplist has an associated sorting function which **must** be
/// well-behaved. Specifically, given some ordering function `f(a, b)`, it must
/// satisfy the following properties:
///
/// - Be well defined: `f(a, b)` should always return the same value
/// - Be anti-symmetric: `f(a, b) == Greater` iff `f(b, a) == Less` and `f(a, b)
///   == Equal == f(b, a)`.
/// - By transitive: If `f(a, b) == Greater` and `f(b, c) == Greater` then `f(a,
///   c) == Greater`.
///
/// **Failure to satisfy these properties can result in unexpected behavior at
/// best, and at worst will cause a segfault, null deref, or some other bad
/// behavior.**
pub struct OrdSkipList<T> {
    length: usize,
    head: NonNull<SkipNode<T>>,
    cmp: Comparator<T>,
    level_generator: Box<dyn GenerateLevel>,
    _marker: PhantomData<Box<SkipNode<T>>>,
}

/// The options to create a skip list
pub struct Options<T: 'static> {
    // Custom comparator
    pub cmp: Option<Comparator<T>>,
    // Use default level generator, but set different max level(default is 16)
    pub level_bound: Option<usize>,
    // Use custom level generator
    pub level_generator: Option<Box<dyn GenerateLevel>>,
}

impl<T> Options<T> {
    pub fn take_level_generator(&mut self) -> Result<Box<dyn GenerateLevel>, CollectionError> {
        match self.level_generator.take() {
            Some(g) => Ok(g),
            None => {
                let g = match self.level_bound {
                    Some(level_bound) => DefaultLevelGenerator::new(level_bound, 1.0 / 2.0)?,
                    None => DefaultLevelGenerator::default(),
                };
                Ok(Box::new(g))
            }
        }
    }

    pub fn take_comparator(&mut self) -> Result<Comparator<T>, CollectionError> {
        match self.cmp.take() {
            Some(cmp) => Ok(Box::new(cmp)),
            None => Err(CollectionError::InvalidParameter(
                "comparator must be provided".to_string(),
            )),
        }
    }
}

impl<T: Ord> OrdSkipList<T> {
    pub fn new() -> Self {
        let g = DefaultLevelGenerator::default();
        Self {
            length: 0,
            cmp: Box::new(|x, y| x.cmp(y)),
            head: NonNull::new(Box::into_raw(Box::new(SkipNode::head(g.level_bound())))).unwrap(),
            level_generator: Box::new(g),
            _marker: PhantomData,
        }
    }

    pub fn ord_with_options(mut options: Options<T>) -> Result<Self, CollectionError> {
        if options.cmp.is_none() {
            options.cmp = Some(Box::new(|x, y| x.cmp(y)))
        }
        Self::with_options(options)
    }
}

impl<T: Ord> Default for OrdSkipList<T> {
    fn default() -> Self {
        let g = DefaultLevelGenerator::default();
        Self {
            length: 0,
            cmp: Box::new(|x, y| x.cmp(y)),
            head: NonNull::new(Box::into_raw(Box::new(SkipNode::head(g.level_bound())))).unwrap(),
            level_generator: Box::new(g),
            _marker: PhantomData,
        }
    }
}

impl<T> OrdSkipList<T> {
    pub fn with_options(mut options: Options<T>) -> Result<Self, CollectionError> {
        let g = options.take_level_generator()?;
        Ok(Self {
            length: 0,
            head: NonNull::new(Box::into_raw(Box::new(SkipNode::head(g.level_bound())))).unwrap(),
            cmp: options.take_comparator()?,
            level_generator: g,
            _marker: PhantomData,
        })
    }

    /// Returns true if the value is contained in the skiplist.
    pub fn contains(&self, v: &T) -> bool {
        unsafe {
            let mut cur = self.head.as_ref();

            for i in (0..=cur.level).rev() {
                while cur.next[i].is_some() {
                    let next_node = cur.next[i].unwrap().as_ref();
                    if (self.cmp)(next_node.val.as_ref().unwrap(), v) == Ordering::Less {
                        cur = next_node;
                    } else {
                        break;
                    }
                }
                if cur.next[i].is_some()
                    && (self.cmp)(cur.next[i].unwrap().as_ref().val.as_ref().unwrap(), v)
                        == Ordering::Equal
                {
                    return true;
                }
            }
        }
        false
    }

    /// Insert a new node by the given data
    pub fn insert(&mut self, data: T) -> Result<(), CollectionError> {
        if self.contains(&data) {
            return Err(CollectionError::DuplicateKey);
        }

        let level = self.level_generator.random();
        let new_node = Box::new(SkipNode::new(data, level));
        let mut new_node = NonNull::new(Box::into_raw(new_node));

        unsafe {
            let mut cur = self.head.as_mut();
            for i in (0..=cur.level).rev() {
                while cur.next[i].is_some() {
                    let next_node = cur.next[i].unwrap().as_mut();
                    if (self.cmp)(
                        next_node.val.as_ref().unwrap(),
                        new_node.as_ref().unwrap().as_ref().val.as_ref().unwrap(),
                    ) == Ordering::Less
                    {
                        cur = next_node;
                    } else {
                        break;
                    }
                }

                if level >= i {
                    match cur.next[i] {
                        Some(next) => {
                            cur.next[i] = new_node;
                            new_node.as_mut().unwrap().as_mut().next[i] = Some(next);
                        }
                        None => {
                            cur.next[i] = new_node;
                        }
                    }
                }
            }
        }

        self.length += 1;

        Ok(())
    }

    /// Peek the front value
    pub fn peek_front(&self) -> Option<&T> {
        unsafe {
            let first_node = self.head.as_ref().next[0].as_ref();
            match first_node {
                Some(node) => node.as_ref().val.as_ref(),
                None => None,
            }
        }
    }

    /// Remove the node that value equals to the given value
    pub fn remove(&mut self, val: &T) -> Option<T> {
        if !self.contains(val) {
            return None;
        }

        let mut cur = unsafe { self.head.as_mut() };
        let max_level = cur.level;
        let mut update: Vec<Option<*mut SkipNode<T>>> = vec![None; max_level + 1];
        let ret_val;
        unsafe {
            for i in (0..=max_level).rev() {
                while cur.next[i].is_some() {
                    let next_node = cur.next[i].unwrap().as_mut();
                    if (self.cmp)(next_node.val.as_ref().unwrap(), val) == Ordering::Less {
                        cur = next_node;
                    } else {
                        break;
                    }
                }
                update[i] = Some(cur as *mut SkipNode<T>);
            }

            let mut ret_val_ref = None;
            if cur.next[0].is_some()
                && (self.cmp)(cur.next[0].unwrap().as_ref().val.as_ref().unwrap(), val)
                    == Ordering::Equal
            {
                ret_val_ref = cur.next[0];
                for i in (0..=max_level).rev() {
                    if update[i].is_some()
                        && (*update[i].unwrap()).next[i].is_some()
                        && (self.cmp)(
                            (*update[i].unwrap()).next[i]
                                .unwrap()
                                .as_mut()
                                .val
                                .as_ref()
                                .unwrap(),
                            val,
                        ) == Ordering::Equal
                    {
                        (*update[i].unwrap()).next[i] =
                            (*update[i].unwrap()).next[i].unwrap().as_mut().next[i];
                    }
                }
            }
            ret_val = match ret_val_ref {
                None => None,
                Some(ret) => Box::from_raw(ret.as_ptr()).into_val(),
            }
        }

        self.length -= 1;

        ret_val
    }

    /// Remove the first element from the skiplist
    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            let first_node = self.head.as_ref().next[0].as_ref();
            match first_node {
                Some(node) => self.remove(node.as_ref().val.as_ref().unwrap()),
                None => None,
            }
        }
    }

    /// Clears the skiplist, removing all values.
    pub fn clear(&mut self) {
        while self.pop_front().is_some() {}
    }

    /// Returns `true` if the skiplist contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Returns the number of elements in the skiplist.
    #[inline]
    pub fn length(&self) -> usize {
        self.length
    }

    pub fn iter(&self) -> Iter<T> {
        let node = unsafe { self.head.as_ref().next[0] };

        Iter {
            head: node,
            len: self.length,
            _marker: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        let node = unsafe { self.head.as_ref().next[0] };

        IterMut {
            head: node,
            len: self.length,
            _marker: PhantomData,
        }
    }
}

impl<T> Drop for OrdSkipList<T> {
    fn drop(&mut self) {
        struct DropGuard<'a, T>(&'a mut OrdSkipList<T>);

        impl<'a, T> Drop for DropGuard<'a, T> {
            fn drop(&mut self) {
                // Continue the same loop we do below. This only runs when a destructor has
                // panicked. If another one panics this will abort.
                while self.0.pop_front().is_some() {}
            }
        }

        while let Some(node) = self.pop_front() {
            let guard = DropGuard(self);
            drop(node);
            mem::forget(guard);
        }
        // println!("OrdSkipList dropped!")
    }
}

impl<T: Debug> OrdSkipList<T> {
    pub fn print(&self) {
        print!("[");
        self.iter().for_each(|i| {
            print!("{:?}, ", i);
        });
        println!("]");
    }
}

pub struct Iter<'a, T: 'a> {
    head: Link<T>,
    len: usize,
    _marker: PhantomData<&'a SkipNode<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            match self.head {
                Some(node) => {
                    self.len -= 1;

                    unsafe {
                        let node = &*node.as_ptr();
                        self.head = node.next[0];
                        node.val.as_ref()
                    }
                }
                None => None,
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

pub struct IterMut<'a, T: 'a> {
    head: Link<T>,
    len: usize,
    _marker: PhantomData<&'a mut SkipNode<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            match self.head {
                Some(node) => {
                    self.len -= 1;

                    unsafe {
                        let node = &mut *node.as_ptr();
                        self.head = node.next[0];
                        node.val.as_mut()
                    }
                }
                None => None,
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

pub struct IntoIter<T> {
    list: OrdSkipList<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.length, Some(self.list.length))
    }
}

impl<T> IntoIterator for OrdSkipList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

impl<T> Extend<T> for OrdSkipList<T> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iterable: I) {
        let iterator = iterable.into_iter();
        for element in iterator {
            self.insert(element).unwrap();
        }
    }
}

impl<T> iter::FromIterator<T> for OrdSkipList<T>
where
    T: Ord,
{
    #[inline]
    fn from_iter<I>(iter: I) -> OrdSkipList<T>
    where
        I: IntoIterator<Item = T>,
    {
        let mut skiplist = OrdSkipList::default();
        skiplist.extend(iter);
        skiplist
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        // only need to ensure all our elements are read;
        // buffer will clean itself up afterwards.
        for _ in &mut *self {}

        // println!("skiplist's intoIter has been dropped!")
    }
}

#[cfg(test)]
mod tests {
    use crate::collection::skiplist::level_generator::DefaultLevelGenerator;
    use crate::collection::skiplist::{Options, OrdSkipList};

    #[test]
    fn new() {
        let sl: OrdSkipList<i32> = OrdSkipList::new();
        assert_eq!(sl.length, 0);
    }

    #[test]
    fn ord_with_options_cmp() {
        let sl: OrdSkipList<i32> = OrdSkipList::with_options(Options {
            cmp: Some(Box::new(|x: &i32, y: &i32| y.cmp(x))),
            level_bound: None,
            level_generator: None,
        })
        .unwrap();
        assert_eq!(sl.length, 0);
    }

    #[test]
    fn ord_with_options_level_bound() {
        let sl: OrdSkipList<i32> = OrdSkipList::ord_with_options(Options {
            cmp: None,
            level_bound: Some(1024),
            level_generator: None,
        })
        .unwrap();
        assert_eq!(sl.length, 0);
    }

    #[test]
    fn ord_with_options_level_generator() {
        let g = DefaultLevelGenerator::new(4, 0.5).unwrap();
        let sl: OrdSkipList<i32> = OrdSkipList::ord_with_options(Options {
            cmp: None,
            level_bound: None,
            level_generator: Some(Box::new(g)),
        })
        .unwrap();
        assert_eq!(sl.length, 0);
    }

    #[test]
    fn with_options() {
        struct Foo {
            id: usize,
            data: String,
        }

        let mut sl: OrdSkipList<Foo> = OrdSkipList::with_options(Options {
            cmp: Some(Box::new(|x: &Foo, y: &Foo| y.id.cmp(&x.id))),
            level_bound: None,
            level_generator: None,
        })
        .unwrap();
        assert_eq!(sl.length, 0);

        sl.insert(Foo {
            id: 2,
            data: "2".to_string(),
        })
        .unwrap();
        sl.insert(Foo {
            id: 1,
            data: "1".to_string(),
        })
        .unwrap();

        let first = sl.iter().next().unwrap();
        assert_eq!(first.id, 2);
        assert_eq!(first.data, "2".to_string());
    }

    #[test]
    fn contains() {
        let mut l: OrdSkipList<i32> = OrdSkipList::ord_with_options(Options {
            cmp: None,
            level_bound: Some(16),
            level_generator: None,
        })
        .unwrap();

        let test_len = 10000;
        for i in 0..test_len {
            l.insert(i).unwrap();
        }
        assert_eq!(l.length(), test_len as usize);

        for i in 0..test_len {
            assert!(l.contains(&i));
        }
    }

    #[test]
    fn remove() {
        let mut l: OrdSkipList<i32> = OrdSkipList::new();
        l.insert(12).unwrap();
        assert_eq!(l.length(), 1);
        assert!(l.contains(&12));

        l.remove(&12).unwrap();
        assert_eq!(l.length(), 0);
        assert!(!l.contains(&12));

        l.insert(13).unwrap();
        assert_eq!(l.length(), 1);
        assert!(l.contains(&13));
    }

    #[test]
    fn iter() {
        let mut l: OrdSkipList<i32> = OrdSkipList::new();
        for i in 0..100 {
            l.insert(i).unwrap();
        }

        let mut x = 0;
        l.iter().for_each(|i| {
            assert_eq!(i, &x);
            x += 1;
        });
    }

    #[test]
    fn iter_mut() {
        let mut l: OrdSkipList<i32> = OrdSkipList::new();
        for i in 0..100 {
            l.insert(i).unwrap();
        }
        l.iter_mut().for_each(|node_val| *node_val += 1);
        for x in 1..101 {
            assert_eq!(x, l.pop_front().unwrap());
        }
    }

    #[test]
    fn clear() {
        let mut l: OrdSkipList<i32> = OrdSkipList::new();
        for i in 0..100 {
            l.insert(i).unwrap();
        }
        assert_eq!(l.length(), 100);

        l.clear();
        assert_eq!(l.length(), 0);
        for i in 0..100 {
            assert!(!l.contains(&i));
        }
    }

    #[test]
    fn into_iter() {
        let mut l: OrdSkipList<i32> = OrdSkipList::new();
        for i in 0..10 {
            l.insert(i).unwrap();
        }
        assert_eq!(l.length(), 10);

        let mut l_to_string = l
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        println!(
            "transform l into String vec, l_to_string: {:?}",
            l_to_string
        );

        for i in (0..10).rev() {
            assert_eq!(i.to_string(), l_to_string.pop().unwrap());
        }
        // Compiling err:
        // l.print();
    }
}
