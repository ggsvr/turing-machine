#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoubleVec<T> {
    back: Vec<T>,
    front: Vec<T>,
}

impl<T> DoubleVec<T> {
    pub fn new() -> Self {
        Self {
            back: Vec::new(),
            front: Vec::new(),
        }
    }

    pub fn with<V: Into<Vec<T>>, U: Into<Vec<T>>>(back: V, front: U) -> Self {
        Self {
            back: back.into(),
            front: front.into(),
        }
    }

    pub fn len(&self) -> usize {
        self.back.len() + self.front.len()
    }

    pub fn get(&self, index: isize) -> Option<&T> {
        let (v, i) = self.transform_index(index);
        v.get(i)
    }

    pub fn get_mut(&mut self, index: isize) -> Option<&mut T> {
        let (v, i) = self.transform_index_mut(index);
        v.get_mut(i)
    }

    pub fn push_front(&mut self, value: T) {
        self.front.push(value);
    }
    pub fn pop_front(&mut self) -> Option<T> {
        self.front.pop()
    }

    pub fn push_back(&mut self, value: T) {
        self.back.push(value)
    }
    pub fn pop_back(&mut self) -> Option<T> {
        self.back.pop()
    }

    pub fn insert(&mut self, index: isize, value: T) {
        let (v, i) = self.transform_index_mut(index);
        v.insert(i, value);
    }

    pub fn remove(&mut self, index: isize) -> T {
        let (v, i) = self.transform_index_mut(index);
        v.remove(i)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter::new(self)
    }
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut::new(self)
    }

    pub fn grow_with(&mut self, to: isize, f: impl FnMut() -> T) {
        let (v, newlen) = self.transform_index_mut(to);
        if newlen >= v.len() {
            v.resize_with(newlen, f);
        }
        //let (newlen, is_positive) = calc_index(to);

        //if is_positive {
        //    if newlen >= self.front.len() {
        //        self.front.resize_with(newlen, f);
        //    }
        //} else {
        //    if newlen >= self.back.len() {
        //        self.back.resize_with(newlen, f);
        //    }
        //}
    }

    pub fn fits(&self, index: isize) -> bool {
        let (v, i) = self.transform_index(index);
        i < v.len()
        //let (index, is_positive) = calc_index(index);

        //if is_positive {
        //    index < self.front.len()
        //} else {
        //    index < self.back.len()
        //}
    }

    pub fn back(&self) -> &[T] {
        &self.back
    }
    pub fn back_vec(&self) -> &Vec<T> {
        &self.back
    }
    pub fn front(&self) -> &[T] {
        &self.front
    }
    pub fn front_vec(&self) -> &Vec<T> {
        &self.front
    }
    pub fn back_mut(&mut self) -> &mut [T] {
        &mut self.back
    }
    pub fn back_vec_mut(&mut self) -> &mut Vec<T> {
        &mut self.back
    }
    pub fn front_mut(&mut self) -> &mut [T] {
        &mut self.front
    }
    pub fn front_vec_mut(&mut self) -> &mut Vec<T> {
        &mut self.front
    }

    fn transform_index(&self, index: isize) -> (&Vec<T>, usize) {
        let (index, is_positive) = calc_index(index);
        (if is_positive { &self.front } else { &self.back }, index)
    }
    fn transform_index_mut(&mut self, index: isize) -> (&mut Vec<T>, usize) {
        let (index, is_positive) = calc_index(index);
        (
            if is_positive {
                &mut self.front
            } else {
                &mut self.back
            },
            index,
        )
    }
}

fn calc_index(index: isize) -> (usize, bool) {
    if index < 0 {
        ((index * -1 - 1) as usize, false)
    } else {
        (index as usize, true)
    }
}

impl<T: Clone> DoubleVec<T> {
    pub fn grow(&mut self, to: isize, value: T) {
        let (v, newlen) = self.transform_index_mut(to);

        if newlen >= v.len() {
            v.resize(newlen, value);
        }

        //let (newlen, is_positive) = calc_index(to);

        //if is_positive {
        //    if newlen >= self.front.len() {
        //        self.front.resize(newlen, value);
        //    }
        //} else {
        //    if newlen >= self.back.len() {
        //        self.back.resize(newlen, value);
        //    }
        //}
    }
}

impl<T> std::ops::Index<isize> for DoubleVec<T> {
    type Output = T;

    fn index(&self, index: isize) -> &Self::Output {
        let (v, i) = self.transform_index(index);
        &v[i]
        //let (index, is_positive) = calc_index(index);

        //if is_positive {
        //    &self.front[index]
        //} else {
        //    &self.back[index]
        //}
    }
}

impl<T> std::ops::IndexMut<isize> for DoubleVec<T> {
    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        let (v, i) = self.transform_index_mut(index);
        &mut v[i]
        //let (index, is_positive) = calc_index(index);

        //if is_positive {
        //    &mut self.front[index]
        //} else {
        //    &mut self.back[index]
        //}
    }
}

impl<T> IntoIterator for DoubleVec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

pub struct Iter<'a, T> {
    back: std::iter::Rev<std::slice::Iter<'a, T>>,
    front: std::slice::Iter<'a, T>,
}

impl<'a, T> Iter<'a, T> {
    pub fn new(v: &'a DoubleVec<T>) -> Self {
        Self {
            back: v.back().iter().rev(),
            front: v.front().iter(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.back.next().or(self.front.next())
    }
}

pub struct IterMut<'a, T> {
    back: std::iter::Rev<std::slice::IterMut<'a, T>>,
    front: std::slice::IterMut<'a, T>,
}

impl<'a, T> IterMut<'a, T> {
    pub fn new(v: &'a mut DoubleVec<T>) -> Self {
        Self {
            back: v.back.iter_mut().rev(),
            front: v.front.iter_mut(),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.back.next().or(self.front.next())
    }
}

pub struct IntoIter<T> {
    back: std::iter::Rev<std::vec::IntoIter<T>>,
    front: std::vec::IntoIter<T>,
}

impl<T> IntoIter<T> {
    pub fn new(dv: DoubleVec<T>) -> Self {
        Self {
            back: dv.back.into_iter().rev(),
            front: dv.front.into_iter(),
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.back.next().or(self.front.next())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doublevec_new() {
        let mut v1 = DoubleVec::new();
        (0..4).for_each(|i| v1.push_front(i));
        (1..5).for_each(|i| v1.push_back(-i));

        let v2 = DoubleVec::with([-1, -2, -3, -4], [0, 1, 2, 3]);
        assert_eq!(v1, v2);
    }
}
