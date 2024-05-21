// import * as anchor from "@coral-xyz/anchor";
// import { Program, web3, Wallet, AnchorProvider, BN } from "@coral-xyz/anchor";
// import { SopadSolana } from "../target/types/sopad_solana";
// import {
//   PublicKey,
//   SystemProgram,
//   SYSVAR_RENT_PUBKEY,
//   Transaction,
//   AccountMeta,
//   LAMPORTS_PER_SOL,
//   Keypair,
// } from "@solana/web3.js";
// import {
//   getAssociatedTokenAddressSync,
//   createInitializeMintInstruction,
//   createMintToInstruction,
//   getMinimumBalanceForRentExemptMint,
//   createTransferInstruction,
//   MINT_SIZE,
//   TOKEN_PROGRAM_ID,
//   createAssociatedTokenAccountInstruction,
//   getAccount,
//   createInitializeAccountInstruction,
// } from "@solana/spl-token";
// import * as Buffer from "buffer";
// import { assert, config } from "chai";
// import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
// import { getAssTokenAddr, getMockMintAndFeeReceiver, get_signer } from "./01_sopad-solana-init";

// const provider = anchor.AnchorProvider.env();
// anchor.setProvider(provider);
// const program = anchor.workspace.SopadSolana as Program<SopadSolana>;
// const signer = provider.publicKey!;

// const newOwnerKey = Keypair.generate();
// // const newOwner = newOwnerKey.publicKey;
// const newOwner=new PublicKey("8iLS2D49FjXEntKytuV1wEK33FkrqzzUh5ptHz21dfLZ")
// const newManagerKey = Keypair.generate();
// const newManager = newManagerKey.publicKey;

// describe("sopad-solana update_ifo", () => {
//   // Configure the client to use the local cluster.
//   anchor.setProvider(anchor.AnchorProvider.env());


//   it("udpate_ifo  test", async () => {
//     // calc ifoConfig pubkey
//     const [ifoConfigPDA] = web3.PublicKey.findProgramAddressSync(
//       [Buffer.Buffer.from("ifo-config", "utf8")],
//       program.programId
//     );

//     try {
//       await program.methods
//         .updateIfo({
//           admin:newOwner,
//         })
//         .accounts({
//           signer: signer,
//           ifoConfig: ifoConfigPDA,
//         })
//         .rpc();
//     } catch (error) {
//       console.log(error);
//     }
//     const new_ifoConfig= await program.account.ifoConfig.fetch(ifoConfigPDA);
//     // console.log("new ifoConfig",new_ifoConfig.manager.toString(),new_ifoConfig.owner.toString());
//     assert(new_ifoConfig.admin.toString() == newOwner.toString());
//   });




// });
