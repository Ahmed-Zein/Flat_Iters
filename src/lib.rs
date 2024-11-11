pub fn flatten<T>(iter: T) -> Flatten<T::IntoIter>
where
    T: IntoIterator,
    T::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    front_iter: Option<<O::Item as IntoIterator>::IntoIter>,
    back_iter: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    pub fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            front_iter: None,
            back_iter: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut front_iter) = self.front_iter {
                if let Some(i) = front_iter.next() {
                    return Some(i);
                }
                self.front_iter = None;
            }
            match self.outer.next() {
                Some(front_iter) => {
                    self.front_iter = Some(front_iter.into_iter());
                }
                None => return self.back_iter.as_mut()?.next(),
            }
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut back_iter) = self.back_iter {
                if let Some(i) = back_iter.next_back() {
                    return Some(i);
                }
                self.back_iter = None;
            }
            match self.outer.next_back() {
                Some(back_iter) => {
                    self.back_iter = Some(back_iter.into_iter());
                }
                None => return self.front_iter.as_mut()?.next(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0)
    }
    #[test]
    fn one() {
        assert_eq!(flatten(std::iter::once(vec![0])).count(), 1)
    }
    #[test]
    fn two() {
        assert_eq!(flatten(vec![vec!["1", "2"]]).count(), 2)
    }
    #[test]
    fn two_wide() {
        assert_eq!(flatten(vec![vec!["1"], vec!["2"]]).count(), 2)
    }

    #[test]
    fn rev_empty() {
        assert_eq!(
            flatten(Vec::<Vec<()>>::new()).rev().collect::<Vec<()>>(),
            vec![]
        );
    }

    #[test]
    fn rev() {
        assert_eq!(
            flatten(vec![vec!["a"], vec!["b"]])
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        );
    }

    #[test]
    fn rev_two() {
        assert_eq!(
            flatten(vec![vec!["a"], vec!["b", "c"]])
                .rev()
                .collect::<Vec<_>>(),
            vec!["c", "b", "a"]
        );
    }

    #[test]
    fn double_ended() {
        let mut flat_iter = flatten(vec![vec!["a1", "a2", "a3"], vec!["b1", "b2", "b3", "b4"]]);

        assert_eq!(flat_iter.next(), Some("a1"));
        assert_eq!(flat_iter.next_back(), Some("b4"));

        assert_eq!(flat_iter.next(), Some("a2"));
        assert_eq!(flat_iter.next_back(), Some("b3"));

        assert_eq!(flat_iter.next(), Some("a3"));
        assert_eq!(flat_iter.next_back(), Some("b2"));

        assert_eq!(flat_iter.next(), Some("b1"));
        assert_eq!(flat_iter.next_back(), None);

        assert_eq!(flat_iter.next_back(), None);
    }
}
