use bitcoin::{
    Address, Network, PrivateKey, PublicKey,
    psbt::PartiallySignedTransaction,
    transaction::Transaction,
};
use bitcoincore_rpc::{Auth, Client, RpcApi};

pub struct BtcService {
    client: Client,
}

impl BtcService {
    pub fn new(url: &str, user: &str, pass: &str) -> Self {
        let client = Client::new(
            url,
            Auth::UserPass(user.to_string(), pass.to_string()),
        )
        .expect("bitcoin rpc");
        BtcService { client }
    }

    pub fn create_escrow_psbt(
        &self,
        amount_sat: u64,
        initiator_sk: PrivateKey,
        counterparty_pk: PublicKey,
    ) -> PartiallySignedTransaction {
        let address = Address::p2wpkh(
            &initiator_sk.public_key(&bitcoin::secp256k1::Secp256k1::new()),
            Network::Bitcoin,
        )
        .unwrap();
        let utxos = self.client.list_unspent(None, None, None, None, None).unwrap();
        let mut tx = Transaction {
            version: 2,
            lock_time: bitcoin::absolute::LockTime::ZERO,
            input: vec![],
            output: vec![],
        };
        // TODO: complete PSBT build
        PartiallySignedTransaction::from_unsigned_tx(tx).unwrap()
    }
}
