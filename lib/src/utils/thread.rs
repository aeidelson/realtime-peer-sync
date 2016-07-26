use std::thread;
use std::sync::mpsc;

pub struct CancelSender(mpsc::Sender<()>);
impl CancelSender {
    pub fn cancel_thread(self) {
        let CancelSender(sender) = self;
        let _ = sender.send(());
    }
}

pub struct CancelReceiver(mpsc::Receiver<()>);
impl CancelReceiver {
    pub fn has_been_canceled(&self) -> bool {
        let &CancelReceiver(ref receiver) = self;
        match receiver.try_recv() {
            Ok(_) | Err(mpsc::TryRecvError::Disconnected) => return true,
            Err(mpsc::TryRecvError::Empty) => return false,
        }

    }
}

// Wraps the standard thread::spawn, creating a channel to be used for canceling the thread.
// Note that is_canceled will automatically become true if the cancel sender is lost, so be sure to
// hold onto it!
pub fn spawn_cancelable<F>(to_execute: F) -> CancelSender
        where F: FnOnce(CancelReceiver), F: Send + 'static {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || to_execute(CancelReceiver(receiver)));
    CancelSender(sender)
}
