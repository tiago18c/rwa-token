import {
	PublicKey,
	SystemProgram,
	type TransactionInstruction,
} from "@solana/web3.js";
import { type CommonArgs } from "../utils";
import {
	getIdentityAccountPda,
	getIdentityRegistryProgram,
	getIdentityRegistryPda,
	getWalletIdentityAccountPda,
} from "./utils";
import { BN, type AnchorProvider } from "@coral-xyz/anchor";
import { getPolicyEnginePda, getTrackerAccountPda, policyEngineProgramId } from "../policy-engine/utils";

/** Represents arguments for creating an on identity registry on chain. */
export type CreateIdentityRegistryArgs = {
  authority: string;
  signer: string;
  requireIdentityCreation?: boolean;
} & CommonArgs;

/**
 * Builds the transaction instruction to create an identity registry.
 * @param args - {@link CreateIdentityRegistryArgs}.
 * @returns Create identity registry transaction instruction.
 */
export async function getCreateIdentityRegistryIx(
	args: CreateIdentityRegistryArgs,
	provider: AnchorProvider
): Promise<TransactionInstruction> {
	const identityProgram = getIdentityRegistryProgram(provider);
	const ix = await identityProgram.methods
		.createIdentityRegistry(
			new PublicKey(args.authority),
			args.delegate ? new PublicKey(args.delegate) : null,
			args.requireIdentityCreation ? args.requireIdentityCreation : null
		)
		.accountsStrict({
			payer: args.payer,
			signer: args.signer,
			assetMint: args.assetMint,
			identityRegistryAccount: getIdentityRegistryPda(args.assetMint),
			systemProgram: SystemProgram.programId,
		})
		.instruction();
	return ix;
}

/** Represents arguments for creating an identity account on chain. */
export type CreateIdentityAccountArgs = {
  level: number;
  owner: string;
  expiry: BN;
  country: number;
} & CommonArgs;

/**
 * Builds the transaction instruction to create an identity account.
 * @param args - {@link CreateIdentityAccountArgs}.
 * @returns Create identity account transaction instruction.
 */
export async function getCreateIdentityAccountIx(
	args: CreateIdentityAccountArgs,
	provider: AnchorProvider
): Promise<TransactionInstruction> {
	const identityProgram = getIdentityRegistryProgram(provider);
	const ix = await identityProgram.methods
		.createIdentityAccount(new PublicKey(args.owner), args.level, args.expiry, args.country)
		.accountsStrict({
			payer: args.payer,
			identityRegistry: getIdentityRegistryPda(args.assetMint),
			identityAccount: getIdentityAccountPda(args.assetMint, args.owner),
			systemProgram: SystemProgram.programId,
			signer: args.signer
				? args.signer
				: getIdentityRegistryPda(args.assetMint),
		})
		.instruction();
	return ix;
}

/** Represents the arguments required to attach a wallet to an identity account. */
export type AttachWalletToIdentityArgs = {
  wallet: string;
  owner: string;
} & CommonArgs;

export async function getAttachWalletToIdentityIx(
	args: AttachWalletToIdentityArgs,
	provider: AnchorProvider
): Promise<TransactionInstruction> {
	const identityProgram = getIdentityRegistryProgram(provider);
	const ix = await identityProgram.methods
		.attachWalletToIdentity(new PublicKey(args.wallet))
		.accountsStrict({
			payer: args.payer,
			authority: args.authority,
			assetMint: args.assetMint,
			identityAccount: getIdentityAccountPda(args.assetMint, args.owner),
			identityRegistry: getIdentityRegistryPda(args.assetMint),
			walletIdentity: getWalletIdentityAccountPda(args.assetMint, args.wallet),
			systemProgram: SystemProgram.programId,
			newWalletIdentityAccount: getIdentityAccountPda(args.assetMint, args.wallet),
		})
		.instruction();
	return ix;
}

export async function getDetachWalletFromIdentityIx(
	args: AttachWalletToIdentityArgs,
	provider: AnchorProvider
): Promise<TransactionInstruction> {
	const identityProgram = getIdentityRegistryProgram(provider);
	const ix = await identityProgram.methods
		.detachWalletFromIdentity()
		.accountsStrict({
			payer: args.payer,
			authority: args.authority,
			walletIdentity: getWalletIdentityAccountPda(args.assetMint, args.wallet),
			identityAccount: getIdentityAccountPda(args.assetMint, args.owner),
			identityRegistry: getIdentityRegistryPda(args.assetMint),
		})
		.instruction();
	return ix;
}

/** Represents arguments for creating an identity account on chain. */
export type ChangeCountryArgs = {
	owner: string;
	country: number;
	ignorePolicy?: boolean;
  } & CommonArgs;
  
/**
 * Builds the transaction instruction to add a level to identity account
 * @param args - {@link AddLevelToIdentityAccountArgs}.
 * @returns Add level to identity account transaction instruction.
 */
export async function getChangeCountryIx(
	args: ChangeCountryArgs,
	provider: AnchorProvider
): Promise<TransactionInstruction> {
	const identityProgram = getIdentityRegistryProgram(provider);
	const ix = await identityProgram.methods
		.changeCountry(args.country, args.ignorePolicy ?? false)
		.accountsStrict({
			signer: args.signer,
			identityRegistry: getIdentityRegistryPda(args.assetMint),
			identityAccount: getIdentityAccountPda(args.assetMint, args.owner),
			payer: args.payer,
			policyEngineProgram: policyEngineProgramId,
			policyEngine: getPolicyEnginePda(args.assetMint),
			trackerAccount: getTrackerAccountPda(args.assetMint, args.owner),
			assetMint: args.assetMint,
		})
		.instruction();
	return ix;
}

/** Represents the arguments required to add a level to an identity account. */
export type AddLevelToIdentityAccountArgs = {
  owner: string;
  levels: number[];
  expiries: BN[];
  signer: string;
  ignorePolicy?: boolean;
} & CommonArgs;

/**
 * Builds the transaction instruction to add a level to identity account
 * @param args - {@link AddLevelToIdentityAccountArgs}.
 * @returns Add level to identity account transaction instruction.
 */
export async function getAddLevelToIdentityAccount(
	args: AddLevelToIdentityAccountArgs,
	provider: AnchorProvider
): Promise<TransactionInstruction> {
	const identityProgram = getIdentityRegistryProgram(provider);
	const ix = await identityProgram.methods
		.addLevelToIdentityAccount(Buffer.from(args.levels), args.expiries, args.ignorePolicy ?? false)
		.accountsStrict({
			signer: args.signer,
			identityRegistry: getIdentityRegistryPda(args.assetMint),
			identityAccount: getIdentityAccountPda(args.assetMint, args.owner),
			payer: args.payer,
			systemProgram: SystemProgram.programId,
			policyEngineProgram: policyEngineProgramId,
			policyEngine: getPolicyEnginePda(args.assetMint),
			trackerAccount: getTrackerAccountPda(args.assetMint, args.owner),
			assetMint: args.assetMint,
		})
		.instruction();
	return ix;
}

export type RemoveLevelFromIdentityAccountArgs = {
  owner: string;
  levels: number[];
  signer: string;
  enforceLimits?: boolean;
} & CommonArgs;

/**
 * Builds the transaction instruction to remove a level from identity account
 * @param args - {@link RemoveLevelFromIdentityAccount}.
 * @returns Add level to identity account transaction instruction.
 */
export async function getRemoveLevelFromIdentityAccount(
	args: RemoveLevelFromIdentityAccountArgs,
	provider: AnchorProvider
): Promise<TransactionInstruction> {
	const identityProgram = getIdentityRegistryProgram(provider);
	const ix = await identityProgram.methods
		.removeLevelFromIdentityAccount(Buffer.from(args.levels), args.enforceLimits ?? false)
		.accountsStrict({
			signer: args.signer
				? args.signer
				: getIdentityRegistryPda(args.assetMint),
			identityRegistry: getIdentityRegistryPda(args.assetMint),
			identityAccount: getIdentityAccountPda(args.assetMint, args.owner),
			payer: args.signer,
			systemProgram: SystemProgram.programId,
			policyEngineProgram: policyEngineProgramId,
			policyEngine: getPolicyEnginePda(args.assetMint),
			trackerAccount: getTrackerAccountPda(args.assetMint, args.owner),
			assetMint: args.assetMint,
		})
		.instruction();
	return ix;
}
