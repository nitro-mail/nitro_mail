use std::error::Error;
use std::io;
use std::ops::DerefMut;

use bytes::BytesMut;
use futures_lite::{AsyncReadExt, AsyncWriteExt, FutureExt};
use interprocess::local_socket::tokio::{LocalSocketListener, LocalSocketStream};
use tracing::{error, info, trace};
use utils::service::Service;

use crate::directory_service::packets::{FromServicePackets, ToServicePackets};
use crate::directory_type::Directory;
use crate::SOCKET_NAME;

pub mod directory_service_directory;

pub mod packets;

pub struct DirectoryService<D: Directory + Clone> {
    directory: D,
}
impl<D: Directory + Clone> DirectoryService<D> {
    pub fn new(directory: D) -> Self {
        Self { directory }
    }

    pub async fn run(self) {
        info!("Starting directory service for {}", D::directory_name());
        let socket_result = LocalSocketListener::bind(SOCKET_NAME).expect("Failed to bind socket");

        while let Ok(request) = socket_result.accept().await {
            let directory = self.directory.clone();
            trace!("Accepted connection");
            tokio::spawn(async move {
                if let Err(error) = Self::handle_client(directory, request).await {
                    info!("Error handling client: {}", error);
                };
            });
        }
    }
    async fn write(
        connection: &mut LocalSocketStream,
        packet: FromServicePackets,
    ) -> io::Result<()> {
        let result = rkyv::to_bytes::<_, 256>(&packet);
        match result {
            Ok(ok) => {
                connection
                    .write_all(&(ok.len() as u32).to_be_bytes())
                    .await?;
                connection.write_all(&ok).await
            }
            Err(err) => {
                error!("Failed to serialize packet: {}", err);
                Ok(())
            }
        }
    }

    pub async fn handle_client(
        directory: D,
        mut stream: LocalSocketStream,
    ) -> Result<(), anyhow::Error> {
        let mut number_buffer = [0; 4];
        let mut buffer = BytesMut::new();
        loop {
            stream.read_exact(&mut number_buffer).await?;
            let packet_size = u32::from_be_bytes(number_buffer);
            buffer.resize(packet_size as usize, 0);
            trace!("Reading packet of size {}", packet_size);
            stream.read_exact(buffer.deref_mut()).await?;
            trace!("Read packet {:?}", buffer);
            let ok = match rkyv::from_bytes::<ToServicePackets>(&buffer) {
                Ok(ok) => ok,
                Err(err) => {
                    error!("Failed to deserialize packet: {}", err);
                    continue;
                }
            };
            match ok.handle::<D>(&directory).await {
                Ok(ok) => {
                    Self::write(&mut stream, ok).await?;
                }
                Err(err) => {
                    error!("Failed to handle packet: {}", err);
                    Self::handle_directory_error(&mut stream, err).await?;
                }
            }
        }
    }

    async fn handle_directory_error(
        connection: &mut LocalSocketStream,
        error: impl Error,
    ) -> io::Result<()> {
        let error = error.to_string();
        Self::write(
            connection,
            FromServicePackets::InternalDirectoryError(error.to_string()),
        )
        .await
    }
}
