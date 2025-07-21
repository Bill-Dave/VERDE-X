use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    transaction::Transaction,
    system_instruction,
};
use solana_client::rpc_client::RpcClient;

pub struct SolService {
    client: RpcClient,
}

impl SolService {
    pub fn new(url: &str) -> Self {
        SolService {
            client: RpcClient::new(url.to_string()),
        }
    }

    pub fn create_escrow_ix(
        &self,
        escrow_keypair: &Keypair,
        initiator: &Pubkey,
        counterparty: &Pubkey,
        lamports: u64,
    ) -> Transaction {
        let ix = system_instruction::transfer(initiator, &escrow_keypair.pubkey(), lamports);
        Transaction::new_signed_with_payer(
            &[ix],
            Some(initiator),
            &[Keypair::from_bytes(&[0; 64]).unwrap()], // stub
            self.client.get_latest_blockhash().unwrap(),
        )
    }
}
