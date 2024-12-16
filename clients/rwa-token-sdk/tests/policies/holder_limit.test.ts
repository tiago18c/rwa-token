import { BN, Wallet } from "@coral-xyz/anchor";
import {
	getPolicyEngineAccount,
	getTransferTokensIxs,
	RwaClient,
} from "../../src";
import { setupTests } from "../setup";
import { ConfirmOptions, Connection, Transaction, sendAndConfirmTransaction } from "@solana/web3.js";
import { expect, test, describe } from "vitest";
import { Config } from "../../src/classes/types";

describe("test additional policies", async () => {
	let rwaClient: RwaClient;
	let mint: string;
	const setup = await setupTests();
	const decimals = 2;

	test("setup environment", async () => {
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

		// Create asset controller
		const createAssetControllerArgs = {
			decimals,
			payer: setup.payer.toString(),
			authority: setup.authority.toString(),
			name: "Test Asset",
			uri: "https://test.com",
			symbol: "TST",
			enforcePolicyIssuance: true,
		};
		const setupAssetController = await rwaClient.assetController.setupNewRegistry(
			createAssetControllerArgs
		);
		const txnId = await sendAndConfirmTransaction(
			setup.provider.connection,
			new Transaction().add(...setupAssetController.ixs),
			[setup.payerKp, ...setupAssetController.signers]
		);
		mint = setupAssetController.signers[0].publicKey.toString();
		expect(txnId).toBeTruthy();

		// Setup users
		const setupUser1 = await rwaClient.identityRegistry.setupUserIxns({
			payer: setup.payer.toString(),
			owner: setup.user1.toString(),
			assetMint: mint,
			levels: [1],
			expiry: [new BN(Date.now() / 1000 + 24 * 60 * 60)],
			signer: setup.authorityKp.publicKey.toString()
		});
		await sendAndConfirmTransaction(
			setup.provider.connection,
			new Transaction().add(...setupUser1.ixs),
			[setup.payerKp, setup.authorityKp, ...setupUser1.signers]
		);

		const setupUser2 = await rwaClient.identityRegistry.setupUserIxns({
			payer: setup.payer.toString(),
			owner: setup.user2.toString(),
			assetMint: mint,
			levels: [1],
			expiry: [new BN(Date.now() / 1000 + 24 * 60 * 60)],
			signer: setup.authorityKp.publicKey.toString()
		});
		await sendAndConfirmTransaction(
			setup.provider.connection,
			new Transaction().add(...setupUser2.ixs),
			[setup.payerKp, setup.authorityKp, ...setupUser2.signers]
		);

		const setupUser4 = await rwaClient.identityRegistry.setupUserIxns({
			payer: setup.payer.toString(),
			owner: setup.user4.toString(),
			assetMint: mint,
			levels: [2],
			expiry: [new BN(Date.now() / 1000 + 24 * 60 * 60)],
			signer: setup.authorityKp.publicKey.toString()
		});
		await sendAndConfirmTransaction(
			setup.provider.connection,
			new Transaction().add(...setupUser4.ixs),
			[setup.payerKp, setup.authorityKp, ...setupUser4.signers]
		);

		// Issue tokens to user1
		const issueTokens = await rwaClient.assetController.issueTokenIxns({
			authority: setup.authority.toString(),
			payer: setup.payer.toString(),
			owner: setup.user1.toString(),
			assetMint: mint,
			amount: 1000000,
		});
		await sendAndConfirmTransaction(
			setup.provider.connection,
			new Transaction().add(...issueTokens),
			[setup.payerKp, setup.authorityKp]
		);
	});
	describe("test HolderLimit policy", async () => {
		const holderLimit = 2;

		test("attach HolderLimit policy 1", async () => {
			const attachPolicy = await rwaClient.policyEngine.attachPolicy({
				payer: setup.payer.toString(),
				assetMint: mint,
				authority: setup.authority.toString(),
				identityFilter: {
					identityLevels: [1],
					comparisionType: { or: {} },
					counterpartyFilter: { both: {} },
				},
				policyType: { 
					holdersLimit: { 
						min: new BN(2),
						max: new BN(holderLimit),
						currentHolders: new BN(1) // Assuming user1 is the only holder at this point
					} 
				},
			});
			const txnId = await sendAndConfirmTransaction(
				setup.provider.connection,
				new Transaction().add(...attachPolicy.ixs),
				[setup.payerKp, setup.authorityKp, ...attachPolicy.signers]
			);
			expect(txnId).toBeTruthy();
		});
		test("attach HolderLimit policy 2", async () => {
			const attachPolicy = await rwaClient.policyEngine.attachPolicy({
				payer: setup.payer.toString(),
				assetMint: mint,
				authority: setup.authority.toString(),
				identityFilter: {
					identityLevels: [2],
					comparisionType: { or: {} },
					counterpartyFilter: { both: {} },
				},
				policyType: { 
					holdersLimit: { 
						min: new BN(0),
						max: new BN(holderLimit),
						currentHolders: new BN(0) // Assuming user1 is the only holder at this point
					} 
				},
			});
			const txnId = await sendAndConfirmTransaction(
				setup.provider.connection,
				new Transaction().add(...attachPolicy.ixs),
				[setup.payerKp, setup.authorityKp, ...attachPolicy.signers]
			);
			expect(txnId).toBeTruthy();
		});

		test("transfer to new holder within limit", async () => {
			const transferTokensIxs = await getTransferTokensIxs({
				from: setup.user1.toString(),
				to: setup.user2.toString(),
				assetMint: mint,
				amount: 10,
				decimals,
				createTa: true,
			}, rwaClient.provider);
            
			const txnId = await sendAndConfirmTransaction(
				setup.provider.connection,
				new Transaction().add(...transferTokensIxs),
				[setup.user1Kp],
			);
			expect(txnId).toBeTruthy();
		});

		test("attempt transfer to new holder exceeding limit", async () => {
			const setupUser3 = await rwaClient.identityRegistry.setupUserIxns({
				payer: setup.payer.toString(),
				owner: setup.user3.toString(),
				assetMint: mint,
				levels: [1],
				expiry: [new BN(Date.now() / 1000 + 24 * 60 * 60)],
				signer: setup.authorityKp.publicKey.toString()
			});
			await sendAndConfirmTransaction(
				setup.provider.connection,
				new Transaction().add(...setupUser3.ixs),
				[setup.payerKp, setup.authorityKp, ...setupUser3.signers]
			);

			const transferTokensIxs = await getTransferTokensIxs({
				from: setup.user1.toString(),
				to: setup.user3.toString(),
				assetMint: mint,
				amount: 10,
				decimals,
				createTa: true,
			}, rwaClient.provider);

			await expect(sendAndConfirmTransaction(
				setup.provider.connection,
				new Transaction().add(...transferTokensIxs),
				[setup.user1Kp],
			)).rejects.toThrowError(/custom program error: 0x1788/); // HolderLimitExceeded error
		});

		test("transfer back within lower limit", async () => {
			const transferTokensIxs = await getTransferTokensIxs({
				from: setup.user2.toString(),
				to: setup.user1.toString(),
				assetMint: mint,
				amount: 9,
				decimals,
				createTa: true,
			}, rwaClient.provider);

			const txnId = await sendAndConfirmTransaction(
				setup.provider.connection,
				new Transaction().add(...transferTokensIxs),
				[setup.user2Kp],
			);
			expect(txnId).toBeTruthy();
		});	

		test("attempt to transfer back exceeding lower limit", async () => {
			const transferTokensIxs = await getTransferTokensIxs({
				from: setup.user2.toString(),
				to: setup.user1.toString(),
				assetMint: mint,
				amount: 1,
				decimals,
				createTa: true,
			}, rwaClient.provider);

			await expect(sendAndConfirmTransaction(
				setup.provider.connection,
				new Transaction().add(...transferTokensIxs),
				[setup.user2Kp],
			)).rejects.toThrowError(/custom program error: 0x1788/); // HolderLimitExceeded error
		});

		test("attempt to issue tokens that breaks holders limit", async () => {

			// Issue tokens to user1
			const issueTokens = await rwaClient.assetController.issueTokenIxns({
				authority: setup.authority.toString(),
				payer: setup.payer.toString(),
				owner: setup.user3.toString(),
				assetMint: mint,
				amount: 1000000,
			});
			await expect(sendAndConfirmTransaction(
				setup.provider.connection,
				new Transaction().add(...issueTokens),
				[setup.payerKp, setup.authorityKp],
				{
					skipPreflight: true,
				}
			)).rejects.toThrowError(/"InstructionError":\[0,{"Custom":6024}]/); // HolderLimitExceeded error
		});

		test("able to mint and increase holders limit", async () => {

			let policyEngine = await getPolicyEngineAccount(mint, rwaClient.provider);
			let policy = policyEngine?.policies.find((p) => p.policyType.holdersLimit && p.identityFilter.identityLevels.includes(2));
			expect(policy).toBeTruthy();
			expect(policy?.policyType.holdersLimit?.currentHolders.toNumber()).toBe(0);

			// Issue tokens to user1
			const issueTokens = await rwaClient.assetController.issueTokenIxns({
				authority: setup.authority.toString(),
				payer: setup.payer.toString(),
				owner: setup.user4.toString(),
				assetMint: mint,
				amount: 1000000,
			});
			const txnid = await sendAndConfirmTransaction(
				setup.provider.connection,
				new Transaction().add(...issueTokens),
				[setup.payerKp, setup.authorityKp],
				{
					skipPreflight: true,
				}
			); // HolderLimitExceeded error

			expect(txnid).toBeTruthy();
			console.log("txnid", txnid);

			policyEngine = await getPolicyEngineAccount(mint, rwaClient.provider);

			policy = policyEngine?.policies.find((p) => p.policyType.holdersLimit && p.identityFilter.identityLevels.includes(2));
			expect(policy).toBeTruthy();
			expect(policy?.policyType.holdersLimit?.currentHolders.toNumber()).toBe(1);
		});
	});
});
