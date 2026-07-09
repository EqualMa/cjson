use core::marker::PhantomData;

use super::{HasConstCompileTimeChunk, HasConstState, State};

pub enum Init {}

impl HasConstState for Init {
    const STATE: State = State::INIT;
}

enum Never {}
pub struct NextStateOf<T: ?Sized + HasConstCompileTimeChunk>(Never, PhantomData<T>);

impl<T: ?Sized + HasConstCompileTimeChunk> HasConstState for NextStateOf<T> {
    const STATE: State = T::CHUNK.into_next_state();
}

pub struct ThenValue<T: ?Sized + HasConstState>(Never, PhantomData<T>);

impl<T: ?Sized + HasConstState> HasConstState for ThenValue<T> {
    const STATE: State = T::STATE.json_value();
}

pub struct ThenItemsAfterItem<T: ?Sized + HasConstState>(Never, PhantomData<T>);
impl<T: ?Sized + HasConstState> HasConstState for ThenItemsAfterItem<T> {
    const STATE: State = T::STATE.json_items_after_item();
}

pub struct ThenItemsAfterArrayStartBeforeItem<T: ?Sized + HasConstState>(Never, PhantomData<T>);
impl<T: ?Sized + HasConstState> HasConstState for ThenItemsAfterArrayStartBeforeItem<T> {
    const STATE: State = T::STATE.json_items_after_array_start_before_item();
}
