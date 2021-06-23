use actix_web::web::Bytes;
use actix_web::{rt, Error};
use futures::Stream;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use std::sync::Arc;
use tokio::sync::mpsc::{error::SendError, unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::time::{interval_at, Instant};
use moonlight::SessionId;

pub type ShareableSSE = Arc<Mutex<SSE>>;

// ------ Connection ------

#[derive(Clone)]
pub struct Connection {
    session_id: SessionId,
    sender: UnboundedSender<Bytes>,
}

impl Connection {
    fn new(session_id: Option<SessionId>) -> (Connection, EventStream) {
        let (sender, receiver) = unbounded_channel();
        let connection = Self {
            session_id: session_id.unwrap_or_else(SessionId::new),
            sender,
        };
        (connection, EventStream(receiver))
    }

    fn session_id(&self) -> SessionId {
        self.session_id
    }

    pub fn send(&self, event: &str, data: &str) -> Result<(), SendError<Bytes>> {
        let message = Bytes::from(["event: ", event, "\n", "data: ", data, "\n\n"].concat());
        self.sender.send(message)
    }
}

// ------ EventStream ------

pub struct EventStream(UnboundedReceiver<Bytes>);

impl Stream for EventStream {
    type Item = Result<Bytes, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.0).poll_recv(cx) {
            Poll::Ready(Some(bytes)) => Poll::Ready(Some(Ok(bytes))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

// ------ SSE ------

pub struct SSE {
    connections: HashMap<SessionId, Connection>,
}

impl SSE {
    pub fn start() -> ShareableSSE {
        let sse = SSE {
            connections: HashMap::new(),
        };
        let this = Arc::new(Mutex::new(sse));
        this.spawn_connection_remover();
        this
    }
}

// ------ ShareableSSEMethods ------

pub trait ShareableSSEMethods {
    fn spawn_connection_remover(&self);

    fn new_connection(&self, session_id: Option<SessionId>) -> (Connection, EventStream);

    fn broadcast(&self, event: &str, data: &str) -> Result<(), Vec<SendError<Bytes>>>;
}

impl ShareableSSEMethods for ShareableSSE {
    fn spawn_connection_remover(&self) {
        let this = self.clone();
        rt::spawn(async move {
            let mut interval = interval_at(Instant::now(), Duration::from_secs(10));
            loop {
                interval.tick().await;
                this.lock()
                    .connections
                    .retain(|_, connection| connection.send("ping", "").is_ok());
            }
        });
    }

    fn new_connection(&self, session_id: Option<SessionId>) -> (Connection, EventStream) {
        let (connection, event_stream) = Connection::new(session_id);
        self.lock()
            .connections
            .insert(connection.session_id(), connection.clone());
        (connection, event_stream)
    }

    fn broadcast(&self, event: &str, data: &str) -> Result<(), Vec<SendError<Bytes>>> {
        let errors = self
            .lock()
            .connections
            .values()
            .filter_map(|connection| connection.send(event, data).err())
            .collect::<Vec<_>>();

        if errors.is_empty() {
            return Ok(());
        }
        Err(errors)
    }
}
