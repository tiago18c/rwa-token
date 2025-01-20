import { type Idl, Program, type Provider } from "@coral-xyz/anchor";
import * as DataRegistryIdl from "../programs/idls/DataRegistry.json";
import { PublicKey } from "@solana/web3.js";
import { type DataRegistryIdlTypes } from "../programs/types";

export const dataRegistryProgramId = new PublicKey(
	"JDxF8P4SctW8rB5s5FbupPFYecKFqLSv7fTES3cFei3b"
);

/**
 * Returns the data registry program as a typed anchor program.
 * @param provider - Solana anchor provider.
 * @returns Typed solana program to be used for transaction building.
 */
export const getDataRegistryProgram = (provider: Provider) =>
  new Program(
    DataRegistryIdl as Idl,
    provider
  ) as unknown as Program<DataRegistryIdlTypes>;

/**
 * Retrieves the data registry pda public key for a specific asset mint.
 * @param assetMint - The string representation of the asset's mint address.
 * @returns The data registry pda.
 */
export const getDataRegistryPda = (assetMint: string) =>
	PublicKey.findProgramAddressSync(
		[new PublicKey(assetMint).toBuffer()],
		dataRegistryProgramId
	)[0];
