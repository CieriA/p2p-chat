use {
    std::{
        io::BufRead,
        sync::Arc,
    },
    tokio::{
        io::{
            AsyncBufReadExt,
            AsyncWriteExt,
            BufReader,
        },
        net::tcp::{OwnedReadHalf, OwnedWriteHalf},
        sync::{
            Mutex,
            mpsc::Sender
        },
    },
    crate::message::Message,
};

pub async fn handle_connection(reader: OwnedReadHalf) {
    let reader = BufReader::new(reader);
    let mut lines = reader.lines();
    while let Ok(Some(line)) = lines.next_line().await {
        let message: Message = serde_json::from_str(&line).unwrap();
        match message {
            Message { id, text } => {
                println!("{id}: {text}");
            }
        }
    }
}

pub async fn input_loop(tx: Sender<String>) {
    let mut reader = std::io::stdin().lock();
    let mut line = String::new();
    loop {
        line.clear();
        if reader.read_line(&mut line).unwrap() == 0 {
            break;
        }
        let line = line.trim_end().to_owned();
        tx.send(line).await.unwrap();
    }
}

pub async fn send_message(writers: &Arc<Mutex<Vec<OwnedWriteHalf>>>, message: &Message) {
    let mut serialized = serde_json::to_string(message).unwrap();
    serialized.push('\n');
    let mut writers = writers.lock().await;
    for writer in writers.iter_mut() {
        if let Err(e) = writer.write_all(serialized.as_bytes()).await {
            eprintln!("{e}");
        }
    }
}
