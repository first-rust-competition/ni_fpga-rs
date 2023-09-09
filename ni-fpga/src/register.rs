use std::{borrow::Borrow, marker::PhantomData};

use crate::{Datatype, Error, Offset, SessionAccess};

#[derive(Clone, Copy)]
pub struct ConstOffset<const N: Offset>;

impl<const N: Offset> From<ConstOffset<N>> for Offset {
    #[inline]
    fn from(_value: ConstOffset<N>) -> Self {
        N
    }
}

#[derive(Clone, Copy)]
pub struct StoredOffset(Offset);

impl From<StoredOffset> for Offset {
    #[inline]
    fn from(value: StoredOffset) -> Self {
        value.0
    }
}

pub struct Register<T, O> {
    _offset_type: O,
    _type: PhantomData<T>,
}

impl<T> Register<T, StoredOffset> {
    #[inline]
    pub fn new(offset: Offset) -> Self {
        Self {
            _offset_type: StoredOffset(offset),
            _type: PhantomData,
        }
    }
}

impl<T, const N: Offset> Register<T, ConstOffset<N>> {
    #[inline]
    pub fn new_const() -> Self {
        Self {
            _offset_type: ConstOffset,
            _type: PhantomData,
        }
    }
}

impl<T, const N: Offset> From<Register<T, ConstOffset<N>>> for Register<T, StoredOffset> {
    #[inline]
    fn from(_: Register<T, ConstOffset<N>>) -> Self {
        Self {
            _offset_type: StoredOffset(N),
            _type: PhantomData,
        }
    }
}

pub trait RegisterReadAccess<T>
where
    T: Datatype,
{
    fn offset_read(&self) -> Offset;

    #[inline(never)]
    fn read(&self, session: &impl SessionAccess) -> Result<T, Error> {
        T::read(session, self.offset_read())
    }
}

pub trait RegisterWriteAccess<T>
where
    T: Datatype,
{
    fn offset_write(&self) -> Offset;

    #[inline]
    unsafe fn write(
        &mut self,
        session: &impl SessionAccess,
        value: impl Borrow<T>,
    ) -> Result<(), Error> {
        T::write(session, self.offset_write(), value)
    }
}

impl<T, U> RegisterReadAccess<T> for Register<T, U>
where
    T: Datatype,
    Offset: From<U>,
    U: Copy,
{
    #[inline]
    fn offset_read(&self) -> Offset {
        self._offset_type.into()
    }
}

impl<T, U> RegisterWriteAccess<T> for Register<T, U>
where
    T: Datatype,
    Offset: From<U>,
    U: Copy,
{
    #[inline]
    fn offset_write(&self) -> Offset {
        self._offset_type.into()
    }
}
