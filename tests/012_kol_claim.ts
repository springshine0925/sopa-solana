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
import * as Buffer from "buffer";
import { assert, config } from "chai";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
import { getAssTokenAddr, getMockMintAndFeeReceiver, getPrivateKey, getprovider } from "./010_kol_init_ifo";
import { keccak256 } from "ethereum-cryptography/keccak.js"
import * as ethUtil from "@ethereumjs/util";


const [provider,program]=getprovider();
const signer = provider.publicKey!;

const amount = new anchor.BN(10);
const max_amount = new anchor.BN(500);

const [mockMint, feeReceiver, mockEthMint] = getMockMintAndFeeReceiver();
const private_key = getPrivateKey();

describe("kol claim", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  it("claim kol spl test", async () => {
    const pool_number = new anchor.BN(0);
    const Time = Math.floor(Date.now() / 1000);

    // calc poolConfig pubkey
    const [poolConfigPDA] = web3.PublicKey.findProgramAddressSync(
      [
        Buffer.Buffer.from("pool-config", "utf8"),
        pool_number.toArrayLike(Buffer.Buffer, "be", 16),
      ],
      program.programId
    );
    const [poolUnlockPDA] = web3.PublicKey.findProgramAddressSync(
      [
        Buffer.Buffer.from("pool-unlock-info", "utf8"),
        pool_number.toArrayLike(Buffer.Buffer, "be", 16),
      ],
      program.programId
    );

    // fetch poolConfig info
    const poolConfig = await program.account.poolConfig.fetch(poolConfigPDA);
    const [claimOrderAccountPDA] = web3.PublicKey.findProgramAddressSync(
      [
        Buffer.Buffer.from("claim-order-seed", "utf8"),
        poolConfigPDA.toBuffer(),
        poolConfig.totalClaimOrder.toArrayLike(Buffer.Buffer, "be", 16),
      ],
      program.programId
    );

    const [userConfigPDA] = web3.PublicKey.findProgramAddressSync(
        [
          Buffer.Buffer.from("user-config", "utf8"),
          signer.toBuffer(),
        ],
        program.programId
      );
  
    console.log("userConfigPDA",userConfigPDA);
        // fetch poolConfig info
    const userConfig = await program.account.userConfig.fetch(userConfigPDA);
    console.log("user config",userConfig);

    const poollpTokenAccountPDA = getAssTokenAddr(mockEthMint, poolConfigPDA);

    // calc poolTokenAccountPDA
    const userLpTokenAccountPDA = getAssTokenAddr(mockEthMint, signer);
    const [poolTokenAccount] = web3.PublicKey.findProgramAddressSync(
        [Buffer.Buffer.from("pool-sol-token-account", "utf8")],
        program.programId
      );
          
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
  
      const result: number[] = new Array(64).fill(0);
      for (let i = 0; i < Math.min(signature.length, 64); i++) {
          result[i] = signature[i];
      }

      const [ifoConfigPDA] = web3.PublicKey.findProgramAddressSync(
        [Buffer.Buffer.from("ifo-config", "utf8")],
        program.programId
      );
      
  

    try {
      await program.methods
        .claim({
          poolIndex: pool_number,
          userAccount: userConfigPDA,
          poolConfigPda: poolConfigPDA,
          signature:result,
        recoveryId:recoveryId,
        message: Buffer.Buffer.from(messageForActivation),
        })
        .accounts({
          signer: signer,
          poolConfig: poolConfigPDA,
          poolTokenAccount:poolTokenAccount,
          lpTokenMint: mockEthMint,//rasing token
          userConfig: userConfigPDA,
          userLpTokenAccount: userLpTokenAccountPDA, //rasing token
          poolLpTokenAccount: poollpTokenAccountPDA,//rasing token
          claimOrderAccount: claimOrderAccountPDA,
          offeringTokenMint:mockMint,//offering token
          poolOfferingTokenAccount: getAssTokenAddr(mockMint, poolConfigPDA),
          userOfferingTokenAccount: getAssTokenAddr(mockMint, signer),
          ifoConfig: ifoConfigPDA,

        })
        .rpc();
    } catch (error) {
      console.log(error);
    }


  });


});

const DEPOSIT_TYPEHASH =
  "0x5389a5e529de40c9685335fe495d7d2f0e57ff19979a6e0484a4ebf599b6f2d4";

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
  