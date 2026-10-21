#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Conditional iterator chaining without dependencies or allocation.
//!
//! Import [`ChainIfExt`] to append iterators based on a condition.

use core::fmt;
use core::iter::{Chain, Flatten, FusedIterator, OnceWith};
use core::marker::PhantomData;
use core::option::IntoIter as OptionIntoIter;

/// The optional tail used by [`ChainIfExt::chain_if`] and
/// [`ChainIfExt::chain_if_else`].
///
/// The implementation is private, so it can change without exposing its
/// concrete iterator adapters.
pub struct ChainIfTail<I>
where
    I: IntoIterator,
{
    inner: Flatten<OptionIntoIter<I>>,
}

impl<I> ChainIfTail<I>
where
    I: IntoIterator,
{
    fn new(condition: bool, iter: I) -> Self {
        Self {
            inner: condition.then_some(iter).into_iter().flatten(),
        }
    }
}

impl<I> Clone for ChainIfTail<I>
where
    I: IntoIterator + Clone,
    I::IntoIter: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<I> fmt::Debug for ChainIfTail<I>
where
    I: IntoIterator + fmt::Debug,
    I::IntoIter: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ChainIfTail")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<I> Iterator for ChainIfTail<I>
where
    I: IntoIterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.inner.nth(n)
    }

    fn fold<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        self.inner.fold(init, f)
    }

    fn count(self) -> usize {
        self.inner.count()
    }

    fn last(self) -> Option<Self::Item> {
        self.inner.last()
    }

    fn for_each<F>(self, f: F)
    where
        F: FnMut(Self::Item),
    {
        self.inner.for_each(f)
    }

    fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        self.inner.find(predicate)
    }

    fn find_map<B, F>(&mut self, f: F) -> Option<B>
    where
        F: FnMut(Self::Item) -> Option<B>,
    {
        self.inner.find_map(f)
    }

    fn position<P>(&mut self, predicate: P) -> Option<usize>
    where
        P: FnMut(Self::Item) -> bool,
    {
        self.inner.position(predicate)
    }

    fn any<P>(&mut self, predicate: P) -> bool
    where
        P: FnMut(Self::Item) -> bool,
    {
        self.inner.any(predicate)
    }

    fn all<P>(&mut self, predicate: P) -> bool
    where
        P: FnMut(Self::Item) -> bool,
    {
        self.inner.all(predicate)
    }
}

impl<I> DoubleEndedIterator for ChainIfTail<I>
where
    I: IntoIterator,
    I::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.inner.nth_back(n)
    }

    fn rfold<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        self.inner.rfold(init, f)
    }

    fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        self.inner.rfind(predicate)
    }
}

impl<I> FusedIterator for ChainIfTail<I> where I: IntoIterator {}

/// The lazily initialized optional tail used by
/// [`ChainIfExt::chain_if_lazy`] and [`ChainIfExt::chain_if_else_lazy`].
///
/// The factory runs at most once, on the first attempt to access the tail.
/// In forward iteration that happens after the base iterator is exhausted;
/// in reverse iteration the tail is accessed first.
pub struct ChainIfLazyTail<F, I>
where
    F: FnOnce() -> I,
    I: IntoIterator,
{
    inner: Flatten<Flatten<OptionIntoIter<OnceWith<F>>>>,
    marker: PhantomData<fn() -> I>,
}

impl<F, I> ChainIfLazyTail<F, I>
where
    F: FnOnce() -> I,
    I: IntoIterator,
{
    fn new(condition: bool, factory: F) -> Self {
        Self {
            inner: condition
                .then_some(core::iter::once_with(factory))
                .into_iter()
                .flatten()
                .flatten(),
            marker: PhantomData,
        }
    }
}

impl<F, I> Clone for ChainIfLazyTail<F, I>
where
    F: FnOnce() -> I + Clone,
    I: IntoIterator,
    I::IntoIter: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            marker: PhantomData,
        }
    }
}

impl<F, I> fmt::Debug for ChainIfLazyTail<F, I>
where
    F: FnOnce() -> I,
    I: IntoIterator,
    I::IntoIter: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ChainIfLazyTail")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<F, I> Iterator for ChainIfLazyTail<F, I>
where
    F: FnOnce() -> I,
    I: IntoIterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.inner.nth(n)
    }

    fn fold<B, Func>(self, init: B, f: Func) -> B
    where
        Func: FnMut(B, Self::Item) -> B,
    {
        self.inner.fold(init, f)
    }

    fn count(self) -> usize {
        self.inner.count()
    }

    fn last(self) -> Option<Self::Item> {
        self.inner.last()
    }

    fn for_each<Func>(self, f: Func)
    where
        Func: FnMut(Self::Item),
    {
        self.inner.for_each(f)
    }

    fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        self.inner.find(predicate)
    }

    fn find_map<B, Func>(&mut self, f: Func) -> Option<B>
    where
        Func: FnMut(Self::Item) -> Option<B>,
    {
        self.inner.find_map(f)
    }

    fn position<P>(&mut self, predicate: P) -> Option<usize>
    where
        P: FnMut(Self::Item) -> bool,
    {
        self.inner.position(predicate)
    }

    fn any<P>(&mut self, predicate: P) -> bool
    where
        P: FnMut(Self::Item) -> bool,
    {
        self.inner.any(predicate)
    }

    fn all<P>(&mut self, predicate: P) -> bool
    where
        P: FnMut(Self::Item) -> bool,
    {
        self.inner.all(predicate)
    }
}

impl<F, I> DoubleEndedIterator for ChainIfLazyTail<F, I>
where
    F: FnOnce() -> I,
    I: IntoIterator,
    I::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.inner.nth_back(n)
    }

    fn rfold<B, Func>(self, init: B, f: Func) -> B
    where
        Func: FnMut(B, Self::Item) -> B,
    {
        self.inner.rfold(init, f)
    }

    fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        self.inner.rfind(predicate)
    }
}

impl<F, I> FusedIterator for ChainIfLazyTail<F, I>
where
    F: FnOnce() -> I,
    I: IntoIterator,
{
}

/// Extension methods for conditionally appending iterators.
pub trait ChainIfExt: Iterator + Sized {
    /// Appends `iter` only when `condition` is `true`.
    ///
    /// The argument is evaluated by the caller before this method is called,
    /// even when `condition` is `false`. Use [`Self::chain_if_lazy`] to defer
    /// constructing the iterator.
    fn chain_if<I>(self, condition: bool, iter: I) -> Chain<Self, ChainIfTail<I>>
    where
        I: IntoIterator<Item = Self::Item>,
    {
        self.chain(ChainIfTail::new(condition, iter))
    }

    /// Appends an iterator made by `factory` only when `condition` is `true`.
    ///
    /// `factory` runs at most once, when the appended tail is first accessed,
    /// not when this method is called. With `next()`, this occurs after the
    /// base iterator is exhausted. With `next_back()`, the tail is accessed
    /// first, so the factory is called immediately on the first reverse step.
    /// If the condition is false or the tail is never accessed, it is not run.
    fn chain_if_lazy<F, I>(self, condition: bool, factory: F) -> Chain<Self, ChainIfLazyTail<F, I>>
    where
        F: FnOnce() -> I,
        I: IntoIterator<Item = Self::Item>,
    {
        self.chain(ChainIfLazyTail::new(condition, factory))
    }

    /// Appends `if_true` or `if_false` depending on `condition`.
    ///
    /// Both arguments are evaluated before this method is called, but only
    /// the selected iterator is consumed. The two iterator types may differ.
    fn chain_if_else<A, B>(
        self,
        condition: bool,
        if_true: A,
        if_false: B,
    ) -> Chain<Chain<Self, ChainIfTail<A>>, ChainIfTail<B>>
    where
        A: IntoIterator<Item = Self::Item>,
        B: IntoIterator<Item = Self::Item>,
    {
        self.chain_if(condition, if_true)
            .chain_if(!condition, if_false)
    }

    /// Appends the iterator produced by the selected factory.
    ///
    /// Only the selected factory can run, and only on first access to its
    /// tail. The two factories may return different iterator types.
    /// As with [`Self::chain_if_lazy`], reverse iteration accesses the tail
    /// before the base iterator.
    fn chain_if_else_lazy<FA, FB, A, B>(
        self,
        condition: bool,
        if_true: FA,
        if_false: FB,
    ) -> Chain<Chain<Self, ChainIfLazyTail<FA, A>>, ChainIfLazyTail<FB, B>>
    where
        FA: FnOnce() -> A,
        FB: FnOnce() -> B,
        A: IntoIterator<Item = Self::Item>,
        B: IntoIterator<Item = Self::Item>,
    {
        self.chain_if_lazy(condition, if_true)
            .chain_if_lazy(!condition, if_false)
    }
}

impl<I: Iterator> ChainIfExt for I {}

#[cfg(test)]
extern crate std;

#[cfg(test)]
mod tests {
    use super::*;
    use core::cell::Cell;

    #[test]
    fn conditional_and_else() {
        assert!((0..2).chain_if(true, 2..4).eq([0, 1, 2, 3]));
        assert!((0..2).chain_if(false, 2..4).eq([0, 1]));
        assert!((0..2)
            .chain_if_else(true, 10..12, [20, 21])
            .eq([0, 1, 10, 11]));
        assert!((0..2)
            .chain_if_else(false, 10..12, [20, 21])
            .eq([0, 1, 20, 21]));
    }

    #[test]
    fn lazy_defers_until_tail() {
        let calls = Cell::new(0);
        let mut it = (0..3).chain_if_lazy(true, || {
            calls.set(calls.get() + 1);
            3..6
        });
        assert_eq!(calls.get(), 0);
        assert_eq!(it.next(), Some(0));
        assert_eq!(calls.get(), 0);
        assert_eq!(it.by_ref().take(2).collect::<std::vec::Vec<_>>(), [1, 2]);
        assert_eq!(calls.get(), 0);
        assert_eq!(it.next(), Some(3));
        assert_eq!(calls.get(), 1);
        assert_eq!(it.collect::<std::vec::Vec<_>>(), [4, 5]);
        assert_eq!(calls.get(), 1);
    }

    #[test]
    fn lazy_next_back_calls_factory_immediately() {
        let calls = Cell::new(0);
        let mut it = (0..3).chain_if_lazy(true, || {
            calls.set(calls.get() + 1);
            3..6
        });
        assert_eq!(calls.get(), 0);
        assert_eq!(it.next_back(), Some(5));
        assert_eq!(calls.get(), 1);
        assert_eq!(it.next(), Some(0));
        assert_eq!(it.next_back(), Some(4));
        assert_eq!(calls.get(), 1);
    }

    #[test]
    fn false_factory_never_runs() {
        let calls = Cell::new(0);
        let result: std::vec::Vec<_> = (0..2)
            .chain_if_lazy(false, || {
                calls.set(calls.get() + 1);
                2..4
            })
            .collect();
        assert_eq!(result, [0, 1]);
        assert_eq!(calls.get(), 0);
    }

    #[test]
    fn if_else_lazy_accepts_different_iterator_types() {
        assert!((0..2)
            .chain_if_else_lazy(true, || 10..12, || [20, 21])
            .eq([0, 1, 10, 11]));
        assert!((0..2)
            .chain_if_else_lazy(false, || 10..12, || [20, 21])
            .eq([0, 1, 20, 21]));
    }

    #[test]
    fn only_selected_factory_runs() {
        let yes = Cell::new(0);
        let no = Cell::new(0);
        let mut it = (0..1).chain_if_else_lazy(
            true,
            || {
                yes.set(yes.get() + 1);
                1..3
            },
            || {
                no.set(no.get() + 1);
                [4, 5]
            },
        );
        assert_eq!(it.next_back(), Some(2));
        assert_eq!(yes.get(), 1);
        assert_eq!(no.get(), 0);
    }

    // Deliberately non-fused: yields Some after yielding None.
    #[derive(Clone, Debug)]
    struct Flaky(u8);

    impl Iterator for Flaky {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            let result = match self.0 {
                0 => Some(7),
                1 => None,
                2 => Some(8),
                _ => None,
            };
            self.0 += 1;
            result
        }
    }

    #[test]
    fn tails_are_fused_even_if_inner_is_not() {
        fn assert_fused<I: FusedIterator>(_: &I) {}

        let mut eager = ChainIfTail::new(true, Flaky(0));
        assert_fused(&eager);
        assert_eq!(eager.next(), Some(7));
        assert_eq!(eager.next(), None);
        assert_eq!(eager.next(), None);

        let mut lazy = ChainIfLazyTail::new(true, || Flaky(0));
        assert_fused(&lazy);
        assert_eq!(lazy.next(), Some(7));
        assert_eq!(lazy.next(), None);
        assert_eq!(lazy.next(), None);
    }

    #[test]
    fn clone_preserves_iterator_state() {
        let mut eager = (0..2).chain_if(true, 2..5);
        assert_eq!(eager.next(), Some(0));
        assert_eq!(eager.clone().collect::<std::vec::Vec<_>>(), [1, 2, 3, 4]);
        assert_eq!(eager.next(), Some(1));

        let mut lazy = (0..2).chain_if_lazy(true, || 2..5);
        assert_eq!(lazy.next_back(), Some(4));
        assert_eq!(lazy.clone().collect::<std::vec::Vec<_>>(), [0, 1, 2, 3]);
        assert_eq!(lazy.next_back(), Some(3));

        let _ = std::format!("{eager:?} {lazy:?}");
    }

    #[test]
    fn fused_and_double_ended() {
        fn assert_traits<I: DoubleEndedIterator + FusedIterator>(_: &I) {}
        let mut it = (0..2).chain_if_lazy(true, || 2..4);
        assert_traits(&it);
        assert_eq!(it.next_back(), Some(3));
        assert_eq!(it.next(), Some(0));
        assert_eq!(it.next_back(), Some(2));
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.next(), None);
        assert_eq!(it.next_back(), None);
    }
    #[test]
    fn delegated_forward_methods() {
        assert_eq!((0..2).chain_if(true, 2..5).count(), 5);
        assert_eq!((0..2).chain_if_lazy(true, || 2..5).last(), Some(4));
        #[allow(clippy::unnecessary_fold)]
        {
            assert_eq!((0..2).chain_if(true, 2..5).fold(0, |a, x| a + x), 10);
        }

        let mut it = (0..2).chain_if_lazy(true, || 2..5);
        assert_eq!(it.nth(2), Some(2));
        assert_eq!(it.find(|x| *x == 4), Some(4));

        let mut it = (0..2).chain_if(true, 2..5);
        assert_eq!(it.find_map(|x| (x == 3).then_some(x * 2)), Some(6));
        assert_eq!(it.position(|x| x == 4), Some(0));

        let mut it = (0..2).chain_if_lazy(true, || 2..5);
        assert!(it.any(|x| x == 3));
        assert!(it.all(|x| x == 4));

        let sum = Cell::new(0);
        (0..2)
            .chain_if(true, 2..5)
            .for_each(|x| sum.set(sum.get() + x));
        assert_eq!(sum.get(), 10);
    }

    #[test]
    fn delegated_reverse_methods() {
        let mut it = (0..2).chain_if(true, 2..5);
        assert_eq!(it.nth_back(1), Some(3));
        assert_eq!(it.rfind(|x| *x == 1), Some(1));

        let mut it = (0..2).chain_if_lazy(true, || 2..5);
        assert_eq!(it.nth_back(2), Some(2));
        assert_eq!(it.rfind(|x| *x == 0), Some(0));
        assert_eq!(
            (0..2)
                .chain_if_lazy(true, || 2..5)
                .rfold(0, |a, x| a * 10 + x),
            43210
        );
    }
}
