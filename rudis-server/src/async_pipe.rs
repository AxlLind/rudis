use smol::channel::{Receiver, Sender};

#[derive(Clone)]
pub struct AsyncPipe<C, R> {
    tx: Sender<(C, Sender<anyhow::Result<R>>)>,
    rx: Receiver<(C, Sender<anyhow::Result<R>>)>,
}

impl<C, R> AsyncPipe<C, R> {
    pub fn new(cap: usize) -> Self {
        let (tx, rx) = smol::channel::bounded(cap);
        Self {tx, rx}
    }

    pub async fn recv(&self) -> (C, Sender<anyhow::Result<R>>) {
        self.rx.recv().await.expect("failed to read command")
    }

    pub async fn send(&self, c: C, tx: Sender<anyhow::Result<R>>) {
        self.tx.send((c, tx)).await.expect("failed to send command");
    }
}
