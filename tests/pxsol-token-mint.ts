import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PxsolTokenMint } from "../target/types/pxsol_token_mint";
import { 
  TOKEN_2022_PROGRAM_ID,
  getMint,
  getAssociatedTokenAddress,
  getAccount,
} from "@solana/spl-token";

describe("pxsol-token-mint", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.PxsolTokenMint as Program<PxsolTokenMint>;
  const [mint, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("mint")],
    program.programId
  );
  const [token, tokenBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("token")],
    program.programId,
  );

  it("Create a mint account", async () => {
    const tx = await program.methods
      .createMint()
      .accounts({
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .rpc({ commitment: "confirmed" })
    console.log("Your transaction signature:\n", tx);

    const mintAccount = await getMint(
      program.provider.connection,
      mint,
      "confirmed",
      TOKEN_2022_PROGRAM_ID
    );

    console.log("Mint account :\n", mintAccount);
  });

  it("Create an associated token account", async () => {
    const tx = await program.methods
      .createTokenAccount()
      .accounts({
        mint: mint,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .rpc({ commitment: "confirmed" })
    console.log("Your transaction signature:\n", tx);

    const associatedTokenAccount = await getAssociatedTokenAddress(
      mint,
      program.provider.publicKey,
      false,
      TOKEN_2022_PROGRAM_ID
    );

    const tokenAccount = await getAccount(
      program.provider.connection,
      associatedTokenAccount,
      "confirmed",
      TOKEN_2022_PROGRAM_ID
    );

    console.log("Token account :\n", tokenAccount);
  });

  it("Mint new tokens", async () => {
    const tx = await program.methods
      .mintTokens(new anchor.BN(100000000))
      .accounts({
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .rpc({ commitment: "confirmed" })
    console.log("Your transaction signature:\n", tx);

    const associatedTokenAccount = await getAssociatedTokenAddress(
      mint,
      program.provider.publicKey,
      false,
      TOKEN_2022_PROGRAM_ID
    );

    const tokenAccount = await getAccount(
      program.provider.connection,
      associatedTokenAccount,
      "confirmed",
      TOKEN_2022_PROGRAM_ID
    );

    console.log("Tokens minted to account :\n", tokenAccount);
  });
});
