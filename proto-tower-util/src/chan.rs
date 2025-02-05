use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

pub fn sx_rx_chans<I, O>() -> ((Receiver<I>, Sender<O>), (Receiver<O>, Sender<I>)) {
    let (sx1, rx1) = mpsc::channel(1);
    let (sx2, rx2) = mpsc::channel(1);
    ((rx2, sx1), (rx1, sx2))
}
