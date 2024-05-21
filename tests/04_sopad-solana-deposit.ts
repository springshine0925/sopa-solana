import * as anchor from "@coral-xyz/anchor";
import { Program, web3, Wallet, AnchorProvider, BN } from "@coral-xyz/anchor";
import { SopadSolana } from "../target/types/sopad_solana";
import {
  PublicKey,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  Transaction,
  AccountMeta,
  LAMPORTS_PER_SOL,
  Keypair,
  sendAndConfirmTransaction,
  
  ComputeBudgetProgram,
} from "@solana/web3.js";
import nacl from "tweetnacl";
import {
  getAssociatedTokenAddressSync,
  createInitializeMintInstruction,
  createMintToInstruction,
  getMinimumBalanceForRentExemptMint,
  createTransferInstruction,
  MINT_SIZE,
  TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  getAccount,
  createInitializeAccountInstruction,
  createAssociatedTokenAccountIdempotentInstruction,
} from "@solana/spl-token";
import * as Buffer from "buffer";
import { assert, config } from "chai";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
import {
  getAssTokenAddr,
  getManagerKey,
  getMockMintAndFeeReceiver,
  getMockMintKey,
  getPrivateKey,
  get_signer,
  getprovider,
} from "./01_sopad-solana-init";

import * as ethUtil from "@ethereumjs/util";
import { keccak256 } from "ethereum-cryptography/keccak.js"

const DEPOSIT_TYPEHASH =
  "0x5389a5e529de40c9685335fe495d7d2f0e57ff19979a6e0484a4ebf599b6f2d4";

const [provider, program] = getprovider();

const signer = provider.publicKey!;

const [mockMint, feeReceiver, mockEthMint] = getMockMintAndFeeReceiver();

const amount = new anchor.BN(10);
const max_amount = new anchor.BN(500);
const private_key = getPrivateKey();

describe("sopad-solana deposit", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  it("deposit_pool spl test", async () => {
    const pool_number = new anchor.BN(0);
    // calc ifoConfig pubkey
    const [ifoConfigPDA] = web3.PublicKey.findProgramAddressSync(
      [Buffer.Buffer.from("ifo-config", "utf8")],
      program.programId
    );
    // calc poolConfig pubkey
    const [poolConfigPDA] = web3.PublicKey.findProgramAddressSync(
      [
        Buffer.Buffer.from("pool-config", "utf8"),
        pool_number.toArrayLike(Buffer.Buffer, "be", 16),
      ],
      program.programId
    );

    // fetch poolConfig info
    const poolConfig = await program.account.poolConfig.fetch(poolConfigPDA);
    const [depositOrderPDA] = web3.PublicKey.findProgramAddressSync(
      [
        Buffer.Buffer.from("deposit-order", "utf8"),
        poolConfigPDA.toBuffer(),
        poolConfig.totalDepositAmount.toArrayLike(Buffer.Buffer, "be", 16),
      ],
      program.programId
    );
    // console.log("poolConfig", poolConfig); 
    const [userConfigPDA] = web3.PublicKey.findProgramAddressSync(
      [Buffer.Buffer.from("user-config", "utf8"), signer.toBuffer()],
      program.programId
    );
    // console.log("userConfigPDA", userConfigPDA);

    const uint8Array_private_key = new Uint8Array(private_key);
    
    const message = getDigest(
      signer.toString(),
      pool_number.toNumber(),
      amount.toNumber(),
      max_amount.toNumber()
    );
    const messageForActivation = new TextEncoder().encode(message) // activation msg
    const hashedMessageForActivation = keccak256(messageForActivation)

    const { r, s, v} = ethUtil.ecsign(hashedMessageForActivation, private_key)
    const signature = Uint8Array.from([...r, ...s])
    const recoveryId = Number(ethUtil.calculateSigRecovery(v))

    // calc userTokenAccountPDA
    const userlpTokenAccountPDA = getAssTokenAddr(mockEthMint, signer);
    // calc poolTokenAccountPDA
    const poollpTokenAccountPDA = getAssTokenAddr(mockEthMint, poolConfigPDA);



    const [poolTokenAccount] = web3.PublicKey.findProgramAddressSync(
      [Buffer.Buffer.from("pool-sol-token-account", "utf8")],
      program.programId
    );
    const modifyComputeUnits = ComputeBudgetProgram.setComputeUnitLimit({
      units: 9999999,
    });
      const addPriorityFee = ComputeBudgetProgram.setComputeUnitPrice({
        microLamports: 1,
      });
      const result: number[] = new Array(64).fill(0);
      for (let i = 0; i < Math.min(signature.length, 64); i++) {
          result[i] = signature[i];
      }
      
      const deposit= await program.methods
      .depositPool({
        maxAmount: max_amount,
        poolIndex: pool_number,
        amount: amount,
        signature:result,
        recoveryId:recoveryId,
        message: Buffer.Buffer.from(messageForActivation),
      })
      .accounts({
        signer: signer,
        ifoConfig: ifoConfigPDA,
        poolConfig: poolConfigPDA,
        userConfig: userConfigPDA,
        userLpTokenAccount: userlpTokenAccountPDA,
        depositOrderAccount: depositOrderPDA,
        lpTokenMint: mockEthMint,
        poolLpTokenAccount: poollpTokenAccountPDA,
      })
      .instruction();
      const transaction = new Transaction()
      .add(modifyComputeUnits)
      .add(addPriorityFee)
      .add(deposit)
    
        try {
            await provider.sendAndConfirm(transaction);
          } catch (error) {
            console.log(error);
          }
   

  });
//   it("deposit_pool sol test", async () => {
//     const pool_number = new anchor.BN(1);
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

//     // fetch poolConfig info
//     const poolConfig = await program.account.poolConfig.fetch(poolConfigPDA);
//     const [depositOrderPDA] = web3.PublicKey.findProgramAddressSync(
//       [
//         Buffer.Buffer.from("deposit-order", "utf8"),
//         poolConfigPDA.toBuffer(),
//         poolConfig.totalDepositAmount.toArrayLike(Buffer.Buffer, "be", 16),
//       ],
//       program.programId
//     );
//     // console.log("poolConfig", poolConfig);

//     const [userConfigPDA] = web3.PublicKey.findProgramAddressSync(
//       [Buffer.Buffer.from("user-config", "utf8"), signer.toBuffer()],
//       program.programId
//     );
//     // console.log("userConfigPDA", userConfigPDA);

//     const uint8Array_private_key = new Uint8Array(private_key);
    
//     const message = getDigest(
//       signer.toString(),
//       pool_number.toNumber(),
//       amount.toNumber(),
//       max_amount.toNumber()
//     );
//     // console.log("message", message);
//     const plaintext = Buffer.Buffer.from(message, "utf8");
//     let plaintextHash = Buffer.Buffer.from(
//       keccak_256.update(plaintext).digest(),
//       "utf8"
//     );

//     // console.log("message hash", plaintext);
//     let { signature, recid: recoveryId } = secp256k1.ecdsaSign(
//       plaintextHash,
//       uint8Array_private_key
//     );

//     const [poolTokenAccount] = web3.PublicKey.findProgramAddressSync(
//       [Buffer.Buffer.from("pool-sol-token-account", "utf8")],
//       program.programId
//     );

//     // try {
//     //   await program.methods
//     //     .depositSolPool({
//     //       maxAmount: max_amount,
//     //       poolIndex: pool_number,
//     //       amount: new anchor.BN(1),
//     //       signature:signature.toString(),
//     //       recoveryId: recoveryId,
//     //     })
//     //     .accounts({
//     //       signer: signer,
//     //       ifoConfig: ifoConfigPDA,
//     //       poolConfig: poolConfigPDA,
//     //       userConfig: userConfigPDA,
//     //       depositOrderAccount: depositOrderPDA,
//     //       poolTokenAccount: poolTokenAccount,
//     //     })
//     //     .rpc();
//     // } catch (error) {
//     //   console.log(error);
//     // }

//   });
});

function getDigest(
  user: string,
  pid: number,
  amount: number,
  maxAmount: number
): string {
  let data = Buffer.Buffer.alloc(0);
  console.log("user:", user,"pool_index:",pid,"amount:",amount,"maxAmount:",maxAmount);
  data = Buffer.Buffer.concat([
    data,
    Buffer.Buffer.from(
      DEPOSIT_TYPEHASH +
        user +
        pid.toString() +
        amount.toString() +
        maxAmount.toString()
    ),
  ]);
  const result = keccak256(data);
  return "0x" + result.toString();
}
