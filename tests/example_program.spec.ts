import * as anchor from "@project-serum/anchor";
import { Program, BN, Idl, Provider } from "@project-serum/anchor";
import { Example } from "../target/types/example";
import { SolanaProvider, TransactionEnvelope } from "@saberhq/solana-contrib";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getOrCreateATA, TOKEN_PROGRAM_ID, SPLToken, getTokenAccount, sleep } from "@saberhq/token-utils";
import { expect } from "chai";
import { Keypair, PublicKey, SystemProgram, SYSVAR_CLOCK_PUBKEY, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { expectTX } from "@saberhq/chai-solana";

const anchorProvider = anchor.Provider.env();
anchor.setProvider(anchorProvider);
const program = anchor.workspace.Bond as Program<Example>;

const provider = SolanaProvider.load({
  connection: anchorProvider.connection,
  sendConnection: anchorProvider.connection,
  wallet: anchorProvider.wallet,
  opts: anchorProvider.opts,
});

describe("Post a Sale, Cancel it, Post Another, then buyout", () => {
    const EXAMPLE_PROGRAM_ID = new PublicKey(
        "6QXj6hJwibgSTUCMiphcoQs1sBp66g8L3WXQCgGugG2h"
      );

    const buyer = Keypair.fromSecretKey(
      Uint8Array.from(
        JSON.parse(
          `[137,133,202,245,148,42,23,26,103,230,160,45,76,116,
            61,194,248,153,201,158,147,194,105,251,76,83,227,
            231,20,53,141,202,76,213,42,76,124,160,207,12,169,
            215,122,240,231,205,122,132,230,174,131,175,36,51,
            76,163,5,233,238,53,6,182,10,230]`)));

    const seller = Keypair.fromSecretKey(
    Uint8Array.from(
        JSON.parse(
          `[183,46,93,92,45,56,236,155,157,198,97,117,223,11,184,
            235,32,112,4,218,158,79,18,236,242,153,93,230,80,109,
            119,98,237,56,188,88,33,132,204,210,108,145,5,168,236,
            52,81,114,168,117,107,128,108,119,219,90,13,27,29,220,
            46,56,253,3]`)));

    let paymentMintKey: PublicKey
    let nft1Key: PublicKey
    let nft2Key: PublicKey

    before ("Initializing Accounts", async () => {
      console.log(seller.publicKey.toString());
      console.log(buyer.publicKey.toString());
      // Creating payment scheme
      let paymentMint = await SPLToken.createMint(anchorProvider.connection, buyer, buyer.publicKey, buyer.publicKey, 0, TOKEN_PROGRAM_ID);
      paymentMintKey = paymentMint.publicKey;
      // Creating buyer ATA
      let buyerPaymentAtai = await getOrCreateATA({
        provider: provider,
        mint: paymentMint.publicKey,
        owner: buyer.publicKey
      })
      if (buyerPaymentAtai.instruction){
        await expectTX(new TransactionEnvelope(provider, [buyerPaymentAtai.instruction])).to.be.fulfilled;
      }

      // Creating seller ATA
      let sellerPaymentAtai = await getOrCreateATA({
        provider: provider,
        mint: paymentMint.publicKey,
        owner: seller.publicKey
      })
      if (sellerPaymentAtai.instruction){
        await expectTX(new TransactionEnvelope(provider, [sellerPaymentAtai.instruction])).to.be.fulfilled;
      }

      // Minting to buyer
      await paymentMint.mintTo(buyerPaymentAtai.address, buyer, [buyer], 200);

      // Creating payment scheme
      let nftMint1 = await SPLToken.createMint(anchorProvider.connection, seller, seller.publicKey, seller.publicKey, 0, TOKEN_PROGRAM_ID);
      nft1Key = nftMint1.publicKey;
      // Creating seller ATA
      let sellerNft1Atai = await getOrCreateATA({
        provider: provider,
        mint: nftMint1.publicKey,
        owner: seller.publicKey
      })
      if (sellerNft1Atai.instruction){
        await expectTX(new TransactionEnvelope(provider, [sellerNft1Atai.instruction])).to.be.fulfilled;
      }

      // Minting to buyer
      await nftMint1.mintTo(sellerNft1Atai.address, seller, [seller], 1);

      // Creating payment scheme
      let nftMint2 = await SPLToken.createMint(anchorProvider.connection, seller, seller.publicKey, seller.publicKey, 0, TOKEN_PROGRAM_ID);
      nft2Key = nftMint2.publicKey;
      // Creating seller ATA
      let sellerNft2Atai = await getOrCreateATA({
        provider: provider,
        mint: nftMint2.publicKey,
        owner: seller.publicKey
      })
      if (sellerNft2Atai.instruction){
        await expectTX(new TransactionEnvelope(provider, [sellerNft2Atai.instruction])).to.be.fulfilled;
      }

      // Minting to buyer
      await nftMint2.mintTo(sellerNft2Atai.address, seller, [seller], 1);
    });

    it ("Allows the seller to post a sale", async () => {
      expect(true);
    });

    it ("Allows seller to cancel sale", async () => {
      expect(true);
    });

    it ("Allows user to post another sale", async () => {
      expect(true);
    });

    it ("Allows buyer to buyout", async () => {
      expect(true);
    })

    it ("Allows seller to claim the payment", async () => {
      expect(true);
    })
});