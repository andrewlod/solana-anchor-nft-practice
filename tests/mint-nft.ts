import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MintNft } from "../target/types/mint_nft";
import { MPL_TOKEN_METADATA_PROGRAM_ID } from '@metaplex-foundation/mpl-token-metadata';
import { associatedAddress } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { getLocalAccount } from "./util";

describe("mint-nft", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const wallet = provider.wallet as anchor.Wallet;
  anchor.setProvider(provider);

  const program = anchor.workspace.MintNft as Program<MintNft>;
  const nftTitle = 'Andre Wlod';
  const nftSymbol = 'WLOD';
  const nftUri = 'https://media.licdn.com/dms/image/C4D03AQHqrn4tBjRtyA/profile-displayphoto-shrink_800_800/0/1600261790603?e=1721865600&v=beta&t=cG4vbDZLQgjN8iLMNJvEADrqYuBvnJqtwGn3k2zm37o';

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

    await program.methods.mint(
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
      .rpc()
      .catch(err => console.log(err));
  });
});
