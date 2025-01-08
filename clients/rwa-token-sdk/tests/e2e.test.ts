import { BN, Wallet } from "@coral-xyz/anchor";
import {
	type AttachPolicyArgs,
	CreateDataAccountArgs,
	DeleteDataAccountArgs,
	getDataAccountsWithFilter,
	getFreezeTokenIx,
	getRevokeTokensIx,
	getSetupUserIxs,
	getThawTokenIx,
	getTrackerAccount,
	type IssueTokenArgs,
	type TransferTokensArgs,
	type UpdateDataAccountArgs,
	VoidTokensArgs,
	getIdentityAccount,
	getPolicyEngineAccount,
	getSeizeTokensIx,
	getWalletIdentityAccountsWithFilter,
	getIdentityAccountPda,
	getIdentityAccountFromOwner,
} from "../src";
import { setupTests } from "./setup";
import {
	type ConfirmOptions,
	Connection,
	Transaction,
	sendAndConfirmTransaction,
} from "@solana/web3.js";
import { expect, test, describe } from "vitest";
import { type Config } from "../src/classes/types";
import { RwaClient } from "../src/classes";

describe("e2e tests", async () => {
	let rwaClient: RwaClient;
	let mint: string;
	const setup = await setupTests();

	const decimals = 2;
	let dataAccount: string;

	test("setup provider", async () => {
		const connectionUrl = process.env.RPC_URL ?? "http://localhost:8899";
		const connection = new Connection(connectionUrl, "processed");

		const confirmationOptions: ConfirmOptions = {
			skipPreflight: false,
			maxRetries: 3,
			commitment: "processed",
		};

		const config: Config = {
			connection,
			rpcUrl: connectionUrl,
			confirmationOptions,
		};

		rwaClient = new RwaClient(config, new Wallet(setup.payerKp));
	});

	test("initialize asset controller", async () => {
		const setupAssetControllerArgs = {
			decimals,
			payer: setup.payer.toString(),
			authority: setup.authority.toString(),
			name: "Test Class Asset",
			uri: "https://test.com",
			symbol: "TFT",
			allowMultipleWallets: true,
		};

		const setupIx = await rwaClient.assetController.setupNewRegistry(
			setupAssetControllerArgs
		);
		mint = setupIx.signers[0].publicKey.toString();
		var tomorrow = Date.now() / 1000 + 24 * 60 * 60;

		const setupUserIxs = await getSetupUserIxs({
			assetMint: mint,
			payer: setup.payer.toString(),
			owner: setup.user1.toString(),
			signer: setup.authority.toString(),
			levels: [1],
			expiry: [new BN(tomorrow)],
			country: 1,
		}, rwaClient.provider);
		const setupUser2Ixs = await getSetupUserIxs({
			assetMint: mint,
			payer: setup.payer.toString(),
			owner: setup.user2.toString(),
			signer: setup.authority.toString(),
			levels: [1],
			expiry: [new BN(1800000000)],
			country: 1,
		}, rwaClient.provider);
		const setupUser3Ixs = await getSetupUserIxs({
			assetMint: mint,
			payer: setup.payer.toString(),
			owner: setup.authority.toString(),
			signer: setup.authority.toString(),
			levels: [255],
			expiry: [new BN(tomorrow)],
			country: 1,
		}, rwaClient.provider);
		const txnId = await sendAndConfirmTransaction(
			rwaClient.provider.connection,
			new Transaction().add(...setupIx.ixs).add(...setupUserIxs.ixs),
			[setup.payerKp, setup.authorityKp, ...setupIx.signers, ...setupUserIxs.signers]
		);
		const txnId2 = await sendAndConfirmTransaction(
			rwaClient.provider.connection,
			new Transaction().add(...setupUser2Ixs.ixs).add(...setupUser3Ixs.ixs),
			[setup.payerKp, setup.authorityKp, ...setupUser2Ixs.signers, ...setupUser3Ixs.signers]
		);
		expect(txnId).toBeTruthy();
		const trackerAccount = await getTrackerAccount(
			mint,
			setup.user1.toString(),
			rwaClient.provider
		);
		expect(trackerAccount).toBeTruthy();
		expect(trackerAccount!.assetMint.toString()).toBe(mint);

		const identityAccount = await getIdentityAccountFromOwner(mint, setup.user1.toString(), rwaClient.provider);
		expect(identityAccount).toBeTruthy();
		expect(identityAccount!.owner.toString()).toBe(setup.user1.toString());
		expect(identityAccount!.numWallets).toBe(1);
	});

	test("update asset metadata", async () => {
		const updateAssetMetadataArgs = {
			authority: setup.authority.toString(),
			name: "Test Class Asset - Updated",
			assetMint: mint,
			payer: setup.payer.toString(),
		};

		const updateIx = await rwaClient.assetController.updateAssetMetadata(
			updateAssetMetadataArgs
		);
		const txnId = await sendAndConfirmTransaction(
			rwaClient.provider.connection,
			new Transaction().add(updateIx),
			[setup.payerKp, setup.authorityKp]
		);
		expect(txnId).toBeTruthy();
	});

	test("setup data account", async () => {
		const createDataAccountArgs: CreateDataAccountArgs = {
			type: { legal: {} },
			name: "Test Data Account",
			uri: "https://test.com",
			payer: setup.payer.toString(),
			signer: setup.authority.toString(),
			assetMint: mint,
		};
		const createDataAccountIx = await rwaClient.dataRegistry.setupDataAccount(
			createDataAccountArgs
		);
		const txnId = await sendAndConfirmTransaction(
			rwaClient.provider.connection,
			new Transaction().add(...createDataAccountIx.ixs),
			[setup.payerKp, createDataAccountIx.signers[0], setup.authorityKp]
		);
		expect(txnId).toBeTruthy();
		dataAccount = createDataAccountIx.signers[0].publicKey.toString();
	});

	test("create identity approval policy", async () => {
		const policyArgs: AttachPolicyArgs = {
			authority: setup.authority.toString(),
			assetMint: mint,
			payer: setup.payer.toString(),
			identityFilter: {
				simple: [ {
					single: [
						{
							target: {bothOr: {}},
							mode: {include: {}},
							level: {level: [1]},
						}
					]
				}]
			},
			policyType: {
				identityApproval: {},
			},
		};

		const policyIx = await rwaClient.policyEngine.attachPolicy(policyArgs);
		const txnId = await sendAndConfirmTransaction(
			rwaClient.provider.connection,
			new Transaction().add(...policyIx.ixs),
			[setup.payerKp, setup.authorityKp]
		);
		expect(txnId).toBeTruthy();
	});

	test("attach transaction amount limit policy", async () => {
		const policyArgs: AttachPolicyArgs = {
			payer: setup.payer.toString(),
			assetMint: mint,
			authority: setup.authority.toString(),
			identityFilter: {
				simple: [ {
					single: [
						{
							target: {bothOr: {}},
							mode: {include: {}},
							level: {level: [1]},
						}
					]
				}]
			},
			policyType: {
				transactionAmountLimit: {
					limit: new BN(10000),
				},
			},
		};

		const policyIx = await rwaClient.policyEngine.attachPolicy(policyArgs);

		const txnId = await sendAndConfirmTransaction(
			rwaClient.provider.connection,
			new Transaction().add(...policyIx.ixs),
			[setup.payerKp, setup.authorityKp]
		);
		expect(txnId).toBeTruthy();
	});
	test("attach transaction amount velocity policy", async () => {
		const policyArgs: AttachPolicyArgs = {
			payer: setup.payer.toString(),
			assetMint: mint,
			authority: setup.authority.toString(),
			identityFilter: {
				simple: [ {
					single: [
						{
							target: {bothOr: {}},
							mode: {include: {}},
							level: {level: [1]},
						}
					]
				}]
			},
			policyType: {
				transactionAmountVelocity: {
					limit: new BN(100000),
					timeframe: new BN(60),
				},
			},
		};

		const policyIx = await rwaClient.policyEngine.attachPolicy(policyArgs);
		const txnId = await sendAndConfirmTransaction(
			rwaClient.provider.connection,
			new Transaction().add(...policyIx.ixs),
			[setup.payerKp, setup.authorityKp]
		);
		expect(txnId).toBeTruthy();
	});
	test("attach transaction count velocity policy", async () => {
		const policyArgs: AttachPolicyArgs = {
			payer: setup.payer.toString(),
			assetMint: mint,
			authority: setup.authority.toString(),
			identityFilter: {
				simple: [ {
					single: [
						{
							target: {bothOr: {}},
							mode: {include: {}},
							level: {level: [1]},
						}
					]
				}]
			},
			policyType: {
				transactionCountVelocity: {
					limit: new BN(100),
					timeframe: new BN(60),
				},
			},
		};

		const policyIx = await rwaClient.policyEngine.attachPolicy(policyArgs);
		const txnId = await sendAndConfirmTransaction(
			rwaClient.provider.connection,
			new Transaction().add(...policyIx.ixs),
			[setup.payerKp, setup.authorityKp]
		);
		expect(txnId).toBeTruthy();
	});

	test("issue tokens", async () => {
		const issueArgs: IssueTokenArgs = {
			authority: setup.authority.toString(),
			payer: setup.payer.toString(),
			owner: setup.user1.toString(),
			assetMint: mint,
			amount: 1000000,
		};
		const issueIx = await rwaClient.assetController.issueTokenIxns(issueArgs);
		const issue2Ix = await rwaClient.assetController.issueTokenIxns({...issueArgs, owner: setup.authority.toString() });
		const txnId = await sendAndConfirmTransaction(
			rwaClient.provider.connection,
			new Transaction().add(...issueIx).add(...issue2Ix),
			[setup.payerKp, setup.authorityKp],
		);
		expect(txnId).toBeTruthy();
	});

	test("void tokens", async () => {
		const voidArgs: VoidTokensArgs = {
			payer: setup.payer.toString(),
			amount: 100,
			owner: setup.user1.toString(),
			assetMint: mint,
			authority: setup.user1.toString(),
		};
		const voidIx = await rwaClient.assetController.voidTokenIxns(voidArgs);
		const txnId = await sendAndConfirmTransaction(
			rwaClient.provider.connection,
			new Transaction().add(voidIx),
			[setup.payerKp, setup.user1Kp]
		);
		expect(txnId).toBeTruthy();
	});

	test("revoke tokens", async () => {
		const revokeIx = await getRevokeTokensIx({
			owner: setup.user1.toString(),
			assetMint: mint,
			amount: 100,
			authority: setup.authority.toString(),
		}, rwaClient.provider);
		const txnId = await sendAndConfirmTransaction(
			rwaClient.provider.connection,
			new Transaction().add(revokeIx),
			[setup.payerKp, setup.authorityKp],
			{skipPreflight: true}
		);
		expect(txnId).toBeTruthy();
	});

	test("seize tokens", async () => {
		const seizeIx = await getSeizeTokensIx({
			from: setup.user1.toString(),
			to: setup.authority.toString(),
			assetMint: mint,
			amount: 100,
			authority: setup.authority.toString(),
		}, rwaClient.provider);
		const txnId = await sendAndConfirmTransaction(
			rwaClient.provider.connection,
			new Transaction().add(...seizeIx),
			[setup.payerKp, setup.authorityKp],
			{skipPreflight: true}
		);
		expect(txnId).toBeTruthy();
	});

	test("freeze and thaw token account", async () => {
		const freezeIx = await getFreezeTokenIx({
			authority: setup.authority.toString(),
			payer: setup.payer.toString(),
			owner: setup.user1.toString(),
			assetMint: mint,
		}, rwaClient.provider);
		const txnId = await sendAndConfirmTransaction(
			rwaClient.provider.connection,
			new Transaction().add(freezeIx),
			[setup.payerKp, setup.authorityKp]
		);
		expect(txnId).toBeTruthy();

		const thawIx = await getThawTokenIx({
			authority: setup.authority.toString(),
			payer: setup.payer.toString(),
			owner: setup.user1.toString(),
			assetMint: mint,
		}, rwaClient.provider);
		const txnId2 = await sendAndConfirmTransaction(
			rwaClient.provider.connection,
			new Transaction().add(thawIx),
			[setup.payerKp, setup.authorityKp]
		);
		expect(txnId2).toBeTruthy();
	});

	test("create wallet identity account", async () => {
		const createWalletIdentityAccountIx = await rwaClient.identityRegistry.attachWalletToIdentity({
			owner: setup.user1.toString(),
			assetMint: mint,
			payer: setup.payer.toString(),
			wallet: setup.user4.toString(),
			authority: setup.user1.toString(),
		});
		const txnId2 = await sendAndConfirmTransaction(
			rwaClient.provider.connection,
			new Transaction().add(createWalletIdentityAccountIx),
			[setup.payerKp, setup.user1Kp]
		);
		expect(txnId2).toBeTruthy();
	});

	test("get wallet identity account", async () => {
		const identityAccount = getIdentityAccountPda(mint, setup.user1.toString());
		const walletIdentityAccounts = await getWalletIdentityAccountsWithFilter(identityAccount.toBase58(), rwaClient.provider);
		expect(walletIdentityAccounts).toBeTruthy();
		expect(walletIdentityAccounts!.length).toBe(2);
		expect(walletIdentityAccounts!.some(walletIdentityAccount => walletIdentityAccount.wallet.toString() === setup.user4.toString())).toBe(true);
		expect(walletIdentityAccounts!.some(walletIdentityAccount => walletIdentityAccount.wallet.toString() === setup.user1.toString())).toBe(true);
	});

	test("issue to wallet identity account", async () => {
		const issueToWalletIdentityAccountIx = await rwaClient.assetController.issueTokenIxns({
			authority: setup.authority.toString(),
			payer: setup.payer.toString(),
			owner: setup.user1.toString(),
			wallet: setup.user4.toString(),
			assetMint: mint,
			amount: 100,
		});
		const txnId = await sendAndConfirmTransaction(
			rwaClient.provider.connection,
			new Transaction().add(...issueToWalletIdentityAccountIx),
			[setup.payerKp, setup.authorityKp]
		);
		expect(txnId).toBeTruthy();
	});

	test("transfer from wallet identity account", async () => {
		const transferFromWalletIdentityAccountIx = await rwaClient.assetController.transfer({
			from: setup.user4.toString(),
			to: setup.user1.toString(),
			assetMint: mint,
			amount: 100,
			decimals,
		});
		const txnId = await sendAndConfirmTransaction(
			rwaClient.provider.connection,
			new Transaction().add(...transferFromWalletIdentityAccountIx),
			[setup.payerKp, setup.user4Kp]
		);
		expect(txnId).toBeTruthy();
	});

	test("detach wallet identity account", async () => {
		const detachWalletIdentityAccountIx = await rwaClient.identityRegistry.detachWalletFromIdentity({
			owner: setup.user1.toString(),
			assetMint: mint,
			payer: setup.payer.toString(),
			wallet: setup.user4.toString(),
			authority: setup.authority.toString(),
		});
		const txnId = await sendAndConfirmTransaction(
			rwaClient.provider.connection,
			new Transaction().add(detachWalletIdentityAccountIx),
			[setup.payerKp, setup.authorityKp]
		);
		expect(txnId).toBeTruthy();
	});

	test("update data account", async () => {
		const updateDataAccountArgs: UpdateDataAccountArgs = {
			dataAccount,
			name: "Example Token Updates",
			uri: "newUri",
			type: { tax: {} },
			payer: setup.payer.toString(),
			assetMint: mint,
			authority: setup.authority.toString(),
			signer: setup.authority.toString(),
		};
		const updateDataIx = await rwaClient.dataRegistry.updateAssetsDataAccountInfoIxns(updateDataAccountArgs);
		const txnId = await sendAndConfirmTransaction(
			rwaClient.provider.connection,
			new Transaction().add(updateDataIx),
			[setup.payerKp, setup.authorityKp]
		);
		expect(txnId).toBeTruthy();
	});

	test("delete data account", async () => {
		const deleteDataAccountArgs: DeleteDataAccountArgs = {
			dataAccount,
			payer: setup.payer.toString(),
			assetMint: mint,
			signer: setup.authority.toString(),
		};
		const updateDataIx = await rwaClient.dataRegistry.deleteAssetsDataAccountInfoIxns(deleteDataAccountArgs);
		const txnId = await sendAndConfirmTransaction(
			rwaClient.provider.connection,
			new Transaction().add(updateDataIx),
			[setup.authorityKp]
		);
		expect(txnId).toBeTruthy();
		expect(await getDataAccountsWithFilter({assetMint: mint}, rwaClient.provider)).toHaveLength(0);
	});

	test("transfer tokens", async () => {
		const transferArgs: TransferTokensArgs = {
			from: setup.user1.toString(),
			to: setup.user2.toString(),
			assetMint: mint,
			amount: 2000,
			decimals,
			createTa: true,
		};

		const transferIxs = await rwaClient.assetController.transfer(transferArgs);
		const txnId = await sendAndConfirmTransaction(
			rwaClient.provider.connection,
			new Transaction().add(...transferIxs),
			[setup.user1Kp],
			{skipPreflight: true}
		);
		expect(txnId).toBeTruthy();
	});

	test("detach all policies", async () => {
		let policyAccount = await getPolicyEngineAccount(mint, rwaClient.provider);

		for (const policy of policyAccount?.policies ?? []) {
			const policyIx = await rwaClient.policyEngine.detachPolicy({
				authority: setup.authority.toString(),
				assetMint: mint,
				payer: setup.payer.toString(),
				hash: policy.hash,
			});
			const txnId = await sendAndConfirmTransaction(
				rwaClient.provider.connection,
				new Transaction().add(...policyIx.ixs),
				[setup.payerKp, setup.authorityKp]
			);
			expect(txnId).toBeTruthy();
		}
		policyAccount = await getPolicyEngineAccount(mint, rwaClient.provider);

		expect(policyAccount?.policies.length).toBe(0);
	});

	test("change country", async () => {
		const changeCountryIx = await rwaClient.identityRegistry.changeCountry({
			owner: setup.user1.toString(),
			assetMint: mint,
			payer: setup.payer.toString(),
			country: 100,
			signer: setup.authority.toString(),
		});
		const txnId = await sendAndConfirmTransaction(
			rwaClient.provider.connection,
			new Transaction().add(changeCountryIx),
			[setup.payerKp, setup.authorityKp]
		);
		expect(txnId).toBeTruthy();
	});

	test("add  forbidden country", async () => {
		const policyArgs: AttachPolicyArgs = {
			payer: setup.payer.toString(),
			assetMint: mint,
			authority: setup.authority.toString(),
			identityFilter: {
				simple: [ {
					single: [
						{
							target: {bothOr: {}},
							mode: {include: {}},
							level: {country: [100]},
						}
					]
				}]
			},
			policyType: {
				forbiddenIdentityGroup: {},
			},
		};
		const policyIx = await rwaClient.policyEngine.attachPolicy(policyArgs);
		const txnId = await sendAndConfirmTransaction(
			rwaClient.provider.connection,
			new Transaction().add(...policyIx.ixs),
			[setup.payerKp, setup.authorityKp]
		);
		expect(txnId).toBeTruthy();
	});

	test("fail to transfer out of user 1", async () => {
		const transferIx = await rwaClient.assetController.transfer({
			from: setup.user1.toString(),
			to: setup.user2.toString(),
			assetMint: mint,
			amount: 100,
			decimals,
		});
		expect(sendAndConfirmTransaction(
			rwaClient.provider.connection,
			new Transaction().add(...transferIx),
			[setup.user1Kp]
		)).rejects.toThrow();
	});

	test("change mapping", async () => {
		const changeMappingIx = await rwaClient.policyEngine.changeMapping({
			authority: setup.authority.toString(),
			payer: setup.payer.toString(),
			assetMint: mint,
			mappingSource: [100],
			mappingValue: [1],
		});
		const txnId = await sendAndConfirmTransaction(
			rwaClient.provider.connection,
			new Transaction().add(...changeMappingIx.ixs),
			[setup.payerKp, setup.authorityKp]
		);
		expect(txnId).toBeTruthy();
	});

	test("add forbidden identity group mapping", async () => {
		const policyArgs: AttachPolicyArgs = {
			payer: setup.payer.toString(),
			assetMint: mint,
			authority: setup.authority.toString(),
			identityFilter: {
				simple: [ {
					single: [
						{
							target: {sender: {}},
							mode: {exclude: {}},
							level: { levelMapping: {source: 1, target: 100}},
						}
					]
				}]
			},
			policyType: {
				identityApproval: {},
			},
		};
		const policyIx = await rwaClient.policyEngine.attachPolicy(policyArgs);
		const txnId = await sendAndConfirmTransaction(
			rwaClient.provider.connection,
			new Transaction().add(...policyIx.ixs),
			[setup.payerKp, setup.authorityKp]
		);
		expect(txnId).toBeTruthy();
	});

	test("fail to transfer out of user 2", async () => {
		const transferIx = await rwaClient.assetController.transfer({
			from: setup.user2.toString(),
			to: setup.user1.toString(),
			assetMint: mint,
			amount: 100,
			decimals,
		});
		expect(sendAndConfirmTransaction(
			rwaClient.provider.connection,
			new Transaction().add(...transferIx),
			[setup.user2Kp]
		)).rejects.toThrow();
	});
});

