// import * as anchor from "@coral-xyz/anchor";
// import { Program, web3, Wallet, AnchorProvider, BN } from "@coral-xyz/anchor";
// import { SopadSolana } from "../target/types/sopad_solana";

// import * as Buffer from "buffer";
// import { assert, config } from "chai";
// import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
// import { getAssTokenAddr, getMockMintAndFeeReceiver, get_signer } from "./01_sopad-solana-init";

// const provider = anchor.AnchorProvider.env();
// anchor.setProvider(provider);
// const program = anchor.workspace.SopadSolana as Program<SopadSolana>;
// const signer = provider.publicKey!;

// const offeringToken = 20000;
// const startTime = 1912380000;
// const endTime = 1912480000;
// const claimTime = 1932582499;
// const minAmount = 700;
// const maxAmount = 300000;
// const raisingAmount = 33000;



// describe("sopad-solana udpate_pool", () => {
//   // Configure the client to use the local cluster.
//   anchor.setProvider(anchor.AnchorProvider.env());
  
//   it("update_pool test", async () => {
//     const pool_number = new anchor.BN(0);

//     // calc ifoConfig pubkey
//     const [ifoConfigPDA] = web3.PublicKey.findProgramAddressSync(
//       [Buffer.Buffer.from("ifo-config", "utf8")],
//       program.programId
//     );
//     // calc poolConfig pubkey
//     const [poolConfigPDA] = web3.PublicKey.findProgramAddressSync(
//       [
//         Buffer.Buffer.from("pool-config", "utf8"),
//         pool_number.toArrayLike(Buffer.Buffer, "be", 16),
//       ],
//       program.programId
//     );

//     const poolConfig = await program.account.poolConfig.fetch(poolConfigPDA);
//     console.log("poolConfig",poolConfig);
    

//     const [poolUnlockPDA] = web3.PublicKey.findProgramAddressSync(
//         [
//           Buffer.Buffer.from("pool-unlock-info", "utf8"),
//           pool_number.toArrayLike(Buffer.Buffer, "be", 16),
//         ],
//         program.programId
//       );

//     try {
//       await program.methods
//         .updatePool({
//             poolIdex: new anchor.BN(pool_number),
//             offeringAmount: new anchor.BN(offeringToken),
//             startTime: new anchor.BN(startTime),
//             endTime: new anchor.BN(endTime),
//             claimTime: new anchor.BN(claimTime),
//             minAmount: new anchor.BN(minAmount),
//             maxAmount: new anchor.BN(maxAmount),
//             raisingAmount: new BN(raisingAmount),
//             initialRate: new anchor.BN(2e7),
//             tn:new anchor.BN(10),
//             cliff:new anchor.BN(11),
//             period:new anchor.BN(12),
//         })
//         .accounts({
//           signer: signer,
//           ifoConfig: ifoConfigPDA,
//           poolConfig: poolConfigPDA,
//         })
//         .rpc();
//     } catch (error) {
//       console.log(error);
//     }
//     const new_poolConfig = await program.account.poolConfig.fetch(poolConfigPDA);
//     console.log("new_poolConfig",new_poolConfig);
//     console.log("raisingAmount",new_poolConfig.raisingAmount.toNumber());
    
    
//   });

  
// });
