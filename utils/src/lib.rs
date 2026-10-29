use core::fmt;
use std::iter::Peekable;

pub struct PeekingTakeWhile<'iter, I: Iterator, P: FnMut(&I::Item) -> bool> {
    iter: &'iter mut Peekable<I>,
    flag: bool,
    predicate: P,
}

impl<'iter, I: Iterator, P: Fn(&I::Item) -> bool> Iterator for PeekingTakeWhile<'iter, I, P> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.flag {
            return None;
        }
        let Some(item) = self.iter.next_if(&self.predicate) else {
            self.flag = true;
            return None;
        };
        Some(item)
    }
}

impl<'iter, I: Iterator, P: FnMut(&I::Item) -> bool> PeekingTakeWhile<'iter, I, P> {
    pub const fn new(iter: &'iter mut Peekable<I>, predicate: P) -> PeekingTakeWhile<'iter, I, P> {
        PeekingTakeWhile {
            iter,
            flag: false,
            predicate,
        }
    }
}

impl<'iter, I: fmt::Debug + Iterator, P> fmt::Debug for PeekingTakeWhile<'iter, I, P>
where
    Peekable<I>: fmt::Debug,
    P: FnMut(&I::Item) -> bool,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PeekingTakeWhile")
            .field("iter", &self.iter)
            .field("flag", &self.flag)
            .finish()
    }
}

pub trait ToPeekingTakeWhile<'iter, I: Iterator>: Iterator {
    fn peeking_take_while<P>(&'iter mut self, predicate: P) -> PeekingTakeWhile<'iter, I, P>
    where
        Self: Sized,
        P: FnMut(&I::Item) -> bool;
}

impl<'iter, I: Iterator> ToPeekingTakeWhile<'iter, I> for Peekable<I> {
    fn peeking_take_while<P>(&'iter mut self, predicate: P) -> PeekingTakeWhile<'iter, I, P>
    where
        Self: Sized,
        P: FnMut(&I::Item) -> bool,
    {
        PeekingTakeWhile::new(self, predicate)
    }
}
