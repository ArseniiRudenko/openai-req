use serde::ser::StdError;
use async_trait::async_trait;


#[async_trait]
pub trait AsyncTryFrom<T>: Sized {

    type Error: 'static+StdError+Send+Sync;

    async fn try_from(value: T) -> Result<Self, Self::Error>;
}

#[async_trait]
pub trait AsyncTryInto<T>: Sized {

    type Error: 'static+StdError+Send+Sync;

    async fn try_into(self) -> Result<T, Self::Error>;
}

#[async_trait]
impl<T, U> AsyncTryInto<U> for T
    where
        U: AsyncTryFrom<T>,
        T: Send
{
    type Error = U::Error;

    async fn try_into(self) -> Result<U, Self::Error>{
        U::try_from(self).await
    }
}