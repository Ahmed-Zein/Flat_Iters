pub fn flatten<T>(iter: T) -> Flatten<T>
where
    T: Iterator,
    T::Item: IntoIterator,
{
    Flatten::new(iter)
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    pub fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            inner: None,
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
            if let Some(ref mut inner_iter) = self.inner {
                if let Some(i) = inner_iter.next() {
                    return Some(i);
                }
                self.inner = None;
            }
            let inner_iter = self.outer.next()?.into_iter();
            self.inner = Some(inner_iter);
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
        // assert_eq!(flatten(std::iter::once(vec!["1", "2"])).count(), 2)
        assert_eq!(flatten(vec![vec!["1", "2"]].iter()).count(), 2)
    }
    #[test]
    fn two_wide() {
        assert_eq!(flatten(vec![vec!["1"], vec!["2"]].iter()).count(), 2)
    }
}

/*
 *  fn main() {
 *      let vs = vec![1, 2, 3];
 *      for i in vs.iter() {
 *          // borrows vs, & to v
 *      }
 *      for i in vs {
 *          // consumnes vs, owned v
 *      }
 *  }
 * */
