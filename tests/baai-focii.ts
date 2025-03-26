import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BaaiToken } from "../target/types/baai_token";
import { FociiVerifier } from "../target/types/focii_verifier";
import { TOKEN_PROGRAM_ID, createMint, createAccount } from "@solana/spl-token";
import { assert } from "chai";

describe("baai-focii", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const baaiProgram = anchor.workspace.BaaiToken as Program<BaaiToken>;
  const fociiProgram = anchor.workspace.FociiVerifier as Program<FociiVerifier>;

  let mint: anchor.web3.PublicKey;
  let recipientTokenAccount: anchor.web3.PublicKey;
  let verifierPDA: anchor.web3.PublicKey;
  let verifierBump: number;

  before(async () => {
    // Find the verifier PDA
    const [verifierAddress, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("verifier")],
      fociiProgram.programId
    );
    verifierPDA = verifierAddress;
    verifierBump = bump;

    // Create the mint
    mint = await createMint(
      provider.connection,
      await provider.wallet.payer,
      provider.wallet.publicKey, // Temporary mint authority
      null,
      9 // 9 decimals
    );
  });

  it("Initialize BAAI token", async () => {
    const tx = await baaiProgram.methods
      .initialize()
      .accounts({
        mint: mint,
        fociiVerifier: verifierPDA,
        payer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .rpc();

    console.log("BAAI token initialized with tx:", tx);
  });

  it("Initialize Focii verifier", async () => {
    const tx = await fociiProgram.methods
      .initialize()
      .accounts({
        verifier: verifierPDA,
        baaiMint: mint,
        authority: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Focii verifier initialized with tx:", tx);
  });

  it("Verify and mint tokens", async () => {
    // Create a token account for the recipient
    recipientTokenAccount = await createAccount(
      provider.connection,
      await provider.wallet.payer,
      mint,
      provider.wallet.publicKey
    );

    const verificationData = Buffer.from("test data");
    const amount = new anchor.BN(1000000000); // 1 token with 9 decimals

    const tx = await fociiProgram.methods
      .verifyAndMint(verificationData, amount)
      .accounts({
        verifier: verifierPDA,
        baaiMint: mint,
        recipient: recipientTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    console.log("Tokens minted with tx:", tx);

    // Verify the balance
    const balance = await provider.connection.getTokenAccountBalance(recipientTokenAccount);
    assert.equal(balance.value.amount, "1000000000");
  });
}); 