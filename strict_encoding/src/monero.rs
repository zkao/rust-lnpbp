use super::{Error, Strategy, StrictDecode, StrictEncode};
use std::io;

pub struct MoneroConsensus;

impl Strategy for monero::PublicKey {
    type Strategy = MoneroConsensus;
}

impl Strategy for monero::PrivateKey {
    type Strategy = MoneroConsensus;
}

impl<T> StrictEncode for amplify::Holder<T, MoneroConsensus>
where
    T: monero::consensus::Encodable,
{
    #[inline]
    fn strict_encode<E: io::Write>(&self, mut e: E) -> Result<usize, Error> {
        self.as_inner()
            .consensus_encode(&mut e)
            .map_err(Error::from)
    }
}

impl<T> StrictDecode for amplify::Holder<T, MoneroConsensus>
where
    T: monero::consensus::Decodable,
{
    #[inline]
    fn strict_decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
        Ok(Self::new(T::consensus_decode(&mut d).map_err(Error::from)?))
    }
}

impl From<monero::consensus::encode::Error> for Error {
    #[inline]
    fn from(e: monero::consensus::encode::Error) -> Self {
        Error::DataIntegrityError(e.to_string())
    }
}
