import { type PublicKey, type TransactionInstruction } from "@solana/web3.js";
import { type SetupUserArgs, getSetupUserIxs } from "../asset-controller";
import { type IxReturn } from "../utils";
import {
	type AddLevelToIdentityAccountArgs,
	type RemoveLevelFromIdentityAccountArgs,
	getAddLevelToIdentityAccount,
	getRemoveLevelFromIdentityAccount,
	getIdentityRegistryPda,
	getIdentityAccountPda,
	AttachWalletToIdentityArgs,
	getAttachWalletToIdentityIx,
	getDetachWalletFromIdentityIx,
	ChangeCountryArgs,
	getChangeCountryIx,
	RevokeIdentityAccountArgs,
	getRevokeIdentityAccountIx,
} from "../identity-registry";
import { type RwaClient } from "./Client";

/**
 * Represents the client for Identity Registry for an RWA.
 */
export class IdentityRegistry {
	private readonly rwaClient: RwaClient;

	constructor(rwaClient: RwaClient) {
		this.rwaClient = rwaClient;
	}

	/**
   * Asynchronously generates instructions to setup a user.
   * @param - {@link SetupUserArgs}
   * @returns A Promise that resolves to the instructions to setup a user.
   *
   * It is required for at least a single user to be setup before issuing tokens.
   */
	async setupUserIxns(setupUserArgs: SetupUserArgs): Promise<IxReturn> {
		const setupUserIx = await getSetupUserIxs(
			setupUserArgs,
			this.rwaClient.provider
		);
		return setupUserIx;
	}

	async attachWalletToIdentity(attachWalletArgs: AttachWalletToIdentityArgs): Promise<TransactionInstruction> {
		const attachWalletIx = await getAttachWalletToIdentityIx(attachWalletArgs, this.rwaClient.provider);
		return attachWalletIx;
	}

	async detachWalletFromIdentity(detachWalletArgs: AttachWalletToIdentityArgs): Promise<TransactionInstruction> {
		const detachWalletIx = await getDetachWalletFromIdentityIx(detachWalletArgs, this.rwaClient.provider);
		return detachWalletIx;
	}

	async changeCountry(changeCountryArgs: ChangeCountryArgs): Promise<TransactionInstruction> {
		const changeCountryIx = await getChangeCountryIx(changeCountryArgs, this.rwaClient.provider);
		return changeCountryIx;
	}

	/**
   * Asynchronously update user account identity
   * @param - {@link AddLevelToIdentityAccountArgs}
   * @returns A Promise that resolves to the instructions to update user account identity.
   * */
	async addIdentityLevelToUserAccount(
		addLevelArgs: AddLevelToIdentityAccountArgs
	): Promise<TransactionInstruction> {
		const addLevelIx = await getAddLevelToIdentityAccount(
			addLevelArgs,
			this.rwaClient.provider
		);
		return addLevelIx;
	}

	/**
   * Asynchronously reduces a user identity account level
   * @param - {@link RemoveLevelFromIdentityAccount}
   * @returns A Promise that resolves to the instructions to reduce the level of a user identity account.
   */
	async removeIdentityLevelFromUserAccount(
		removeLevelArgs: RemoveLevelFromIdentityAccountArgs
	): Promise<TransactionInstruction> {
		const reduceLevelIx = await getRemoveLevelFromIdentityAccount(
			removeLevelArgs,
			this.rwaClient.provider
		);
		return reduceLevelIx;
	}

	/**
   * Asynchronously revokes an identity account
   * @param - {@link RevokeIdentityAccountArgs}
   * @returns A Promise that resolves to the instructions to revoke an identity account.
   */
	async revokeIdentityAccount(revokeIdentityAccountArgs: RevokeIdentityAccountArgs): Promise<TransactionInstruction> {
		const revokeIdentityAccountIx = await getRevokeIdentityAccountIx(revokeIdentityAccountArgs, this.rwaClient.provider);
		return revokeIdentityAccountIx;
	}

	/**
   * Retrieves the identity registry pda account for a specific asset mint.
   * @param assetMint - The string representation of the asset's mint address.
   * @returns The identity registry pda as a public key.
   */
	getIdentityRegistryPda(assetMint: string): PublicKey {
		return getIdentityRegistryPda(assetMint);
	}

	/**
   * Retrieves the identity account pda public key for a specific asset mint.
   * @param assetMint - The string representation of the asset's mint address.
   * @param owner - The string representation of the asset's owner.
   * @returns The identity account pda.
   */
	getIdentityAccountPda(assetMint: string, owner: string): PublicKey {
		return getIdentityAccountPda(assetMint, owner);
	}
}
