import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Todolist } from "../target/types/todolist";
 
import { PublicKey, SystemProgram } from "@solana/web3.js";
import {
  createAssociatedTokenAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createMint,
  getAssociatedTokenAddress,
} from "@solana/spl-token";
import { assert } from "chai";

describe("todolist", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Todolist as Program<Todolist>;

  let user = anchor.web3.Keypair.generate();
  let userState;
  before(async () => {
    const signature = await program.provider.connection.requestAirdrop(
      user.publicKey,
      5 * anchor.web3.LAMPORTS_PER_SOL // 2 SOL = 2 billion lamports
    );
    await program.provider.connection.confirmTransaction(signature);
    userState = PublicKey.findProgramAddressSync(
      [Buffer.from("user"), user.publicKey.toBuffer()],
      program.programId
    )[0];

  })
  it("initialize", async () => {
    // Add your test here.
    const tx = await program.methods.initialize(20)
    .accounts({
      user: user.publicKey,
      userState: userState,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([user])
    .rpc();
     
    const userStateData = await program.account.userState.fetch(userState);
   
    assert.equal(userStateData.user.toBase58(), user.publicKey.toBase58());
    assert.equal(userStateData.indexArray.length, 20);
  });
});
