import {
	PublicKey,
	SystemProgram,
	type TransactionInstruction,
} from "@solana/web3.js";
import { type CommonArgs, type IxReturn } from "../utils";
import {
	getExtraMetasListPda,
	getPolicyEnginePda,
	getPolicyEngineProgram,
	getPolicyEnginerEventAuthority,
	getTrackerAccountPda,
} from "./utils";
import { type PolicyType, type IdentityFilter, Counter, CounterLimit } from "./types";
import { type AnchorProvider } from "@coral-xyz/anchor";
import { getIdentityAccountPda, getIdentityRegistryPda } from "../identity-registry";

/** Represents the arguments required to create a policy engine account. */
export type CreatePolicyEngineArgs = {
  authority: string;
  signer: string;
  enforcePolicyIssuance: boolean;
} & CommonArgs;

/**
 * Generate instructions to create a new policy account.
 * @param args {@link CreatePolicyEngineArgs}.
 * @returns Create policy engine transaction instruction.
 */
export async function getCreatePolicyEngineIx(
	args: CreatePolicyEngineArgs,
	provider: AnchorProvider
): Promise<TransactionInstruction> {
	const policyProgram = getPolicyEngineProgram(provider);
	const ix = await policyProgram.methods
		.createPolicyEngine(
			new PublicKey(args.authority),
			args.delegate ? new PublicKey(args.delegate) : null,
			args.enforcePolicyIssuance
		)
		.accountsStrict({
			payer: args.payer,
			signer: args.signer,
			assetMint: args.assetMint,
			policyEngineAccount: getPolicyEnginePda(args.assetMint),
			extraMetasAccount: getExtraMetasListPda(args.assetMint),
			systemProgram: SystemProgram.programId,
		})
		.instruction();
	return ix;
}

/** Represents the arguments required to attach a policy to an asset. */
export type AttachPolicyArgs = {
  authority: string;
  assetMint: string;
  payer: string;
  identityFilter: IdentityFilter;
  policyType: PolicyType;
};

/** Represents the arguments required to detach a policy from an asset. */
export type DetachPolicyArgs = {
	authority: string;
	assetMint: string;
	payer: string;
	hash: string;
  };

/**
 * Generate instructions to connect am policy to an asset.
 *
 * This function constructs an instruction to attach a policy account to an asset
 * using the provided arguments. It calls the policy engine program to attach the policy account,
 * and returns the generated instruction along with the required signers.
 *
 * @param args {@link AttachPolicyArgs}
 * @returns - {@link IxReturn}, a list of transaction instructions and a new key pair responsible to sign it.
 */
export async function getAttachToPolicyEngineIx(
	args: AttachPolicyArgs,
	provider: AnchorProvider
): Promise<IxReturn> {
	const policyProgram = getPolicyEngineProgram(provider);
	const ix = await policyProgram.methods
		.attachToPolicyEngine(args.identityFilter, args.policyType)
		.accountsStrict({
			signer: new PublicKey(args.authority),
			payer: args.payer,
			policyEngine: getPolicyEnginePda(args.assetMint),
			systemProgram: SystemProgram.programId,
		})
		.instruction();
	return {
		ixs: [ix],
		signers: [],
	};
}

export type ChangeMappingArgs = {
	authority: string;
	payer: string;
	assetMint: string;
	mappingSource: number[];
	mappingValue: number[];
}

export async function getChangeMappingIx(
	args: ChangeMappingArgs,
	provider: AnchorProvider
): Promise<IxReturn> {
	const policyProgram = getPolicyEngineProgram(provider);
	const ix = await policyProgram.methods
		.changeMapping(Buffer.from(args.mappingSource), Buffer.from(args.mappingValue))
		.accountsStrict({
			signer: new PublicKey(args.authority),
			payer: args.payer,
			policyEngine: getPolicyEnginePda(args.assetMint),
		})
		.instruction();
	return {
		ixs: [ix],
		signers: [],
	};
}

export type ChangeCountersArgs = {
	authority: string;
	payer: string;
	assetMint: string;
	removedCounters: Buffer;
	addedCounters: Counter[];
}

/**
 * Generate instructions to connect am policy to an asset.
 *
 * This function constructs an instruction to attach a policy account to an asset
 * using the provided arguments. It calls the policy engine program to attach the policy account,
 * and returns the generated instruction along with the required signers.
 *
 * @param args {@link AttachPolicyArgs}
 * @returns - {@link IxReturn}, a list of transaction instructions and a new key pair responsible to sign it.
 */
export async function getChangeCountersIx(
	args: ChangeCountersArgs,
	provider: AnchorProvider
): Promise<IxReturn> {
	const policyProgram = getPolicyEngineProgram(provider);
	const ix = await policyProgram.methods
		.changeCounters(args.removedCounters, args.addedCounters)
		.accountsStrict({
			signer: new PublicKey(args.authority),
			payer: args.payer,
			policyEngine: getPolicyEnginePda(args.assetMint),
			systemProgram: SystemProgram.programId,
		})
		.instruction();
	return {
		ixs: [ix],
		signers: [],
	};
}

export type ChangeCounterLimitsArgs = {
	authority: string;
	payer: string;
	assetMint: string;
	removedCounterLimits: Buffer;
	addedCounterLimits: CounterLimit[];
}


/**
 * Generate instructions to connect am policy to an asset.
 *
 * This function constructs an instruction to attach a policy account to an asset
 * using the provided arguments. It calls the policy engine program to attach the policy account,
 * and returns the generated instruction along with the required signers.
 *
 * @param args {@link AttachPolicyArgs}
 * @returns - {@link IxReturn}, a list of transaction instructions and a new key pair responsible to sign it.
 */
export async function getChangeCounterLimitsIx(
	args: ChangeCounterLimitsArgs,
	provider: AnchorProvider
): Promise<IxReturn> {
	const policyProgram = getPolicyEngineProgram(provider);
	const ix = await policyProgram.methods
		.changeCounterLimits(args.removedCounterLimits, args.addedCounterLimits)
		.accountsStrict({
			signer: new PublicKey(args.authority),
			payer: args.payer,
			policyEngine: getPolicyEnginePda(args.assetMint),
			systemProgram: SystemProgram.programId,
		})
		.instruction();
	return {
		ixs: [ix],
		signers: [],
	};
}

/**
 * Generate instructions to detach an identity policy account to an asset.
 *
 * This function constructs an instruction to attach a policy account to an asset
 * using the provided arguments. It creates a new policy account, calls the policy
 * engine program to attach the policy account, and returns the generated instruction
 * along with the required signers.
 *
 * @param args {@link AttachPolicyArgs}
 * @returns - {@link IxReturn}, a list of transaction instructions and a new key pair responsible to sign it.
 */
export async function getDetachFromPolicyEngineIx(
	args: DetachPolicyArgs,
	provider: AnchorProvider
): Promise<IxReturn> {
	const policyProgram = getPolicyEngineProgram(provider);
	const ix = await policyProgram.methods
		.detachFromPolicyEngine(args.hash)
		.accountsStrict({
			signer: new PublicKey(args.authority),
			payer: args.payer,
			policyEngine: getPolicyEnginePda(args.assetMint),
			systemProgram: SystemProgram.programId,
		})
		.instruction();
	return {
		ixs: [ix],
		signers: [],
	};
}

export interface CreateTrackerAccountArgs {
	payer: string;
	owner: string;
	assetMint: string;
}

export async function getCreateTrackerAccountIx(
	args: CreateTrackerAccountArgs,
	provider: AnchorProvider
): Promise<TransactionInstruction> {
	const policyProgram = getPolicyEngineProgram(provider);
	const trackerAccount = getTrackerAccountPda(args.assetMint, args.owner);
	const ix = await policyProgram.methods
		.createTrackerAccount()
		.accountsStrict({
			payer: args.payer,
			trackerAccount,
			systemProgram: SystemProgram.programId,
			program: policyProgram.programId,
			assetMint: new PublicKey(args.assetMint),
			eventAuthority: getPolicyEnginerEventAuthority(),
			identityRegistry: getIdentityRegistryPda(args.assetMint),
			identityAccount: getIdentityAccountPda(args.assetMint, args.owner),
		})
		.instruction();
	return ix;
}