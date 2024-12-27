
import { BN, Wallet } from "@coral-xyz/anchor";
import {
	getPolicyEnginePda, getPolicyEngineProgram, getTransferTokensIxs, 
	RwaClient,
} from "../src";
import { setupTests } from "./setup";
import { ConfirmOptions, Connection, Transaction, sendAndConfirmTransaction } from "@solana/web3.js";
import { expect, test, describe } from "vitest";
import { Config } from "../src/classes/types";

describe("test policy setup", async () => {
	let rwaClient: RwaClient;
	let mint: string;
	const setup = await setupTests();

	const decimals = 2;

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

	test("setup registries", async () => {
		const createAssetControllerArgs = {
			decimals,
			payer: setup.payer.toString(),
			authority: setup.authority.toString(),
			name: "Test Asset",
			uri: "https://test.com",
			symbol: "TST",
		};
		const setupAssetController = await rwaClient.assetController.setupNewRegistry(
			createAssetControllerArgs
		);

		const txnId = await sendAndConfirmTransaction(setup.provider.connection, new Transaction().add(...setupAssetController.ixs), [setup.payerKp, ...setupAssetController.signers]);
		mint = setupAssetController.signers[0].publicKey.toString();
		expect(txnId).toBeTruthy();
	});

	test("create policy account and attach identity approval policy", async () => {
		const attachPolicy = await rwaClient.policyEngine.attachPolicy({
			payer: setup.payer.toString(),
			assetMint: mint,
			authority: setup.authority.toString(),
			
			identityFilter: {
				simple: [ {
					tuple: [
						{
							target: {bothOr: {}},
							mode: {include: {}},
							level: {level: [1]},
						},
						{
							or: {}
						}, 
						{
							target: {bothOr: {}},
							mode: {include: {}},
							level: {level: [2]},
						}
					]
				}]
			},
			policyType: {
				identityApproval: {},
			},
		});
		const txnId = await sendAndConfirmTransaction(setup.provider.connection, new Transaction().add(...attachPolicy.ixs), [setup.payerKp, setup.authorityKp, ...attachPolicy.signers]);
		expect(txnId).toBeTruthy();
	});

	test("attach transaction amount limit policy to identity level 1", async () => {
		const attachPolicy = await rwaClient.policyEngine.attachPolicy({
			payer: setup.payer.toString(),
			assetMint: mint,
			authority: setup.authority.toString(),
			
			identityFilter: {
				simple: [ {
					single: [
						{
							target: {receiver: {}},
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
		});
		const txnId = await sendAndConfirmTransaction(setup.provider.connection, new Transaction().add(...attachPolicy.ixs), [setup.payerKp, setup.authorityKp, ...attachPolicy.signers]);
		expect(txnId).toBeTruthy();
	});

	test("attach transaction amount limit policy to identity level 2", async () => {
		const attachPolicy = await rwaClient.policyEngine.attachPolicy({
			payer: setup.payer.toString(),
			assetMint: mint,
			authority: setup.authority.toString(),
			identityFilter: {
				simple: [ {
					single: [
						{
							target: {sender: {}},
							mode: {include: {}},
							level: {level: [2]},
						}
					]
				}]
			},
			policyType: {
				transactionAmountLimit: {
					limit: new BN(200000),
				},
			},
		});
		const txnId = await sendAndConfirmTransaction(setup.provider.connection, new Transaction().add(...attachPolicy.ixs), [setup.payerKp, setup.authorityKp, ...attachPolicy.signers]);
		expect(txnId).toBeTruthy();
	});

	test("attach transaction amount velocity policy to identity level 1", async () => {
		const attachPolicy = await rwaClient.policyEngine.attachPolicy({
			payer: setup.payer.toString(),
			assetMint: mint,
			authority: setup.authority.toString(),
			identityFilter: {
				simple: [ {
					single: [
						{
							target: {receiver: {}},
							mode: {include: {}},
							level: {level: [1]},
						}
					]
				}]
			},
			policyType: {
				transactionAmountVelocity: {
					limit: new BN(25000),
					timeframe: new BN(3000),
				},
			},
		});
		const txnId = await sendAndConfirmTransaction(setup.provider.connection, new Transaction().add(...attachPolicy.ixs), [setup.payerKp, setup.authorityKp, ...attachPolicy.signers]);
		expect(txnId).toBeTruthy();
	});

	test("attach transaction count velocity policy to identity level 1", async () => {
		const attachPolicy = await rwaClient.policyEngine.attachPolicy({
			payer: setup.payer.toString(),
			assetMint: mint,
			authority: setup.authority.toString(),
			identityFilter: {
				simple: [ {
					single: [
						{
							target: {receiver: {}},
							mode: {include: {}},
							level: {level: [1]},
						}
					]
				}]
			},
			policyType: {
				transactionCountVelocity: {
					limit: new BN(4),
					timeframe: new BN(300),
				},
			},
		});
		const txnId = await sendAndConfirmTransaction(setup.provider.connection, new Transaction().add(...attachPolicy.ixs), [setup.payerKp, setup.authorityKp, ...attachPolicy.signers]);
		expect(txnId).toBeTruthy();
	});

	test("attach transaction count velocity policy to identity level 2", async () => {
		const attachPolicy = await rwaClient.policyEngine.attachPolicy({
			payer: setup.payer.toString(),
			assetMint: mint,
			authority: setup.authority.toString(),
			identityFilter: {
				simple: [ {
					single: [
						{
							target: {sender: {}},
							mode: {include: {}},
							level: {level: [2]},
						}
					]
				}]
			},
			policyType: {
				transactionCountVelocity: {
					limit: new BN(3),
					timeframe: new BN(60),
				},
			},
		});
		const txnId = await sendAndConfirmTransaction(setup.provider.connection, new Transaction().add(...attachPolicy.ixs), [setup.payerKp, setup.authorityKp, ...attachPolicy.signers]);
		expect(txnId).toBeTruthy();
		const policyAccount = await getPolicyEngineProgram(setup.provider).account.policyEngineAccount.fetch(getPolicyEnginePda(mint));
		expect(policyAccount.policies.length).toBe(6);
	});

	test("setup user1", async () => {
		const setupUser = await rwaClient.identityRegistry.setupUserIxns({
			payer: setup.payer.toString(),
			owner: setup.user1.toString(),
			assetMint: mint,
			levels: [1],
			expiry: [new BN(Date.now() / 1000 + 24 * 60 * 60)],
			signer: setup.authorityKp.publicKey.toString(),
			country: 1,
		});
		const txnId = await sendAndConfirmTransaction(setup.provider.connection, new Transaction().add(...setupUser.ixs), [setup.payerKp, setup.authorityKp, ...setupUser.signers]);
		expect(txnId).toBeTruthy();
	});

	test("setup user2", async () => {
		const setupUser = await rwaClient.identityRegistry.setupUserIxns({
			payer: setup.payer.toString(),
			owner: setup.user2.toString(),
			assetMint: mint,
			levels: [2],
			expiry: [new BN(Date.now() / 1000 + 24 * 60 * 60)],
			signer: setup.authorityKp.publicKey.toString(),
			country: 1,
		});
		const txnId = await sendAndConfirmTransaction(setup.provider.connection, new Transaction().add(...setupUser.ixs), [setup.payerKp, setup.authorityKp, ...setupUser.signers]);
		expect(txnId).toBeTruthy();
	});

	test("setup user3", async () => {
		const setupUser = await rwaClient.identityRegistry.setupUserIxns({
			payer: setup.payer.toString(),
			owner: setup.user3.toString(),
			assetMint: mint,
			levels: [255], // Skips all policies
			expiry: [new BN(Date.now() / 1000 + 24 * 60 * 60)],
			signer: setup.authorityKp.publicKey.toString(),
			country: 1,
		});
		const txnId = await sendAndConfirmTransaction(setup.provider.connection, new Transaction().add(...setupUser.ixs), [setup.payerKp, setup.authorityKp, ...setupUser.signers]);
		expect(txnId).toBeTruthy();
	});

	test("issue tokens", async () => {
		let issueTokens = await rwaClient.assetController.issueTokenIxns({
			authority: setup.authority.toString(),
			payer: setup.payer.toString(),
			owner: setup.user1.toString(),
			assetMint: mint,
			amount: 1000000,
		});
		let txnId = await sendAndConfirmTransaction(setup.provider.connection, new Transaction().add(...issueTokens), [setup.payerKp, setup.authorityKp]);
		expect(txnId).toBeTruthy();
		issueTokens = await rwaClient.assetController.issueTokenIxns({
			authority: setup.authority.toString(),
			payer: setup.payer.toString(),
			owner: setup.user2.toString(),
			assetMint: mint,
			amount: 1000000,
		});
		txnId = await sendAndConfirmTransaction(setup.provider.connection, new Transaction().add(...issueTokens), [setup.payerKp, setup.authorityKp]);
		expect(txnId).toBeTruthy();
		issueTokens = await rwaClient.assetController.issueTokenIxns({
			authority: setup.authority.toString(),
			payer: setup.payer.toString(),
			owner: setup.user3.toString(),
			assetMint: mint,
			amount: 1000000,
		});
		txnId = await sendAndConfirmTransaction(setup.provider.connection, new Transaction().add(...issueTokens), [setup.payerKp, setup.authorityKp]);
		expect(txnId).toBeTruthy();
	});

	test("transfer 1000 tokens from user1, user2 and user3. fail for user1, success for others", async () => {
		let transferTokensIxs = await getTransferTokensIxs({
			from: setup.user2.toString(),
			to: setup.user1.toString(),
			assetMint: mint,
			amount: 100000,
			decimals,
		}, rwaClient.provider);
		void expect(sendAndConfirmTransaction(
			setup.provider.connection,
			new Transaction().add(...transferTokensIxs),
			[setup.user2Kp],
		)).rejects.toThrowError();
		transferTokensIxs = await getTransferTokensIxs({
			from: setup.user3.toString(),
			to: setup.user2.toString(),
			assetMint: mint,
			amount: 1000,
			decimals,
		},  rwaClient.provider);
		let txnId = await sendAndConfirmTransaction(
			setup.provider.connection,
			new Transaction().add(...transferTokensIxs),
			[setup.user3Kp],
		);
		expect(txnId).toBeTruthy();
		transferTokensIxs = await getTransferTokensIxs({
			from: setup.user1.toString(),
			to: setup.user3.toString(),
			assetMint: mint,
			amount: 1000,
			decimals,
		}, rwaClient.provider);
		txnId = await sendAndConfirmTransaction(
			setup.provider.connection,
			new Transaction().add(...transferTokensIxs),
			[setup.user1Kp],
		);
		expect(txnId).toBeTruthy();
	});

	test("transfer 10000 tokens 3 times to user1, fail 3rd time", async () => {
		let transferTokensIxs = await getTransferTokensIxs({
			from: setup.user2.toString(),
			to: setup.user1.toString(),
			assetMint: mint,
			amount: 10000,
			decimals,
		}, rwaClient.provider);
		let txnId = await sendAndConfirmTransaction(
			setup.provider.connection,
			new Transaction().add(...transferTokensIxs),
			[setup.user2Kp],
		);
		expect(txnId).toBeTruthy();
		transferTokensIxs = await getTransferTokensIxs({
			from: setup.user2.toString(),
			to: setup.user1.toString(),
			assetMint: mint,
			amount: 10000,
			decimals,
		}, rwaClient.provider);
		txnId = await sendAndConfirmTransaction(
			setup.provider.connection,
			new Transaction().add(...transferTokensIxs),
			[setup.user2Kp],
		);
		expect(txnId).toBeTruthy();
		transferTokensIxs = await getTransferTokensIxs({
			from: setup.user2.toString(),
			to: setup.user1.toString(),
			assetMint: mint,
			amount: 10000,
			decimals,
		}, rwaClient.provider);
		void expect(sendAndConfirmTransaction(
			setup.provider.connection,
			new Transaction().add(...transferTokensIxs),
			[setup.user2Kp],
		)).rejects.toThrowError();
	});
});