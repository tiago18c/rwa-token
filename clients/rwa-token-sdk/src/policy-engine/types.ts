import { type IdlAccounts, type IdlTypes } from "@coral-xyz/anchor";
import { type PolicyEngineIdlTypes } from "../programs";

/** Represents on chain policy. */
export type PolicyType = IdlTypes<PolicyEngineIdlTypes>["policyType"];
export type PolicyTypeMinMaxBalance = IdlTypes<PolicyEngineIdlTypes>["policyType"]["minMaxBalance"];

/** Represents on chain filter for identity used by the policy engine. */
export type IdentityFilter = IdlTypes<PolicyEngineIdlTypes>["identityFilter"];

/** Represents on chain identity filter's comparison type used by the policy engine. */
export type IdentityFilterComparisonType =
  IdlTypes<PolicyEngineIdlTypes>["comparisionType"];

/** Represents on chain tracker account pda. */
export type TrackerAccount = IdlAccounts<PolicyEngineIdlTypes>["trackerAccount"];

export type ComparisonTypeOr = IdlTypes<PolicyEngineIdlTypes>["comparisionType"]["or"];
export type ComparisonTypeAnd = IdlTypes<PolicyEngineIdlTypes>["comparisionType"]["and"];
export type ComparisonTypeExcept = IdlTypes<PolicyEngineIdlTypes>["comparisionType"]["except"];

export type HolderLimit = IdlTypes<PolicyEngineIdlTypes>["counterLimit"]["holdersLimit"];
export type GroupedHolderLimit = IdlTypes<PolicyEngineIdlTypes>["counterLimit"]["groupedHoldersLimit"];
export type PercentageLimit = IdlTypes<PolicyEngineIdlTypes>["counterLimit"]["percentageLimit"];
export type CounterLimit = IdlTypes<PolicyEngineIdlTypes>["counterLimit"];

export type ComparisonType = IdlTypes<PolicyEngineIdlTypes>["comparisionType"];

export type CounterpartyFilter = IdlTypes<PolicyEngineIdlTypes>["counterpartyFilter"];

export type IdentityLevel = IdlTypes<PolicyEngineIdlTypes>["identityLevel"];

export type Counter = IdlTypes<PolicyEngineIdlTypes>["counter"];

/** Represents on chain policy. */
export type Policy = IdlTypes<PolicyEngineIdlTypes>["policy"];


/** Represents on chain policy engine account. */
export type PolicyEngineAccount =
  IdlAccounts<PolicyEngineIdlTypes>["policyEngineAccount"];
