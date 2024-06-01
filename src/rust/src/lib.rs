use extendr_api::prelude::*;

// use bytes::Bytes;
// use std::path::{Path, PathBuf};
// use tokio::signal;
// use tokio_stream::StreamExt;

use iroh::{node::Node};
use iroh::base::node_addr::AddrInfoOptions;
use iroh::client::docs::ShareMode;
use iroh::docs::DocTicket;
use iroh::{base::base32, client::docs::Entry, docs::store::Query};
use iroh::client::docs::LiveEvent;

use tokio_stream::StreamExt;

use std::str::FromStr;

use async_std::io;
use anyhow::{anyhow, Result};

#[extendr]
fn receive(ticket_id: &str) -> Vec<u8> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            create_node_and_import(ticket_id).await.unwrap()
        })
}

#[extendr]
fn send_to_other(name: &str, x: &[u8]) {
    let vec_x = x.to_owned();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            create_node_and_export(name, vec_x).await.unwrap();
        });
}

async fn create_node_and_import(ticket: &str) -> anyhow::Result<Vec<u8>>{
    let ticket = DocTicket::from_str(ticket)?;
    let node = Node::memory().spawn().await?;

    let client = node.client();
    let (doc, mut events) = node.docs.import_and_subscribe(ticket).await?;

    while let Some(event) = events.try_next().await? {
      match event {
        LiveEvent::SyncFinished(_) => {
          // this event is emitted when the initial sync is finished
          println!("sync finished")
        }
        LiveEvent::PendingContentReady => {
          // this event is emitted when all content of the initial sync is downloaded
          println!("all content downloaded");
          break;
        }
        _ => {
          // ignore other events
        }
      }
    }

    let mut stream = doc.get_many(Query::all()).await?;
    while let Some(entry) = stream.try_next().await? {
        let content = entry.content_bytes(client).await?;
        return Ok(content.to_vec())
    }

    Err(anyhow!("Could Not Find Any Entries!!"))

}

async fn create_node_and_export(name: &str, x: Vec<u8>) -> anyhow::Result<()>{
    let node = Node::memory().spawn().await?;

    // Get client; create doc, ticket, and default author
    let client = node.client();
    let doc = client.docs.create().await?;

    let ticket = doc.share(ShareMode::Write,AddrInfoOptions::Addresses).await?;
    let author = client.authors.default().await?;

    doc.set_bytes(author, name.to_string(), x).await?;

    println!("to get access to this data, use {}", ticket);

    let mut input = String::new();
    io::stdin().read_line(&mut input).await?;
    println!("You entered: {}", input.trim());

    Ok(())
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod workwme;
    fn send_to_other;
    fn receive;
}
