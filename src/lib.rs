#![allow(unstable)]

/// Evidence of a proposition where the witness has been forgotten
pub struct Squash<P: ?Sized>(());

impl<P: ?Sized> Squash<P> {
    /// Construct a `Squash` given a reference to a witness
    #[inline]
    pub fn new(_witness: &P) -> Squash<P> { Squash(()) }
}

/// Canonical proof term for type equality
struct Refl<A>;

/// The `Term` trait classifies proof terms for type equality
trait IdTerm<A, B> {}
impl<A> IdTerm<A, A> for Refl<A> {}

/// Evidence of type equality where the proof term is existentially quantified
pub struct Id<A, B>(Box<IdTerm<A, B> + 'static>);

impl<A> Id<A, A> {
    /// Construct a proof that type `A` is equal to itself
    #[inline]
    pub fn refl() -> Id<A, A> { Id(Box::new(Refl)) }
}

impl<A, B> Id<A, B> {
    /// Construct a `Squash` from a type equality proof.
    #[inline]
    pub fn squash(&self) -> Squash<Id<A, B>> { Squash::new(self) }
}

/// The `Eq` trait acts like a type equality predicate
pub trait Eq<A> {
    /// On demand, provide evidence of the truth of `Eq<A>` in terms
    /// of provable type-equality of `A` and `Self`. The obligation to
    /// define this method keeps the trait from being implemented in
    /// other crates in violation of the intended semantics.
    ///
    /// The method is called completeness because it characterizes the
    /// following notion:
    ///
    /// `Self: Eq<A> ==> exists p: Squash<Id<Self, A>>`
    ///
    /// Implication in the reverse direction would be something like
    /// soundness but that isn't very useful here.
    #[inline]
    fn completeness(&self) -> Squash<Id<Self, A>>;

    /// Given `X: Eq<Y>` and `x: X`, this method will safely coerce
    /// `x` to type `Y`. The safety comes from the fact that the only
    /// time the bound `X: Eq<Y>` holds is when `X` and `Y` are the
    /// same type (determined statically).
    #[inline]
    fn coerce(self) -> A where Self: Sized {
        * unsafe { ::std::mem::transmute::<_, Box<_>>(Box::new(self)) }
    }
}

impl<A> Eq<A> for A {
    #[inline]
    fn completeness(&self) -> Squash<Id<A, A>> { Id::refl().squash() }
}

/// Bi-directional type-level equality constraint
pub trait BiEq<A: Eq<B>, B: Eq<A>> {}
impl<A: Eq<B>, B: Eq<A>, C> BiEq<A, B> for C {}

#[cfg(test)]
mod tests {
    use super::{
        Eq,
    };

    #[test]
    fn coerce() {
        fn aux<X, Y: Eq<X>>(y: Y) -> X {
            y.coerce()
        }
        assert_eq!((), aux::<(), ()>(()))
    }

    // // FIXME: Need compile-time #[should_fail]
    // #[test]
    // #[should_fail]
    // fn equality_coerce_fail() {
    //     fn aux<X, Y: Eq<X>>(y: Y) -> X {
    //         y.coerce()
    //     }
    //     aux::<(), bool>(())
    // }
}
