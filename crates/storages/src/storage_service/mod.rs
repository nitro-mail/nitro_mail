use std::error::Error;
use std::io;
use std::ops::DerefMut;

use futures_lite::{AsyncReadExt, AsyncWriteExt, FutureExt};

pub mod packets;
pub mod storage_service_storage;
