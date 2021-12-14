import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { chaiSolana, expectTX } from "@saberhq/chai-solana";
import { PendingTransaction, Provider, SolanaAugmentedProvider, SolanaProvider } from "@saberhq/solana-contrib";
import { TransactionEnvelope } from "@saberhq/solana-contrib";
import { LAMPORTS_PER_SOL, PublicKey, SystemProgram, SYSVAR_CLOCK_PUBKEY, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { Keypair } from "@solana/web3.js";
import { assert, expect, use } from "chai";
import { Example } from "../target/types/example";

const anchorProvider = anchor.Provider.env();
anchor.setProvider(anchorProvider);
const program = anchor.workspace.Example as Program<Example>;
const provider = SolanaProvider.load({
  connection: anchorProvider.connection,
  sendConnection: anchorProvider.connection,
  wallet: anchorProvider.wallet,
  opts: anchorProvider.opts,
});
use(chaiSolana);
describe("Start auction, end auction, claim nfts and redeem fractions", () => {
let exampleAccount: PublicKey;
let exampleBump: number;

const initializer = new Keypair();
const initializerPubkey = initializer.publicKey;

  before("Set up environment", async () => {

    // Airdrop
    await expectTX(
      new PendingTransaction(
        provider.connection,
        await provider.connection.requestAirdrop(
          initializerPubkey,
          100 * LAMPORTS_PER_SOL
        )
      )
    ).to.be.fulfilled;
    const balance = await provider.connection.getBalance(initializerPubkey)
    expect(balance === 1 * LAMPORTS_PER_SOL, 'Airdrop unsuccessful');

    // Example Accounts
    [exampleAccount, exampleBump] = await PublicKey.findProgramAddress(
      [Buffer.from(anchor.utils.bytes.utf8.encode("example"))], 
      program.programId
    );
  });

  it("initialize example", async () => {
    let initialize_txn = program.instruction.example(exampleBump, {
            accounts: {
                initializerAccount: initializerPubkey,
                exampleAccount: exampleAccount,
                systemProgram: SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY,
                clock: SYSVAR_CLOCK_PUBKEY,
            }
    })
    const txn = new TransactionEnvelope(provider, [initialize_txn], [initializer]);
    await expectTX(txn).to.be.fulfilled;
    let example_account = await program.account.exampleAccount.fetch(exampleAccount);
    let dateTime = new Date(2021,12,1)
    let creationTime = new Date(example_account.creationTime.toNumber() * 1000)
    assert.ok(creationTime >  dateTime, "Example account not intialized properly");
  });

});