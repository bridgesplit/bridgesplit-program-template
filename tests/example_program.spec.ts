import * as anchor from "@project-serum/anchor";
import { Program, BN } from "@project-serum/anchor";
import { Example } from "../target/types/example";
import { SolanaProvider, TransactionEnvelope } from "@saberhq/solana-contrib";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getOrCreateATA, TOKEN_PROGRAM_ID, SPLToken, getTokenAccount } from "@saberhq/token-utils";
import { expect, use } from "chai";
import { Keypair, PublicKey, SystemProgram, SYSVAR_CLOCK_PUBKEY, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { TokenMetadataProgram } from "@metaplex-foundation/js";
import { chaiSolana, expectTX } from "@saberhq/chai-solana";

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
describe("Post a Sale, Cancel it, Post Another, then buyout", () => {
    const EXAMPLE_PROGRAM_ID = new PublicKey(
      "AE748h33zGqpyvoxAyGBKD1kgNH8u4TzxZHwsVkMxosk"
    );
    
    const BRIDGESPLIT_PROGRAM_ID = new PublicKey(
      "2qGyiNeWyZxNdkvWHc2jT5qkCnYa1j1gDLSSUmyoWMh8"
    )

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

    let paymentMintKey: PublicKey;
    let buyerPaymentAccountKey: PublicKey;

    let nft1Key: PublicKey;
    let nft2Key: PublicKey;

    let sellerNft1Key: PublicKey;
    let sellerNft2Key: PublicKey;

    let salesVault1: PublicKey;
    let salesVault2: PublicKey;

    let salesVault2Payment: PublicKey;

    let fractionsMint1: PublicKey;
    let sellerFractions1Account: PublicKey;
    let salesVaultFractions1Account: PublicKey;

    let fractionsMint2: PublicKey;
    let sellerFractions2Account: PublicKey;
    let salesVaultFractions2Account: PublicKey;

    const price = 100;
    const total_shares = 10000;
    const sold_shares = 5000;

    const uri: string = "https://rhlzpyjastylfvqnpzf4fjw5lrea3waof7ufcwb3xhsrob3r6m.arweave.net/ideX4SCU8LLWDX5LwqbdXEgN2A4v6FFYO7nlF-wdx8w";

    before ("Initializing Accounts", async () => {
      // Creating payment scheme
      let paymentMint = await SPLToken.createMint(anchorProvider.connection, buyer, buyer.publicKey, buyer.publicKey, 0, TOKEN_PROGRAM_ID);
      paymentMintKey = paymentMint.publicKey;
      // Creating buyer ATA
      let buyerPaymentAtai = await getOrCreateATA({
        provider: provider,
        mint: paymentMint.publicKey,
        owner: buyer.publicKey
      })
      buyerPaymentAccountKey = buyerPaymentAtai.address;
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
      sellerNft1Key = sellerNft1Atai.address;
      
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
      sellerNft2Key = sellerNft2Atai.address;
      if (sellerNft2Atai.instruction){
        await expectTX(new TransactionEnvelope(provider, [sellerNft2Atai.instruction])).to.be.fulfilled;
      }

      // Minting to seller
      await nftMint2.mintTo(sellerNft2Atai.address, seller, [seller], 1);
    });

    it ("Allows the seller to post a sale", async () => {
      const nonceKP = Keypair.generate();
      const nonce = nonceKP.publicKey;
      [fractionsMint1] = await PublicKey.findProgramAddress(
        [nonce.toBytes()],
        EXAMPLE_PROGRAM_ID
      );

      [salesVault1] = await PublicKey.findProgramAddress(
        [Buffer.from(anchor.utils.bytes.utf8.encode("sales_vault")), fractionsMint1.toBytes()],
        EXAMPLE_PROGRAM_ID
      );

      const [bridgesplitVault] = await PublicKey.findProgramAddress(
        [Buffer.from(anchor.utils.bytes.utf8.encode("vault")), fractionsMint1.toBytes()],
        BRIDGESPLIT_PROGRAM_ID
      );

      let sellerFractions1Atai = await getOrCreateATA({
        provider: provider,
        mint: fractionsMint1,
        owner: seller.publicKey
      });
      sellerFractions1Account = sellerFractions1Atai.address;

      let salesVaultFractions1Atai = await getOrCreateATA({
        provider: provider,
        mint: fractionsMint1,
        owner: salesVault1
      });
      salesVaultFractions1Account = salesVaultFractions1Atai.address;

      let bsVaultNftAccount = await getOrCreateATA({
        provider: provider,
        mint: nft1Key,
        owner: bridgesplitVault
      });

      const [fractionsMetadataAccount] = await PublicKey.findProgramAddress(
        [Buffer.from("metadata"), TokenMetadataProgram.publicKey.toBuffer(), fractionsMint1.toBuffer()],
        TokenMetadataProgram.publicKey
      );
      
      let postIxn = program.instruction.postSale(new BN(price), nonce, new BN(sold_shares), new BN(total_shares), uri, 'Example Nft', 'xNFT', {
        accounts: {
          seller: seller.publicKey,
          salesVault: salesVault1,
          paymentMint: paymentMintKey,
          bridgesplitVault: bridgesplitVault,
          nftMint: nft1Key,
          sellerNftAccount: sellerNft1Key,
          bsVaultNftAccount: bsVaultNftAccount.address,
          fractionsMint: fractionsMint1,
          sellerFractionsAccount: sellerFractions1Account,
          salesVaultFractionsAccount: salesVaultFractions1Account,
          bridgesplitProgram: BRIDGESPLIT_PROGRAM_ID,
          mplTokenMetadata: TokenMetadataProgram.publicKey,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          metadataAccount: fractionsMetadataAccount,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY,
          clock: SYSVAR_CLOCK_PUBKEY
        }
      });

      await expectTX(new TransactionEnvelope(provider, [postIxn], [seller])).to.be.fulfilled;

      let bsVaultNftTokAccount = await getTokenAccount(provider, bsVaultNftAccount.address);
      let bsVaultNftTokAccountAmount = bsVaultNftTokAccount.amount.toNumber();
      expect(bsVaultNftTokAccountAmount == 1, "BS Vault didn't get NFT");

      let sellerNftTokAccount = await getTokenAccount(provider, sellerNft1Key);
      let sellerNftTokAccountAmount = sellerNftTokAccount.amount.toNumber();
      expect(sellerNftTokAccountAmount == 0, "Seller Kept their NFT");

      let salesVaultFractionsTokAccount = await getTokenAccount(provider, salesVaultFractions1Account);
      let salesVaultFractionsTokAccountAmount = salesVaultFractionsTokAccount.amount.toNumber();
      expect(salesVaultFractionsTokAccountAmount == sold_shares, "Sales Vault didn't get Fractions");

      let sellerFractionsTokAccount = await getTokenAccount(provider, sellerFractions1Account);
      let sellerFractionsTokAccountAmount = sellerFractionsTokAccount.amount.toNumber();
      expect(sellerFractionsTokAccountAmount == total_shares - sold_shares, "Seller has correct amount of Fractions");
    });

    it ("Allows seller to cancel sale", async () => {
      let cancelIxn = program.instruction.cancelSale({
        accounts: {
          seller: seller.publicKey,
          salesVault: salesVault1,
          fractionsMint: fractionsMint1,
          sellerFractionsAccount: sellerFractions1Account,
          salesVaultFractionsAccount: salesVaultFractions1Account,
          tokenProgram: TOKEN_PROGRAM_ID
        }
      });
      await expectTX(new TransactionEnvelope(provider, [cancelIxn], [seller])).to.be.fulfilled;

      let salesVaultFractionsTokAccount = await getTokenAccount(provider, salesVaultFractions1Account);
      let salesVaultFractionsTokAccountAmount = salesVaultFractionsTokAccount.amount.toNumber();
      expect(salesVaultFractionsTokAccountAmount == 0, "Sales vault kept fractions");

      let sellerFractionsTokAccount = await getTokenAccount(provider, sellerFractions1Account);
      let sellerFractionsTokAccountAmount = sellerFractionsTokAccount.amount.toNumber();
      expect(sellerFractionsTokAccountAmount == total_shares, "Seller regained fractions");
    });

    it ("Allows user to post another sale", async () => {
      const nonceKP = Keypair.generate();
      const nonce = nonceKP.publicKey;
      [fractionsMint2] = await PublicKey.findProgramAddress(
        [nonce.toBytes()],
        EXAMPLE_PROGRAM_ID
      );

      [salesVault2] = await PublicKey.findProgramAddress(
        [Buffer.from(anchor.utils.bytes.utf8.encode("sales_vault")), fractionsMint2.toBytes()],
        EXAMPLE_PROGRAM_ID
      );

      const [bridgesplitVault] = await PublicKey.findProgramAddress(
        [Buffer.from(anchor.utils.bytes.utf8.encode("vault")), fractionsMint2.toBytes()],
        BRIDGESPLIT_PROGRAM_ID
      );

      let sellerFractions2Atai = await getOrCreateATA({
        provider: provider,
        mint: fractionsMint1,
        owner: seller.publicKey
      });
      sellerFractions2Account = sellerFractions2Atai.address;

      let salesVaultFractions2Atai = await getOrCreateATA({
        provider: provider,
        mint: fractionsMint1,
        owner: salesVault1
      });
      salesVaultFractions1Account = salesVaultFractions2Atai.address;

      let bsVaultNftAccount = await getOrCreateATA({
        provider: provider,
        mint: nft1Key,
        owner: bridgesplitVault
      });

      const [fractionsMetadataAccount] = await PublicKey.findProgramAddress(
        [Buffer.from("metadata"), TokenMetadataProgram.publicKey.toBuffer(), fractionsMint2.toBuffer()],
        TokenMetadataProgram.publicKey
      );
      
      let postIxn = program.instruction.postSale(new BN(50), nonce, new BN(500), new BN(1000), uri, 'Example Nft 2', 'xNFT2', {
        accounts: {
          seller: seller.publicKey,
          salesVault: salesVault2,
          paymentMint: paymentMintKey,
          bridgesplitVault: bridgesplitVault,
          nftMint: nft2Key,
          sellerNftAccount: sellerNft2Key,
          bsVaultNftAccount: bsVaultNftAccount.address,
          fractionsMint: fractionsMint2,
          sellerFractionsAccount: sellerFractions2Account,
          salesVaultFractionsAccount: salesVaultFractions2Account,
          bridgesplitProgram: BRIDGESPLIT_PROGRAM_ID,
          mplTokenMetadata: TokenMetadataProgram.publicKey,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          metadataAccount: fractionsMetadataAccount,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY,
          clock: SYSVAR_CLOCK_PUBKEY
        }
      })
      await expectTX(new TransactionEnvelope(provider, [postIxn], [seller])).to.be.fulfilled;

      let bsVaultNftTokAccount = await getTokenAccount(provider, bsVaultNftAccount.address);
      let bsVaultNftTokAccountAmount = bsVaultNftTokAccount.amount.toNumber();
      expect(bsVaultNftTokAccountAmount == 1, "BS Vault didn't get NFT");

      let sellerNftTokAccount = await getTokenAccount(provider, sellerNft2Key);
      let sellerNftTokAccountAmount = sellerNftTokAccount.amount.toNumber();
      expect(sellerNftTokAccountAmount == 0, "Seller Kept their NFT");

      let salesVaultFractionsTokAccount = await getTokenAccount(provider, salesVaultFractions2Account);
      let salesVaultFractionsTokAccountAmount = salesVaultFractionsTokAccount.amount.toNumber();
      expect(salesVaultFractionsTokAccountAmount == sold_shares, "Sales Vault didn't get fractions");

      let sellerFractionsTokAccount = await getTokenAccount(provider, sellerFractions2Account);
      let sellerFractionsTokAccountAmount = sellerFractionsTokAccount.amount.toNumber();
      expect(sellerFractionsTokAccountAmount == total_shares - sold_shares, "Seller kept incorrect fractions");
    });

    it ("Allows buyer to buyout", async () => {
      let buyerFractionsAtai = await getOrCreateATA({
        provider: provider,
        mint: fractionsMint2,
        owner: buyer.publicKey
      })
      sellerNft2Key = buyerFractionsAtai.address;
      if (buyerFractionsAtai.instruction){
        await expectTX(new TransactionEnvelope(provider, [buyerFractionsAtai.instruction])).to.be.fulfilled;
      }

      let salesVault2PaymentAtai = await getOrCreateATA({
        provider: provider,
        mint: paymentMintKey,
        owner: salesVault2
      })
      salesVault2Payment = salesVault2PaymentAtai.address;
      if (salesVault2PaymentAtai.instruction){
        await expectTX(new TransactionEnvelope(provider, [buyerFractionsAtai.instruction])).to.be.fulfilled;
      }


      let buyoutIxn = program.instruction.buyout({
        accounts: {
          buyer: buyer.publicKey,
          paymentMint: paymentMintKey,
          fractionsMint: fractionsMint2,
          salesVault: salesVault2,
          buyerFractionsAccount: buyerFractionsAtai.address,
          salesVaultFractionsAccount: salesVaultFractions2Account,
          buyerPaymentAccount: buyerPaymentAccountKey,
          salesVaultPaymentAccount: salesVault2Payment,
          tokenProgram: TOKEN_PROGRAM_ID
        }
      })
      await expectTX(new TransactionEnvelope(provider, [buyoutIxn], [buyer])).to.be.fulfilled;

      let salesVaultPaymentTokAccount = await getTokenAccount(provider, salesVault2Payment);
      let salesVaultPaymentTokAccountAmount = salesVaultPaymentTokAccount.amount.toNumber();
      expect(salesVaultPaymentTokAccountAmount == price, "Sales Vault didn't get Payment");

      let buyerFractionsTokAccount = await getTokenAccount(provider, sellerFractions2Account);
      let buyerFractionsTokAccountAmount = buyerFractionsTokAccount.amount.toNumber();
      expect(buyerFractionsTokAccountAmount == sold_shares, "Buyer didn't get fractions");
    })

    it ("Allows seller to claim the payment", async () => {
      let sellerPaymentAtai = await getOrCreateATA({
        provider: provider,
        mint: paymentMintKey,
        owner: seller.publicKey
      })
      if (sellerPaymentAtai.instruction){
        await expectTX(new TransactionEnvelope(provider, [sellerPaymentAtai.instruction])).to.be.fulfilled;
      }

      let claimIxn = program.instruction.claimBuyout({
        accounts: {
          seller: seller.publicKey,
          salesVault: salesVault2,
          fractionsMint: fractionsMint2,
          paymentMint: paymentMintKey,
          sellerPaymentAccount: sellerPaymentAtai.address,
          salesVaultPaymentAccount: salesVault2Payment,
          tokenProgram: TOKEN_PROGRAM_ID
        }
      })
      await expectTX(new TransactionEnvelope(provider, [claimIxn], [buyer])).to.be.fulfilled;

      let salesVaultPaymentTokAccount = await getTokenAccount(provider, salesVault2Payment);
      let salesVaultPaymentTokAccountAmount = salesVaultPaymentTokAccount.amount.toNumber();
      expect(salesVaultPaymentTokAccountAmount == 0, "Sales vault held payment");

      let sellerPaymentTokAccount = await getTokenAccount(provider, sellerPaymentAtai.address);
      let sellerPaymentTokAccountAmount = sellerPaymentTokAccount.amount.toNumber();
      expect(sellerPaymentTokAccountAmount == price, "Seller got incorrect payment");
    })
});