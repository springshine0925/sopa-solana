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
} from "@solana/web3.js";
import * as Buffer from "buffer";
import { assert, config } from "chai";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
import {
  getAssTokenAddr,
  getMockMintAndFeeReceiver,
  getMockMintKey,
  get_signer,
  getprovider,
} from "./01_sopad-solana-init";

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
} from "@solana/spl-token";

const [provider,program]=getprovider();

const signer = provider.publicKey!;
const [mockMint, feeReceiver, mockEthMint] = getMockMintAndFeeReceiver();

const receiverKey = Keypair.generate();
const receiver = receiverKey.publicKey;
const mockMintKey = getMockMintKey();

describe("sopad-solana withdraw", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  it("withdraw spl test", async () => {
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


    // calc userTokenAccountPDA
    const userTokenAccountPDA = getAssTokenAddr(mockMint, signer);
    // calc poolTokenAccountPDA
    const poollpTokenAccountPDA = getAssTokenAddr(mockEthMint, poolConfigPDA);
    // calc poolTokenAccountPDA
    const userLpTokenAccountPDA = getAssTokenAddr(mockEthMint, signer);
    console.log("deposit",userLpTokenAccountPDA);
    
    const poolLpTokenAccount = await getAccount(
        provider.connection,
        poollpTokenAccountPDA
      );
    console.log("poolLpTokenAccount",poolLpTokenAccount);
    const userLpTokenAccount = await getAccount(
      provider.connection,
      userLpTokenAccountPDA
    );
    console.log("userLpTokenAccount",userLpTokenAccount);

    const [poolTokenAccount] = web3.PublicKey.findProgramAddressSync(
      [Buffer.Buffer.from("pool-sol-token-account", "utf8")],
      program.programId
    );

    try {
      await program.methods
        .withdraw({
          offerAmount: new anchor.BN(1),
          poolIndex: pool_number,
          lpAmount: new anchor.BN(0),
        })
        .accounts({
          ifoConfig:ifoConfigPDA,
          signer: signer,
          poolConfig: poolConfigPDA,
          userLpTokenAccount: userLpTokenAccountPDA,
          lpTokenMint: mockEthMint,
          poolLpTokenAccount: poollpTokenAccountPDA,
          poolTokenAccount:poolTokenAccount
        })
        .rpc();
    } catch (error) {
      console.log(error);
    }
  });
  it("withdraw sol test", async () => {
    const pool_number = new anchor.BN(1);

    
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

    // calc poolTokenAccountPDA
    const userLpTokenAccountPDA = getAssTokenAddr(mockEthMint, signer);
    console.log("deposit",userLpTokenAccountPDA);

    const [poolTokenAccount] = web3.PublicKey.findProgramAddressSync(
      [Buffer.Buffer.from("pool-sol-token-account", "utf8")],
      program.programId
    );

    // try {
    //   await program.methods
    //     .withdraw({
    //       offerAmount: new anchor.BN(0.05),
    //       poolIndex: pool_number,
    //       lpAmount: new anchor.BN(0),
    //     })
    //     .accounts({
    //     //   receiver: receiver,
    //       ifoConfig:ifoConfigPDA,
    //       signer: signer,
    //       poolConfig: poolConfigPDA,
    //       userLpTokenAccount: userLpTokenAccountPDA,
    //       lpTokenMint: mockEthMint,
    //       poolLpTokenAccount: null,
    //       poolTokenAccount:poolTokenAccount
    //     })
    //     .rpc();
    // } catch (error) {
    //   console.log(error);
    // }
  });
});
