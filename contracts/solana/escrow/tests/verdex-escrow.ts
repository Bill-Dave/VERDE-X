import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VerdexEscrow } from "../target/types/verdex_escrow";
import { expect } from "chai";

describe("verdex-escrow", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.VerdexEscrow as Program<VerdexEscrow>;

  it("Creates escrow", async () => {
    const escrowKeypair = anchor.web3.Keypair.generate();
    const initiator = (program.provider as any).wallet;
    const counterparty = anchor.web3.Keypair.generate().publicKey;
    const amount = new anchor.BN(1_000_000);

    await program.methods
      .createEscrow(amount)
      .accounts({
        escrow: escrowKeypair.publicKey,
        initiator: initiator.publicKey,
        counterparty,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([escrowKeypair])
      .rpc();

    const escrow = await program.account.escrowAccount.fetch(
      escrowKeypair.publicKey
    );
    expect(escrow.amount.toNumber()).to.equal(1_000_000);
  });
});
