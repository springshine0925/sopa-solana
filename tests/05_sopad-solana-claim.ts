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
// import {
//   getAssTokenAddr,
//   getMockMintAndFeeReceiver,
//   get_signer,
//   getprovider,
// } from "./01_sopad-solana-init";

// const [provider,program]=getprovider();
// const signer = provider.publicKey!;



// const [mockMint, feeReceiver, mockEthMint] = getMockMintAndFeeReceiver();

// describe("sopad-solana claim", () => {
//   // Configure the client to use the local cluster.
//   anchor.setProvider(anchor.AnchorProvider.env());

//   it("claim spl test", async () => {
//     const pool_number = new anchor.BN(0);
//     const Time = Math.floor(Date.now() / 1000);

//     // calc poolConfig pubkey
//     const [poolConfigPDA] = web3.PublicKey.findProgramAddressSync(
//       [
//         Buffer.Buffer.from("pool-config", "utf8"),
//         pool_number.toArrayLike(Buffer.Buffer, "be", 16),
//       ],
//       program.programId
//     );
//     const [poolUnlockPDA] = web3.PublicKey.findProgramAddressSync(
//       [
//         Buffer.Buffer.from("pool-unlock-info", "utf8"),
//         pool_number.toArrayLike(Buffer.Buffer, "be", 16),
//       ],
//       program.programId
//     );

//     // fetch poolConfig info
//     const poolConfig = await program.account.poolConfig.fetch(poolConfigPDA);
//     const [claimOrderAccountPDA] = web3.PublicKey.findProgramAddressSync(
//       [
//         Buffer.Buffer.from("claim-order-seed", "utf8"),
//         poolConfigPDA.toBuffer(),
//         poolConfig.totalClaimOrder.toArrayLike(Buffer.Buffer, "be", 16),
//       ],
//       program.programId
//     );

//     const [userConfigPDA] = web3.PublicKey.findProgramAddressSync(
//         [
//           Buffer.Buffer.from("user-config", "utf8"),
//           signer.toBuffer(),
//         ],
//         program.programId
//       );
  
//     console.log("userConfigPDA",userConfigPDA);
//         // fetch poolConfig info
//     const userConfig = await program.account.userConfig.fetch(userConfigPDA);
//     console.log("user config",userConfig);

//     const poollpTokenAccountPDA = getAssTokenAddr(mockEthMint, poolConfigPDA);

//     // calc poolTokenAccountPDA
//     const userLpTokenAccountPDA = getAssTokenAddr(mockEthMint, signer);
//     const [poolTokenAccount] = web3.PublicKey.findProgramAddressSync(
//         [Buffer.Buffer.from("pool-sol-token-account", "utf8")],
//         program.programId
//       );
  
//     try {
//       await program.methods
//         .claim({
//           poolIndex: pool_number,
//           userAccount: userConfigPDA,
//           poolConfigPda: poolConfigPDA,
//         })
//         .accounts({
//           signer: signer,
//           poolConfig: poolConfigPDA,
//           poolTokenAccount:poolTokenAccount,
//           lpTokenMint: mockEthMint,//rasing token
//           userConfig: userConfigPDA,
//           userLpTokenAccount: userLpTokenAccountPDA, //rasing token
//           poolLpTokenAccount: poollpTokenAccountPDA,//rasing token
//           claimOrderAccount: claimOrderAccountPDA,
//           offeringTokenMint:mockMint,//offering token
//           poolOfferingTokenAccount: getAssTokenAddr(mockMint, poolConfigPDA),
//           userOfferingTokenAccount: getAssTokenAddr(mockMint, signer),

//         })
//         .rpc();
//     } catch (error) {
//       console.log(error);
//     }

//     //get refunding amount fun test
//     try{
//      let refunding_data=await program.methods.getRefundingAmountFun({
//       poolIndex: pool_number,
//      }).accounts({
//         signer: signer,
//         poolConfig: poolConfigPDA,
//         userConfig: userConfigPDA,
//       })
//       .rpc();
//       console.log(refunding_data);
//     }catch (error) {
//       console.log(error);
//     }

//     //get offering amount fun test
//     try{
//       let offering_data=await program.methods.getOfferingAmountFun({
//        poolIndex: pool_number,
//       }).accounts({
//          signer: signer,
//          poolConfig: poolConfigPDA,
//          userConfig: userConfigPDA,

//        })
//        .rpc();
//        console.log(offering_data);
//      }catch (error) {
//        console.log(error);
//      }

//     //get claim amount fun test
//     try{
//       let claim_amount=await program.methods.getClaimAmountFun({
//        poolIndex: pool_number,
//       }).accounts({
//          signer: signer,
//          poolConfig: poolConfigPDA,
//          userConfig: userConfigPDA,

//        })
//        .rpc();
//        console.log(claim_amount);
//      }catch (error) {
//        console.log(error);
//      }
//   });

//   // it("claim sol test", async () => {
//   //   const pool_number = new anchor.BN(1);
//   //   const Time = Math.floor(Date.now() / 1000);

//   //   // calc poolConfig pubkey
//   //   const [poolConfigPDA] = web3.PublicKey.findProgramAddressSync(
//   //     [
//   //       Buffer.Buffer.from("pool-config", "utf8"),
//   //       pool_number.toArrayLike(Buffer.Buffer, "be", 16),
//   //     ],
//   //     program.programId
//   //   );

//   //   // fetch poolConfig info
//   //   const poolConfig = await program.account.poolConfig.fetch(poolConfigPDA);
//   //   const [claimOrderAccountPDA] = web3.PublicKey.findProgramAddressSync(
//   //     [
//   //       Buffer.Buffer.from("claim-order-seed", "utf8"),
//   //       poolConfigPDA.toBuffer(),
//   //       poolConfig.totalClaimOrder.toArrayLike(Buffer.Buffer, "be", 16),
//   //     ],
//   //     program.programId
//   //   );


//   //   const [userConfigPDA] = web3.PublicKey.findProgramAddressSync(
//   //       [
//   //         Buffer.Buffer.from("user-config", "utf8"),
//   //         signer.toBuffer(),
//   //       ],
//   //       program.programId
//   //     );
  
//   //   console.log("userConfigPDA",userConfigPDA);
//   //       // fetch poolConfig info
//   //   const userConfig = await program.account.userConfig.fetch(userConfigPDA);
//   //   console.log("user config",userConfig);

//   //   const [poolTokenAccount] = web3.PublicKey.findProgramAddressSync(
//   //       [Buffer.Buffer.from("pool-sol-token-account", "utf8")],
//   //       program.programId
//   //     );

//   //     const [poolUnlockPDA] = web3.PublicKey.findProgramAddressSync(
//   //       [
//   //         Buffer.Buffer.from("pool-unlock-info", "utf8"),
//   //         pool_number.toArrayLike(Buffer.Buffer, "be", 16),
//   //       ],
//   //       program.programId
//   //     );
//   //   try {
//   //     await program.methods
//   //       .claim({
//   //         amount: new anchor.BN(1),
//   //         time: new anchor.BN(Time),
//   //         poolIndex: pool_number,
//   //         userAccount: userConfigPDA,
//   //         poolConfigPda: poolConfigPDA,
//   //       })
//   //       .accounts({
//   //         signer: signer,
//   //         poolConfig: poolConfigPDA,
//   //         poolTokenAccount:poolTokenAccount,
//   //         lpTokenMint: null,
//   //         userConfig: userConfigPDA,
//   //         userLpTokenAccount: null,
//   //         poolLpTokenAccount: null,
//   //         claimOrderAccount: claimOrderAccountPDA,

//   //       })
//   //       .rpc();
//   //   } catch (error) {
//   //     console.log(error);
//   //   }

//     //get refunding amount fun test
// //     try{
// //      let refunding_data=await program.methods.getRefundingAmountFun({
// //       poolIndex: pool_number,
// //      }).accounts({
// //         signer: signer,
// //         poolConfig: poolConfigPDA,
// //         userConfig: userConfigPDA,

// //       })
// //       .rpc();
// //       console.log(refunding_data);
// //     }catch (error) {
// //       console.log(error);
// //     }

// //     //get offering amount fun test
// //     try{
// //       let offering_data=await program.methods.getOfferingAmountFun({
// //        poolIndex: pool_number,
// //       }).accounts({
// //          signer: signer,
// //          poolConfig: poolConfigPDA,
// //          userConfig: userConfigPDA,

// //        })
// //        .rpc();
// //        console.log(offering_data);
// //      }catch (error) {
// //        console.log(error);
// //      }

// //     //get claim amount fun test
// //     try{
// //       let claim_amount=await program.methods.getClaimAmountFun({
// //        poolIndex: pool_number,
// //       }).accounts({
// //          signer: signer,
// //          poolConfig: poolConfigPDA,
// //          userConfig: userConfigPDA,

// //        })
// //        .rpc();
// //        console.log(claim_amount);
// //      }catch (error) {
// //        console.log(error);
// //      }
// //   });

// });
