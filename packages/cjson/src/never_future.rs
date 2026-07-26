enum Never {}

pub(crate) struct NeverFuture<Out>(Never, ::core::marker::PhantomData<Out>);

impl<Out> Future for NeverFuture<Out> {
    type Output = Out;

    fn poll(
        self: core::pin::Pin<&mut Self>,
        _: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        match self.0 {}
    }
}
