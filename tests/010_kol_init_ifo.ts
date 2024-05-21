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
  TransactionInstruction,
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
import * as ethUtil from "@ethereumjs/util";
import { KolIfo } from "../target/types/kol_ifo";

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


const privateKey = ethUtil.hexToBytes("0x1111111111111111111111111111111111111111111111111111111111111111")
const publicKey = ethUtil.privateToPublic(privateKey)

export function getPrivateKey():Uint8Array{
  return privateKey
}

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

export function getprovider():[anchor.AnchorProvider,Program<KolIfo>]{
  return [provider,program];
}

export function get_signer():web3.PublicKey{
  return signer;
}
describe("kol init", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  it("mock token init mint ", async () => {

    const requiredBalance = await getMinimumBalanceForRentExemptMint(
      provider.connection
    );
    const createNewTokenTransaction = new Transaction().add(
      SystemProgram.createAccount({
        fromPubkey: signer,
        newAccountPubkey: mockEthMint,
        space: MINT_SIZE,
        lamports: requiredBalance,
        programId: TOKEN_PROGRAM_ID,
      }),
      createInitializeMintInstruction(
        mockEthMint, //Mint Address
        6, //Number of Decimals of New mint
        signer, //Mint Authority
        signer
      ),
      createAssociatedTokenAccountInstruction(
        signer,
        getAssTokenAddr(mockEthMint, signer), //userLpTokenAccountPDA
        signer, //token owner
        mockEthMint //Mint
      ),
      createMintToInstruction(
        mockEthMint, //Mint
        getAssTokenAddr(mockEthMint, signer), //userLpTokenAccountPDA
        signer, //Authority
        BigInt("10000000000000000000") //number of
      )
    );
    await provider.sendAndConfirm(createNewTokenTransaction, [mockEthMintKey]);
    const createNewEthTransaction = new Transaction().add(
      SystemProgram.createAccount({
        fromPubkey: signer,
        newAccountPubkey: mockMint,
        space: MINT_SIZE,
        lamports: requiredBalance,
        programId: TOKEN_PROGRAM_ID,
      }),
      createInitializeMintInstruction(
        mockMint, //Mint Address
        6, //Number of Decimals of New mint
        signer, //Mint Authority
        signer
      ),
      createAssociatedTokenAccountInstruction(
        signer,
        getAssTokenAddr(mockMint, signer), //userTokenAccountPDA
        signer, //token owner
        mockMint //Mint
      ),
      createMintToInstruction(
        mockMint, //Mint
        getAssTokenAddr(mockMint, signer), //userTokenAccountPDA
        signer, //Authority
        BigInt("10000000000000000000") //number of
      )
    );
    await provider.sendAndConfirm(createNewEthTransaction, [mockMintKey]);
    
    
  });

  it("Is initialized!", async () => {
    // calc ifo-config pubkey
    const [IfoConfigPDA] = web3.PublicKey.findProgramAddressSync(
      [Buffer.Buffer.from("ifo-config", "utf8")],
      program.programId
    );

    const result: number[] = new Array(64).fill(0);
    for (let i = 0; i < Math.min(publicKey.length, 64); i++) {
        result[i] = publicKey[i];
    }
    await program.methods
      .initialize({
        admin: signer,
        manager:result
      })
      .accounts({
        signer:signer,
        contractsConfig: IfoConfigPDA,
      })
      .rpc();

      const pool_number = new anchor.BN(0);
      const [poolConfigPDA] = web3.PublicKey.findProgramAddressSync(
        [
          Buffer.Buffer.from("pool-config", "utf8"),
          pool_number.toArrayLike(Buffer.Buffer, "be", 16),
        ],
        program.programId
      );
  
      const createTokenAccountTransaction = new Transaction().add(
        createAssociatedTokenAccountInstruction(
          signer,
          getAssTokenAddr(mockMint, poolConfigPDA), //userLpTokenAccountPDA
          poolConfigPDA, //token owner
          mockMint //Mint
        ),
        createAssociatedTokenAccountInstruction(
          signer,
          getAssTokenAddr(mockEthMint, poolConfigPDA), //userLpTokenAccountPDA
          poolConfigPDA, //token owner
          mockEthMint //Mint
        ),
      );
      const txn = await provider.sendAndConfirm(createTokenAccountTransaction);
   
  });

});
