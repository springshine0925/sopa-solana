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
import { KolIfo } from "../target/types/kol_ifo";
const secp256k1 = require("secp256k1");

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const program = anchor.workspace.KolIfo as Program<KolIfo>;
const signer = provider.publicKey!;

const mockMintKey = Keypair.generate();
const mockMint = mockMintKey.publicKey;
// const mockMint=new PublicKey("5cW85NPQpNZBU9Kfzoi9xxiXtfzH2JL7mw2bKFZBspzq")

const mockEthMintKey = Keypair.generate();
const mockEthMint = mockEthMintKey.publicKey;

const feeReceiverKey = Keypair.generate();
const feeReceiver = feeReceiverKey.publicKey;
// const feeReceiver=new PublicKey("5cW85NPQpNZBU9Kfzoi9xxiXtfzH2JL7mw2bKFZBspzq")

const userAccountKey = Keypair.generate();
const userAccount = userAccountKey.publicKey;

const receiverKey = Keypair.generate();
const receiver = receiverKey.publicKey;

const IFOadminKey = Keypair.generate();
const ifoadmin = IFOadminKey.publicKey;
// const signer = ifoadmin;


const offeringToken = 10000;
const startTime = 1700547057;
const endTime = 1722582489;
const claimTime = 1732582489;
const minAmount = 0;
const maxAmount = 100000;
const offeringAmount = 10000;
const raisingAmount = 13000;

const offeringToken2 = 99999;
const startTime2 = 1812380000;
const endTime2 = 1812480000;
const claimTime2 = 1832582499;
const minAmount2 = 8888;
const maxAmount2 = 888888;
const offeringAmount2 = 20000;
const raisingAmount2 = 88000;

let secp256k1PrivateKey;do {
  secp256k1PrivateKey = web3.Keypair.generate().secretKey.slice(0, 32);
} while (!secp256k1.privateKeyVerify(secp256k1PrivateKey));
//  console.log("secp256k1PrivateKey",secp256k1PrivateKey);

 let secp256k1PublicKey = secp256k1
.publicKeyCreate(secp256k1PrivateKey, false)
.slice(1);
// console.log("secp256k1PublicKey",secp256k1PublicKey);


export function getMockMintAndFeeReceiver(): [web3.PublicKey, web3.PublicKey,web3.PublicKey] {
  return [mockMint, feeReceiver,mockEthMint];
}

export function getReceiver(): web3.PublicKey{
  return receiver;
}


export function getMockMintKey():Keypair{
  return mockMintKey;
}


export function getManagerKey():Keypair{
  return IFOadminKey;
}

export function getAssTokenAddr(
  mint: web3.PublicKey,
  owner: web3.PublicKey
): web3.PublicKey {
  return getAssociatedTokenAddressSync(mint, owner, true);
}

export function get_private_key():number[]{
  return secp256k1PrivateKey;
}

export function get_signer():web3.PublicKey{
  return signer;
}
describe("kol init", () => {
  it("create pool spl test!", async () => {
    // calc ifoConfig pubkey
    const [ifoConfigPDA] = web3.PublicKey.findProgramAddressSync(
      [Buffer.Buffer.from("ifo-config", "utf8")],
      program.programId
    );
    //fetch ifoConfig info
    const ifoConfig = await program.account.ifoConfig.fetch(ifoConfigPDA);
    // calc poolConfig pubkey
    const [poolConfigPDA] = web3.PublicKey.findProgramAddressSync(
      [
        Buffer.Buffer.from("pool-config", "utf8"),
        ifoConfig.poolNumber.toArrayLike(Buffer.Buffer, "be", 16),
      ],
      program.programId
    );

    const pool_number = new anchor.BN(0);
    const [poolConfigPDAnew] = web3.PublicKey.findProgramAddressSync(
      [
        Buffer.Buffer.from("pool-config", "utf8"),
        pool_number.toArrayLike(Buffer.Buffer, "be", 16),
      ],
      program.programId
    );

    console.log("signer",signer);
    try {
      await program.methods
        .createPool({
          startTime: new anchor.BN(startTime),
          endTime: new anchor.BN(endTime),
          claimTime: new anchor.BN(claimTime),
          minAmount: new anchor.BN(0),
          maxAmount: new anchor.BN(maxAmount),
          offeringAmount: new BN(offeringAmount),
          raisingAmount: new BN(raisingAmount),
          overFunding: false,
        })
        .accounts({
          signer:signer,
          ifoConfig: ifoConfigPDA,
          poolConfig: poolConfigPDAnew,
        })
        .rpc();
    } catch (error) {
      console.log(error);
    }

    try {
      await program.methods
        .supplementPool({
          tokenMint: mockMint,
          isRefund: true,
          lpToken: mockEthMint,
          initialRate: new anchor.BN(0),
          tn: new anchor.BN(1),
          cliff: new anchor.BN(2),
          period: new anchor.BN(4),
          poolIndex:pool_number
        })
        .accounts({
          signer:signer,
          ifoConfig: ifoConfigPDA,
          poolConfig: poolConfigPDA,
        })
        .rpc();
    } catch (error) {
      console.log(error);
    }
    //fetch account info
    const poolConfig = await program.account.poolConfig.fetch(poolConfigPDA);
    // console.log("poolConfig: " , poolConfig);
    assert(poolConfig.claimTime.toNumber() == claimTime);
    const ifoConfig_new = await program.account.ifoConfig.fetch(ifoConfigPDA);
    assert(ifoConfig_new.poolNumber.toNumber() == 1);
    // console.log("ifoConfig_new",ifoConfig_new);
  });

});
