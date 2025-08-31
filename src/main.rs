mod args;
mod connection;
mod message;
mod ticket;

use {
    args::{Args, Command},
    clap::Parser,
    connection::{handle_connection, input_loop, send_message},
    message::Message,
    std::{error::Error, sync::Arc},
    ticket::Ticket,
    tokio::{
        net::{TcpListener, TcpStream},
        sync::Mutex,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let Args { command } = Args::parse();
    let (id, other_peers) = match command {
        Command::Open { .. } => {
            let topic: String = hex::encode(rand::random::<[u8; 16]>());
            (topic, Vec::new())
        }
        Command::Join { ticket, .. } => {
            let ticket: Ticket = ticket.parse()?;
            (ticket.id, ticket.peers)
        }
    };

    let listener = TcpListener::bind("0.0.0.0:0").await?;
    let address = listener.local_addr()?;
    let peer_id = rand::random::<u64>().to_string();

    let mut peers = other_peers.clone();
    peers.push(address);
    let ticket = Ticket::new(id, peers);

    println!("Listening on: {address}");
    println!("Ticket to join us: {ticket}");

    let writers = Arc::new(Mutex::new(Vec::new()));
    let writers_clone = writers.clone();

    // Handling incoming connections
    tokio::spawn(async move {
        loop {
            match listener.accept().await {
                Ok((stream, ..)) => {
                    let (reader, writer) = stream.into_split();
                    writers_clone.lock().await.push(writer);
                    tokio::spawn(handle_connection(reader));
                }
                Err(e) => {
                    eprintln!("Error accepting the connection: {e}");
                    break;
                }
            }
        }
    });

    for peer in other_peers.iter() {
        match TcpStream::connect(peer).await {
            Ok(stream) => {
                let (reader, writer) = stream.into_split();
                writers.lock().await.push(writer);
                tokio::spawn(handle_connection(reader));
            }
            Err(e) => eprintln!("Error connecting to peer {peer}: {e}"),
        }
    }

    // input
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);
    tokio::task::spawn_blocking(move || {
        tokio::runtime::Handle::current().block_on(input_loop(tx));
    });

    while let Some(text) = rx.recv().await {
        let message = Message::Message {
            id: peer_id.clone(),
            text,
        };
        send_message(&Arc::clone(&writers), &message).await;
    }

    Ok(())
}
