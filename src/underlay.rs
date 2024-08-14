use std::sync::Arc;

use opendal::raw::*;
use opendal::*;

pub struct UnderlayLayer<L: Access + Clone> {
    lower: L,
}

impl<L: Access + Clone> UnderlayLayer<L> {
    pub fn new(lower: L) -> Self {
        Self { lower }
    }
}

impl<U: Access, L: Access + Clone> Layer<U> for UnderlayLayer<L> {
    type LayeredAccess = UnderlayLayerAccessor<U, L>;

    fn layer(&self, upper: U) -> Self::LayeredAccess {
        UnderlayLayerAccessor {
            upper,
            lower: self.lower.clone(),
        }
    }
}

#[derive(Debug)]
pub struct UnderlayLayerAccessor<U: Access, L: Access> {
    upper: U,
    lower: L,
}

impl<U: Access, L: Access> LayeredAccess for UnderlayLayerAccessor<U, L> {
    type Inner = U;
    type Reader = UnderlayLayerReader<U::Reader, L::Reader>;
    type BlockingReader = UnderlayLayerReader<U::BlockingReader, L::BlockingReader>;
    type Writer = U::Writer;
    type BlockingWriter = U::BlockingWriter;
    type Lister = U::Lister;
    type BlockingLister = U::BlockingLister;

    fn inner(&self) -> &Self::Inner {
        &self.upper
    }

    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
        todo!()
    }

    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
        todo!()
    }

    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
        todo!()
    }

    fn blocking_read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::BlockingReader)> {
        todo!()
    }

    fn blocking_write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::BlockingWriter)> {
        todo!()
    }

    fn blocking_list(&self, path: &str, args: OpList) -> Result<(RpList, Self::BlockingLister)> {
        todo!()
    }
}

pub enum UnderlayLayerReader<U, L> {
    Upper(U),
    Inner(L),
}

impl<U: oio::Read, L: oio::Read> oio::Read for UnderlayLayerReader<U, L> {
    async fn read(&mut self) -> Result<Buffer> {
        unimplemented!("UnderlayLayerReader::read")
    }
}

impl<U: oio::BlockingRead, L: oio::BlockingRead> oio::BlockingRead for UnderlayLayerReader<U, L> {
    fn read(&mut self) -> Result<Buffer> {
        unimplemented!("UnderlayLayerReader::read")
    }
}
