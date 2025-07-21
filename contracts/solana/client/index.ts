import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VerdexEscrow } from "../target/types/verdex_escrow";

export class SolClient {
  private program: Program<VerdexEscrow>;

  constructor(connection: anchor.web3.Connection, wallet: anchor.Wallet) {
    const provider = new anchor.AnchorProvider(connection, wallet, {});
    anchor.setProvider(provider);
    this.program = anchor.workspace.VerdexEscrow as Program<VerdexEscrow>;
  }

  async createEscrow(amount: number, counterparty: anchor.web3.PublicKey) {
    const escrow = anchor.web3.Keypair.generate();
    await this.program.methods
      .createEscrow(new anchor.BN(amount))
      .accounts({
        escrow: escrow.publicKey,
        initiator: this.program.provider.publicKey,
        counterparty,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([escrow])
      .rpc();
    return escrow.publicKey;
  }
}
