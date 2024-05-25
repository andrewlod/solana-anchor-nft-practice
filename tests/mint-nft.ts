import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MintNft } from "../target/types/mint_nft";
import { MPL_TOKEN_METADATA_PROGRAM_ID } from '@metaplex-foundation/mpl-token-metadata';
import { associatedAddress } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { getLocalAccount } from "./util";
import { ComputeBudgetProgram, Transaction, sendAndConfirmTransaction } from "@solana/web3.js";

describe("mint-nft", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const wallet = provider.wallet as anchor.Wallet;
  anchor.setProvider(provider);

  const program = anchor.workspace.MintNft as Program<MintNft>;
  const nftTitle = 'Andre Wlod';
  const nftSymbol = 'WLOD';
  const nftUri = 'https://raw.githubusercontent.com/andrewlod/solana-anchor-nft-practice/main/assets/metadata.json';

  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(MPL_TOKEN_METADATA_PROGRAM_ID.toString());

  it("Is initialized!", async () => {
    const walletKeypair = await getLocalAccount();

    const mintKeypair = anchor.web3.Keypair.generate();
    const tokenAddress = associatedAddress({
      mint: mintKeypair.publicKey,
      owner: wallet.publicKey
    });
    console.log(`New token: ${mintKeypair.publicKey}`);

    const [metadataAddress] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mintKeypair.publicKey.toBuffer()
      ],
      TOKEN_METADATA_PROGRAM_ID
    );
    console.log(`Metadata address: ${metadataAddress}`);

    const [masterEditionAddress] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mintKeypair.publicKey.toBuffer(),
        Buffer.from("edition")
      ],
      TOKEN_METADATA_PROGRAM_ID
    );
    console.log(`Master Edition address: ${masterEditionAddress}`);

    const modifyComputeUnits = ComputeBudgetProgram.setComputeUnitLimit({
      units: 400000
    });

    const addPriorityFee = ComputeBudgetProgram.setComputeUnitPrice({
      microLamports: 1
    });

    let mintInstruction = await program.methods.mint(
      nftTitle, nftSymbol, nftUri
    )
      .accounts({
        metadata: metadataAddress,
        masterEdition: masterEditionAddress,
        mint: mintKeypair.publicKey,
        tokenAccount: tokenAddress,
        mintAuthority: walletKeypair.publicKey,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID
      })
      .signers([mintKeypair, walletKeypair])
      .instruction();

    let tx = new Transaction();
    tx.add(modifyComputeUnits);
    tx.add(addPriorityFee);
    tx.add(mintInstruction);

    await sendAndConfirmTransaction(provider.connection, tx, [walletKeypair, mintKeypair]);
  });
});
