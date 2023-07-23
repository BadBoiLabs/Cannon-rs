use crate::syscalls::{self, SyscallError};
use alloc::vec::Vec;

pub use super::PreimageKey;

#[derive(Debug)]
pub enum OracleError {
    NoKeySet,
    EndOfData,
    SyscallError(syscalls::SyscallError),
}

impl From<SyscallError> for OracleError {
    fn from(e: SyscallError) -> Self {
        OracleError::SyscallError(e)
    }
}

pub struct OracleReader {
    key: Option<PreimageKey>,
    length: u64,
    cursor: u64,
}

// the only way to access an oracle reader is through this singleton.
// This is to ensure there cannot be more than one at a time which would have
// unpredictable results
static mut ORACLE_READER: Option<OracleReader> = Some(OracleReader {
    key: None,
    length: 0,
    cursor: 0,
});

/// Get the global oracle reader
///
/// # Panics
/// This will panic if called more than once. This is to ensure there is only one oracle reader at once
/// as it encapsulates host global state.
pub fn oracle_reader() -> OracleReader {
    unsafe {
        let reader = core::ptr::replace(&mut ORACLE_READER, None);
        reader.expect("oracle_reader` has already been called. Can only call once per program")
    }
}

impl OracleReader {
    /// Set the preimage key for the global oracle reader. This will overwrite any existing key
    ///
    /// Internally this sends the 32 bytes of the key to the host by writing into the WritePreimage file descriptor.
    /// This may require several writes as the host may only accept a few bytes at a time. Once 32 bytes have been written
    /// successfully the key is considered set. If it fails to write 32 bytes it will return an error.
    /// Once it has written the key it will read the first 8 bytes of the ReadPreimage file descriptor which is the length
    /// encoded as a big endian u64. This is stored in the oracle reader along with the read cursor position.
    ///
    /// # Examples
    /// ```
    /// use cannon_io::prelude::*;
    ///
    /// let mut oracle = oracle_reader();
    /// oracle.set_key(PreimageKey::new_local(&[0xff;31]));
    /// ```
    pub fn set_key(&mut self, key: PreimageKey) -> Result<(), OracleError> {
        self.key = Some(key);
        let key_bytes: [u8; 32] = key.into();
        let mut written = 0;
        // need to loop and write the bytes a chunk at a time until all are written
        loop {
            match syscalls::write_preimage(&key_bytes[written..]) {
                Ok(0) => break,
                Ok(n) => {
                    written += n as usize;
                    continue;
                }
                Err(e) => return Err(e.into()),
            }
        }

        // first read the length prefix, cache and reset the cursor
        let mut length_buffer = [0_u8; 8];
        self.read_exact(&mut length_buffer)?;
        self.length = u64::from_be_bytes(length_buffer);
        self.cursor = 0;
        Ok(())
    }

    /// Return the current key stored in the global oracle reader
    pub fn key(&self) -> Option<PreimageKey> {
        self.key
    }

    /// length of the current pre-image
    pub fn length(&self) -> u64 {
        self.length
    }

    /// Current position of the read cursor within the current pre-image
    pub fn cursor(&self) -> u64 {
        self.cursor
    }

    /// Get the data corresponding to the currently set key from the host. Return the data in a new heap allocated `Vec<u8>`
    ///
    /// Internally this reads self.length bytes from the ReadPreimage file descriptor into a new heap allocated `Vec<u8>` and returns it.
    /// This is a high level way to interact with the preimage oracle but may not be the best way if heap allocations are not desirable.
    ///
    /// # Examples
    /// ```
    /// use cannon_io::prelude::*;
    ///
    /// let mut oracle = oracle_reader();
    /// let key = PreimageKey::new_local(&[0xff;31]);
    /// let data = oracle.get(key).unwrap();
    /// ```
    pub fn get(&mut self, key: PreimageKey) -> Result<Vec<u8>, OracleError> {
        self.set_key(key)?;
        let mut data_buffer = Vec::with_capacity(self.length as usize);
        data_buffer.resize(self.length as usize, 0);
        self.read_exact(&mut data_buffer)?;
        Ok(data_buffer)
    }

    /// Get the data corresponding to the currently set key from the host. Write the data into the provided buffer
    ///
    /// # Panics
    /// This will panic if the size of the buffer is not equal to the size of the preimage as reported by the host
    ///
    /// # Examples
    /// ```
    /// use cannon_io::prelude::*;
    ///
    /// let mut oracle = oracle_reader();
    /// let key = PreimageKey::new_local(&[0xff;31]);
    /// let mut buffer = [0_u8; 100];
    /// oracle.get_exact(key, &mut buffer).unwrap();
    /// ```
    pub fn get_exact(&mut self, key: PreimageKey, buf: &mut [u8]) -> Result<(), OracleError> {
        self.set_key(key)?;
        assert!(self.length as usize == buf.len(), "Buffer not correct size for preimage data. Preimage size: {} bytes, buffer size: {} bytes", self.length, buf.len());
        self.read_exact(buf)?;
        Ok(())
    }
}

// Since the Rust Error trait cannot be used in no_std, we define our own
// trait that is very similar and should feel familiar
pub trait Read {
    type Error;

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error>;

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, Self::Error>;

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Self::Error>;
}

impl Read for OracleReader {
    type Error = OracleError;

    /// Read up to buf.len() bytes from the ReadPreimage file descriptor into buf
    ///
    /// This returns the number of bytes read. If the end of the data is reached it will return 0.
    /// Subsequent calls can be used to write the rest of the data as the host may only accept arbitrary small
    /// chunks at a time.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.key.ok_or(Self::Error::NoKeySet)?;
        let read = syscalls::read_preimage(buf)?;
        self.cursor += read as u64;
        Ok(read as usize)
    }

    /// Read all the data from the ReadPreimage file descriptor into buf
    ///
    /// This will read all the data from the ReadPreimage file descriptor into buf. This is implemented by calling
    /// read() in a loop until it returns 0. This will read at most 32 bytes per call to read.
    /// New space will be allocated into the return buffer if it fills up.
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, Self::Error> {
        let mut chunk = [0; 32];
        loop {
            let read = self.read(&mut chunk)?;
            if read == 0 {
                break;
            }
            buf.extend_from_slice(&chunk[..read]);
        }
        Ok(buf.len())
    }

    /// Read exactly buf.len() bytes from the ReadPreimage file descriptor into buf
    ///
    /// This will read exactly buf.len() bytes from the ReadPreimage file descriptor into buf. This is implemented by calling
    /// read() in a loop until it returns 0. This will read at most 32 bytes per call to read.
    /// If the end of the data is reached before buf.len() bytes have been read it will return an error.
    /// After this function returns there may be more bytes in the stream to be read.
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        let mut chunk = [0; 32];
        let mut read = 0;
        while read < buf.len() {
            let chunk_read = self.read(&mut chunk)?;
            if chunk_read == 0 {
                return Err(OracleError::EndOfData);
            }
            buf[read..read + chunk_read].copy_from_slice(&chunk[..chunk_read]);
            read += chunk_read;
        }
        Ok(())
    }
}
