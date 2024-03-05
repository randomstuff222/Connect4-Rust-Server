use tokio::net::TcpListener;
use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};
use serde::{Serialize, Deserialize};
use serde_json::from_str;

use std::sync::{Arc, Mutex};


mod board_me;
mod interface_me;
mod minimax_me;

extern crate clap;

use interface_me::GameInterface;

use crate::interface_me::InterfaceObject;
use crate::minimax_me::MinimaxBotMe;

struct UI {
    current_player: usize,
    player: MinimaxBotMe,
}

const HEIGHT: usize = 6;
const WIDTH: usize = 7;
const BOARD_LENGTH: usize = HEIGHT * WIDTH;

const BOT: usize = 2;
const HUMAN: usize = 1;

#[derive(Serialize, Deserialize, Debug)]
struct ConnectMessage {
    cpu: bool,
    data: Vec<i32>,
}

type PlayFn = fn(&board_me::Board, usize) -> u8;

async fn handle_connection(
    addr: &SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {

    ws_stream   
        .send(Message::text("Welcome to chat! Type a message".to_string()))
        .await?;
    let mut bcast_rx = bcast_tx.subscribe();
    
    // A continuous loop for concurrently performing two tasks: (1) receiving
    // messages from `ws_stream` and broadcasting them, and (2) receiving
    // messages on `bcast_rx` and sending them to the client.
    loop {
        tokio::select! {
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            println!("From client {addr:?} {text:?}");
                            handle_message(&text);
                            bcast_tx.send(text.into())?;

                        }
                    }
                    Some(Err(err)) => return Err(err.into()),
                    None => return Ok(()),
                }
            }
            msg = bcast_rx.recv() => {
                // ws_stream.send(Message::text(msg?)).await?;
                ws_stream.send(Message::text(msg?.to_string())).await?;         
            }
        }
    }
}


fn handle_message(message: &str) {
    let msg: ConnectMessage = serde_json::from_str(message).unwrap();
    for client in 0..20{
        print!("piece: {} -- ", msg.data[client]);
    }
    // what we need to do: we need to take the vector array
    // and calculate the algorithm for the pieces for the cpu to play
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // let cpu_board = board_me::Board::new(); 
    // let dummy =  UI {
    //     current_player: 2,
    //     player: MinimaxBotMe {},
    // };


    //let my_play = |cpu_board, &dummy| dummy.player.play(cpu_board, BOT);

    let (bcast_tx, _) = channel(16);

    let listener = TcpListener::bind("127.0.0.1:3001").await?;
    println!("listening on port 3001");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move{
            // Wrap the raw TCP stream into a websocket.
            let ws_stream = ServerBuilder::new().accept(socket).await?;

            handle_connection(&addr, ws_stream, bcast_tx).await
        });
    }
}