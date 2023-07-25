use futures::ready;
use std::{
    io::Result,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::io::{AsyncRead, ReadBuf};

// an adapter that lets you peek/snoop on the data as it is being streamed
pub struct TeeReader<R: AsyncRead + Unpin, F: FnMut(&[u8])> {
    reader: R,
    f: F,
}

impl<R: AsyncRead + Unpin, F: FnMut(&[u8])> Unpin for TeeReader<R, F> {}

impl<R: AsyncRead + Unpin, F: FnMut(&[u8])> TeeReader<R, F> {
    /// Returns a TeeReader which can be used as AsyncRead whose
    /// reads forwards onto the supplied reader, but performs a supplied closure
    /// on the content of that buffer at every moment of the read
    pub fn new(reader: R, f: F) -> TeeReader<R, F> {
        TeeReader { reader: reader, f }
    }

    // / Consumes the `TeeReader`, returning the wrapped reader
    // pub fn into_inner(self) -> R {
    //     self.reader
    // }
}

impl<R: AsyncRead + Unpin, F: FnMut(&[u8])> AsyncRead for TeeReader<R, F> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<()>> {
        ready!(Pin::new(&mut self.reader).poll_read(cx, buf))?;
        (self.f)(&buf.filled());
        Poll::Ready(Ok(()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::AsyncReadExt;

    #[test]
    fn tee() {
        let mut reader = "It's over 9000!".as_bytes();
        let mut altout: Vec<u8> = Vec::new();
        let mut teeout = Vec::new();
        {
            let mut tee = TeeReader::new(&mut reader, |bytes| altout.extend(bytes));
            let _ = tee.read_to_end(&mut teeout);
        }
        assert_eq!(teeout, altout);
    }
}
