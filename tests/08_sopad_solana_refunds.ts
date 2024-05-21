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
// import * as Buffer from "buffer";
// import { assert, config } from "chai";
// import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
// import {
//   getAssTokenAddr,
//   getMockMintAndFeeReceiver,
//   getMockMintKey,
//   get_signer,
//   getprovider,
// } from "./01_sopad-solana-init";

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

// const [provider,program]=getprovider();

// const signer = provider.publicKey!;
// const [mockMint, feeReceiver, mockEthMint] = getMockMintAndFeeReceiver();

// const receiverKey = Keypair.generate();
// const receiver = receiverKey.publicKey;
// const mockMintKey = getMockMintKey();

// describe("sopad-solana refund", () => {
//   // Configure the client to use the local cluster.
//   anchor.setProvider(anchor.AnchorProvider.env());

//   it("refund test", async () => {
//     const pool_number = new anchor.BN(0);
//     // calc ifoConfig pubkey
//     const [ifoConfigPDA] = web3.PublicKey.findProgramAddressSync(
//       [Buffer.Buffer.from("ifo-config", "utf8")],
//       program.programId
//     );
//     // calc poolConfig pubkey
//     const [poolConfigPDA] = web3.PublicKey.findProgramAddressSync(
//         [
//           Buffer.Buffer.from("pool-config", "utf8"),
//           pool_number.toArrayLike(Buffer.Buffer, "be", 16),
//         ],
//         program.programId
//       );
//       const poolConfig = await program.account.poolConfig.fetch(poolConfigPDA);
//       console.log("poolConfigPDA",poolConfig);

//     // calc poolTokenAccountPDA
//     const userLpTokenAccountPDA = getAssTokenAddr(mockEthMint, signer);

//     const [poolTokenAccount] = web3.PublicKey.findProgramAddressSync(
//       [Buffer.Buffer.from("pool-sol-token-account", "utf8")],
//       program.programId
//     );
//     // const poolTokendata = await program.account.poolConfig.fetch(poolTokenAccount);
//     // console.log("poolTokendata",poolTokendata);
//     const [poolUnlockPDA] = web3.PublicKey.findProgramAddressSync(
//         [
//           Buffer.Buffer.from("pool-unlock-info", "utf8"),
//           pool_number.toArrayLike(Buffer.Buffer, "be", 16),
//         ],
//         program.programId
//       );

//     const [userConfigPDA] = web3.PublicKey.findProgramAddressSync(
//         [
//           Buffer.Buffer.from("user-config", "utf8"),
//           signer.toBuffer(),
//         ],
//         program.programId
//       );
//       const poollpTokenAccountPDA = getAssTokenAddr(mockEthMint, poolConfigPDA);

//       const userConfigdata = await program.account.userConfig.fetch(userConfigPDA);
//           console.log("userConfigPDA",userConfigdata);
          
//     try {
//       await program.methods
//         .refund({
//           poolIndex: pool_number,
//         })
//         .accounts({
//           ifoConfig:ifoConfigPDA,
//           signer: signer,
//           poolConfig: poolConfigPDA,
//           userLpTokenAccount: userLpTokenAccountPDA,
//           lpTokenMint: mockEthMint,
//           poolLpTokenAccount: poollpTokenAccountPDA,
//           poolTokenAccount:poolTokenAccount,
//           userConfig:userConfigPDA
//         })
//         .rpc();
//     } catch (error) {
//       console.log(error);
//     }
//   });
// });
