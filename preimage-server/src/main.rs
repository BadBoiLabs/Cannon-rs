use anyhow::Result;
use clap::Parser;
use log::debug;
use preimage_provider::PreimageProvider;
use std::collections::HashMap;
use std::os::fd::FromRawFd;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

mod cli;
mod preimage_provider;

// preimage file descriptors
const PCLIENT_RFD: i32 = 5;
const PCLIENT_WFD: i32 = 6;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    env_logger::init();
    let args = cli::Cli::parse();

    // check if args.path is a directory
    let preimages = if args.path.is_dir() {
        preimage_from_dir(args.path)
    } else {
        // else assume it is a json file
        let json_str = std::fs::read_to_string(args.path).expect("Unable to read preimage file");
        preimage_from_json_str(&json_str)
    };

    let reader = unsafe { File::from_raw_fd(PCLIENT_RFD) };
    let writer = unsafe { File::from_raw_fd(PCLIENT_WFD) };

    wait_for_requests(reader, writer, preimages).await?;

    Ok(())
}

/// Load json string into a preimage HashMap.
///
/// # Panics
/// This will panic if:
///     - the json string is not valid json
///     - if they key is not valid hex or not 32 bytes
///     - if the value is not valid hex
fn preimage_from_json_str(json: &str) -> HashMap<[u8; 32], Vec<u8>> {
    let json: HashMap<String, String> = serde_json::from_str(&json).expect("Unable to parse");

    let mut preimages = HashMap::<[u8; 32], Vec<u8>>::new();
    for (k, v) in json.iter() {
        let k = hex::decode(k).expect("Unable to decode key");
        assert!(k.len() == 32, "Key must be 32 bytes");
        let mut key = [0; 32];
        key.copy_from_slice(&k);
        let v = hex::decode(v).expect("Unable to decode value");
        preimages.insert(key, v);
    }
    debug!("Loaded {} preimages from file", preimages.len());

    preimages
}

/// Load a directory full of files named with their preimage key
///
/// # Panics
/// This will panic if:
///     - any file name is not valid hex or not 32 bytes
fn preimage_from_dir(dir: std::path::PathBuf) -> HashMap<[u8; 32], Vec<u8>> {
    let mut preimages = HashMap::<[u8; 32], Vec<u8>>::new();
    for file in std::fs::read_dir(dir).expect("Unable to read directory") {
        let file = file.expect("Unable to read file");
        let fname = file.file_name();
        let k = hex::decode(fname.to_str().unwrap()).expect("invalid hex in filename");
        let mut key = [0; 32];
        key.copy_from_slice(&k);

        let value = std::fs::read(file.path()).expect("Unable to read file");
        preimages.insert(key, value);
    }
    debug!("Loaded {} preimages from directory", preimages.len());
    preimages
}

/// Infinitely wait for new requests to be forwarded from the emulator on the reader channel
/// On each received request try and retrieve a pre-image and send it to the guest on the writer channel
async fn wait_for_requests(
    mut reader: File,
    mut writer: File,
    preimages: impl PreimageProvider,
) -> Result<()> {
    loop {
        let mut key_buffer = [0; 32];
        reader.read(&mut key_buffer).await?;
        debug!("Received key bytes: {:?}", &key_buffer);

        if let Some(data) = preimages.get(&key_buffer) {
            // first it needs to write the length as a u64 big-endian
            let length: u64 = data.len() as u64;
            writer.write(&length.to_be_bytes()).await?;

            // then write the actual data
            writer.write(&data).await?;
        } else {
            panic!("Guest requested preimage that does not exist")
        }
    }
}
