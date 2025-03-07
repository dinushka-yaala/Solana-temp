import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PxsolTokenMint } from "../target/types/pxsol_token_mint";
import { 
  TOKEN_2022_PROGRAM_ID,
  getMint,
  getAssociatedTokenAddress,
  getAccount,
} from "@solana/spl-token";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";

describe("pxsol-token-mint", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.PxsolTokenMint as Program<PxsolTokenMint>;

  let authority = provider.wallet.publicKey;
  let mintAccounts: { [key: string]: PublicKey } = {};
  const seeds = ["pxSOL", "apxSOL", "upxSOL", "ipxSOL", "iapxSOL", "iupxSOL"];
  const tokenNames = ["Pxsol", "Apxsol", "Upxsol", "Ipxsol", "Iapxsol", "Iupxsol"];

  before(async () => {
    console.log("Program ID:", program.programId.toString());
    // Generate mint accounts
    seeds.forEach((seed, index) => {
      const [accountKey, _] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from(seed)], program.programId);
      mintAccounts[tokenNames[index]] = accountKey;
      console.log(`${seed} Mint Account:`, mintAccounts[tokenNames[index]].toString());
    });
  });

  for (const [index, token] of tokenNames.entries()) {
    it(`Initializes ${token} mint`, async () => {
      await program.methods[`init${token}`]()
        .accounts({
          admin: provider.wallet.publicKey,
          [`mint${token}`]: mintAccounts[token],
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_2022_PROGRAM_ID,
        })
        .rpc({ commitment: "confirmed" });

      // Validate mint account
      const mintAccount = await getMint(
        program.provider.connection,
        mintAccounts[token],
        "confirmed",
        TOKEN_2022_PROGRAM_ID
      );
      assert.ok(mintAccount);
      console.log("Mint account :\n", mintAccount);
    });
  }
});
