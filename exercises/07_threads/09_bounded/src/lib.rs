// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{sync_channel, Receiver, RecvError, Sender, SyncSender, TrySendError};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn new(sender: SyncSender<Command>) -> Self {
        TicketStoreClient { sender }
    }

    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, RecvError> {
        let (res_sender, res_receiver) = sync_channel(10);
        let command = Command::Insert { draft: draft, response_channel: res_sender };
        self.sender.send(command).unwrap();
        res_receiver.recv()
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, RecvError> {
        let (res_sender, res_receiver) = sync_channel(10);
        let command = Command::Get { id: id, response_channel: res_sender };
        self.sender.send(command).unwrap();
        res_receiver.recv()
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient::new(sender)
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Option<Ticket>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                response_channel.send(id).unwrap();
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                response_channel.send(ticket.cloned()).unwrap();
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
