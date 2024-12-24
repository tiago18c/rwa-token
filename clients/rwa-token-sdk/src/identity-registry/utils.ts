import { type Idl, Program, type Provider } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { type IdentityRegistryIdlTypes } from "../programs/types";

import * as IdentityRegistryIdl from "../programs/idls/IdentityRegistry.json";

/** Address of the identity registry program. */
export const identityRegistryProgramId = new PublicKey("7Zis9Cg1pa3PMRCMfJBgzoQThoBA21QMrkmEnx6nZdQQ");

export const getIdentityRegistryProgram = (provider: Provider) => new Program(
	IdentityRegistryIdl as Idl,
	provider,
) as unknown as Program<IdentityRegistryIdlTypes>;

/**
 * Retrieves the identity registry pda public key for a specific asset mint.
 * @param assetMint - The string representation of the asset's mint address.
 * @returns The identity registry pda.
 */
export const getIdentityRegistryPda = (assetMint: string) => PublicKey.findProgramAddressSync(
	[new PublicKey(assetMint).toBuffer()],
	identityRegistryProgramId,
)[0];

/**
 * Retrieves the identity account pda public key for a specific asset mint.
 * @param assetMint - The string representation of the asset's mint address.
 * @param owner - The string representation of the asset's owner.
 * @returns The identity account pda.
 */
export const getIdentityAccountPda = (assetMint: string, owner: string) => PublicKey.findProgramAddressSync(
	[getIdentityRegistryPda(assetMint).toBuffer(), new PublicKey(owner).toBuffer()],
	identityRegistryProgramId,
)[0];


/**
 * Retrieves the wallet identity account pda public key for a specific asset mint.
 * @param assetMint - The string representation of the asset's mint address.
 * @param wallet - The string representation of the wallet address.
 * @returns The identity account pda.
 */
export const getWalletIdentityAccountPda = (assetMint: string, wallet: string) => PublicKey.findProgramAddressSync(
	[new PublicKey(wallet).toBuffer(), new PublicKey(assetMint).toBuffer()],
	identityRegistryProgramId,
)[0];

export const POLICY_SKIP_LEVEL = 255;