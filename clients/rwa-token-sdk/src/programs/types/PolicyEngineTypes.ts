/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/policy_engine.json`.
 */
export type PolicyEngine = {
  "address": "4wyusm2nxncvNyiVej5mZ1k1mVPhfG41oNvHx4BWiYMq",
  "metadata": {
    "name": "policyEngine",
    "version": "0.0.1",
    "spec": "0.1.0",
    "description": "The Policy Registry Program (PRP) enables the creation of policies that can be used to control the flow of funds in a programmatic way."
  },
  "instructions": [
    {
      "name": "attachToPolicyEngine",
      "docs": [
        "policies",
        "attach a policy"
      ],
      "discriminator": [
        99,
        59,
        117,
        21,
        146,
        11,
        54,
        173
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "signer",
          "signer": true
        },
        {
          "name": "policyEngine",
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "identityFilter",
          "type": {
            "defined": {
              "name": "identityFilter"
            }
          }
        },
        {
          "name": "policyType",
          "type": {
            "defined": {
              "name": "policyType"
            }
          }
        }
      ]
    },
    {
      "name": "changeCounterLimits",
      "discriminator": [
        200,
        2,
        8,
        102,
        43,
        168,
        141,
        139
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "signer",
          "signer": true
        },
        {
          "name": "policyEngine",
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "removedCounterLimits",
          "type": "bytes"
        },
        {
          "name": "addedCounterLimits",
          "type": {
            "vec": {
              "defined": {
                "name": "counterLimit"
              }
            }
          }
        }
      ]
    },
    {
      "name": "changeCounters",
      "discriminator": [
        156,
        107,
        88,
        204,
        113,
        131,
        241,
        192
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "signer",
          "signer": true
        },
        {
          "name": "policyEngine",
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "removedCounters",
          "type": "bytes"
        },
        {
          "name": "addedCounters",
          "type": {
            "vec": {
              "defined": {
                "name": "counter"
              }
            }
          }
        }
      ]
    },
    {
      "name": "changeIssuancePolicies",
      "discriminator": [
        186,
        201,
        163,
        157,
        32,
        250,
        166,
        37
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "signer",
          "signer": true
        },
        {
          "name": "policyEngine",
          "writable": true
        }
      ],
      "args": [
        {
          "name": "issuancePolicies",
          "type": {
            "defined": {
              "name": "issuancePolicies"
            }
          }
        }
      ]
    },
    {
      "name": "changeMapping",
      "discriminator": [
        103,
        1,
        52,
        20,
        160,
        194,
        113,
        125
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "signer",
          "signer": true
        },
        {
          "name": "policyEngine",
          "writable": true
        }
      ],
      "args": [
        {
          "name": "mappingSource",
          "type": "bytes"
        },
        {
          "name": "mappingValue",
          "type": "bytes"
        }
      ]
    },
    {
      "name": "createPolicyEngine",
      "docs": [
        "create a policy registry"
      ],
      "discriminator": [
        85,
        105,
        207,
        153,
        73,
        125,
        225,
        54
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "signer",
          "signer": true
        },
        {
          "name": "assetMint"
        },
        {
          "name": "policyEngineAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "assetMint"
              }
            ]
          }
        },
        {
          "name": "extraMetasAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  120,
                  116,
                  114,
                  97,
                  45,
                  97,
                  99,
                  99,
                  111,
                  117,
                  110,
                  116,
                  45,
                  109,
                  101,
                  116,
                  97,
                  115
                ]
              },
              {
                "kind": "account",
                "path": "assetMint"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "authority",
          "type": "pubkey"
        },
        {
          "name": "delegate",
          "type": {
            "option": "pubkey"
          }
        },
        {
          "name": "enforcePolicyIssuance",
          "type": {
            "option": "bool"
          }
        }
      ]
    },
    {
      "name": "createTrackerAccount",
      "docs": [
        "create tracker account"
      ],
      "discriminator": [
        40,
        16,
        40,
        191,
        109,
        177,
        83,
        190
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "identityAccount"
        },
        {
          "name": "identityRegistry",
          "relations": [
            "identityAccount"
          ]
        },
        {
          "name": "assetMint",
          "relations": [
            "identityRegistry"
          ]
        },
        {
          "name": "trackerAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "assetMint"
              },
              {
                "kind": "account",
                "path": "identityAccount"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "eventAuthority",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  95,
                  95,
                  101,
                  118,
                  101,
                  110,
                  116,
                  95,
                  97,
                  117,
                  116,
                  104,
                  111,
                  114,
                  105,
                  116,
                  121
                ]
              }
            ]
          }
        },
        {
          "name": "program"
        }
      ],
      "args": []
    },
    {
      "name": "detachFromPolicyEngine",
      "docs": [
        "remove policy"
      ],
      "discriminator": [
        156,
        137,
        67,
        121,
        46,
        207,
        45,
        12
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "signer",
          "signer": true
        },
        {
          "name": "policyEngine",
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "hash",
          "type": "string"
        }
      ]
    },
    {
      "name": "enforcePolicyIssuance",
      "discriminator": [
        204,
        149,
        175,
        224,
        136,
        255,
        219,
        75
      ],
      "accounts": [
        {
          "name": "assetController",
          "signer": true
        },
        {
          "name": "assetMint",
          "relations": [
            "policyEngine",
            "identityRegistry"
          ]
        },
        {
          "name": "policyEngine",
          "writable": true
        },
        {
          "name": "destinationAccount"
        },
        {
          "name": "identityRegistry",
          "relations": [
            "identityAccount"
          ]
        },
        {
          "name": "identityAccount",
          "relations": [
            "destinationTrackerAccount"
          ]
        },
        {
          "name": "destinationTrackerAccount",
          "writable": true
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "issuanceTimestamp",
          "type": "i64"
        }
      ]
    },
    {
      "name": "enforcePolicyOnLevelsChange",
      "discriminator": [
        236,
        18,
        123,
        137,
        253,
        244,
        32,
        248
      ],
      "accounts": [
        {
          "name": "identityAccount"
        },
        {
          "name": "identityRegistry",
          "signer": true,
          "relations": [
            "identityAccount"
          ]
        },
        {
          "name": "assetMint",
          "relations": [
            "identityRegistry",
            "policyEngine"
          ]
        },
        {
          "name": "trackerAccount",
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "assetMint"
              },
              {
                "kind": "account",
                "path": "identityAccount"
              }
            ]
          }
        },
        {
          "name": "policyEngine",
          "writable": true
        }
      ],
      "args": [
        {
          "name": "newLevels",
          "type": {
            "defined": {
              "name": "newLevelsArgs"
            }
          }
        },
        {
          "name": "newCountry",
          "type": "u8"
        },
        {
          "name": "enforceLimits",
          "type": "bool"
        }
      ]
    },
    {
      "name": "executeTransaction",
      "docs": [
        "execute transfer hook"
      ],
      "discriminator": [
        105,
        37,
        101,
        197,
        75,
        251,
        102,
        26
      ],
      "accounts": [
        {
          "name": "sourceAccount"
        },
        {
          "name": "assetMint"
        },
        {
          "name": "destinationAccount"
        },
        {
          "name": "ownerDelegate"
        },
        {
          "name": "extraMetasAccount",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  120,
                  116,
                  114,
                  97,
                  45,
                  97,
                  99,
                  99,
                  111,
                  117,
                  110,
                  116,
                  45,
                  109,
                  101,
                  116,
                  97,
                  115
                ]
              },
              {
                "kind": "account",
                "path": "assetMint"
              }
            ]
          }
        },
        {
          "name": "policyEngineAccount",
          "writable": true
        },
        {
          "name": "identityRegistry",
          "address": "7Zis9Cg1pa3PMRCMfJBgzoQThoBA21QMrkmEnx6nZdQQ"
        },
        {
          "name": "identityRegistryAccount"
        },
        {
          "name": "destinationIdentityAccount"
        },
        {
          "name": "sourceIdentityAccount"
        },
        {
          "name": "destinationTrackerAccount",
          "writable": true
        },
        {
          "name": "sourceTrackerAccount",
          "writable": true
        },
        {
          "name": "destinationWalletIdentity"
        },
        {
          "name": "sourceWalletIdentity"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "identityAccount",
      "discriminator": [
        194,
        90,
        181,
        160,
        182,
        206,
        116,
        158
      ]
    },
    {
      "name": "identityRegistryAccount",
      "discriminator": [
        154,
        254,
        118,
        4,
        115,
        36,
        125,
        78
      ]
    },
    {
      "name": "policyEngineAccount",
      "discriminator": [
        124,
        85,
        205,
        80,
        2,
        18,
        26,
        45
      ]
    },
    {
      "name": "trackerAccount",
      "discriminator": [
        83,
        95,
        166,
        148,
        57,
        30,
        90,
        210
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "invalidPolicy",
      "msg": "Invalid policy passed"
    },
    {
      "code": 6001,
      "name": "transactionAmountLimitExceeded",
      "msg": "Transaction amount limit exceeded"
    },
    {
      "code": 6002,
      "name": "transactionAmountVelocityExceeded",
      "msg": "Transaction amount velocity exceeded"
    },
    {
      "code": 6003,
      "name": "transactionCountVelocityExceeded",
      "msg": "Transaction count velocity exceeded"
    },
    {
      "code": 6004,
      "name": "identityLevelLimitExceeded",
      "msg": "Identity level limit exceeded"
    },
    {
      "code": 6005,
      "name": "policyEngineFull",
      "msg": "Policy registry is full, cannot add more policies"
    },
    {
      "code": 6006,
      "name": "policyNotFound",
      "msg": "Policy not found"
    },
    {
      "code": 6007,
      "name": "identityFilterFailed",
      "msg": "Identity filter failed"
    },
    {
      "code": 6008,
      "name": "unauthorizedSigner",
      "msg": "Unauthorized signer"
    },
    {
      "code": 6009,
      "name": "policyAlreadyExists",
      "msg": "Policy already exists"
    },
    {
      "code": 6010,
      "name": "maxBalanceExceeded",
      "msg": "Max balance exceeded"
    },
    {
      "code": 6011,
      "name": "minBalanceExceeded",
      "msg": "Min balance exceeded"
    },
    {
      "code": 6012,
      "name": "invalidCpiTransferAmount",
      "msg": "Invalid CPI transfer amount"
    },
    {
      "code": 6013,
      "name": "invalidCpiTransferMint",
      "msg": "Invalid CPI transfer mint"
    },
    {
      "code": 6014,
      "name": "invalidCpiTransferProgram",
      "msg": "Invalid CPI transfer program"
    },
    {
      "code": 6015,
      "name": "invalidPdaPassedIn",
      "msg": "Invalid PDA passed in"
    },
    {
      "code": 6016,
      "name": "transferHistoryFull",
      "msg": "Transfer history full"
    },
    {
      "code": 6017,
      "name": "transferPaused",
      "msg": "All Transfers have been paused"
    },
    {
      "code": 6018,
      "name": "forceFullTransfer",
      "msg": "Expected source account to transfer full amount"
    },
    {
      "code": 6019,
      "name": "holderLimitExceeded",
      "msg": "Holder limit exceeded"
    },
    {
      "code": 6020,
      "name": "balanceLimitExceeded",
      "msg": "Balance limit exceeded"
    },
    {
      "code": 6021,
      "name": "trackerAccountOwnerMismatch",
      "msg": "Tracker account owner mismatch"
    },
    {
      "code": 6022,
      "name": "forbiddenIdentityGroup",
      "msg": "Forbidden identity group"
    },
    {
      "code": 6023,
      "name": "invalidIdentityAccount",
      "msg": "Invalid identity account"
    },
    {
      "code": 6024,
      "name": "holdersLimitExceeded",
      "msg": "Holders limit exceeded"
    },
    {
      "code": 6025,
      "name": "minMaxBalanceExceeded",
      "msg": "Min max balance exceeded"
    },
    {
      "code": 6026,
      "name": "invalidPolicyEngineAccount",
      "msg": "Invalid policy engine account"
    },
    {
      "code": 6027,
      "name": "percentageLimitExceeded",
      "msg": "Percentage limit exceeded"
    },
    {
      "code": 6028,
      "name": "flowback",
      "msg": "flowback"
    },
    {
      "code": 6029,
      "name": "invalidInstructionData",
      "msg": "Invalid instruction data"
    },
    {
      "code": 6030,
      "name": "backdatingNotAllowed",
      "msg": "Backdating not allowed"
    },
    {
      "code": 6031,
      "name": "maxSupplyExceeded",
      "msg": "Max supply exceeded"
    }
  ],
  "types": [
    {
      "name": "counter",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "value",
            "type": "u64"
          },
          {
            "name": "id",
            "type": "u8"
          },
          {
            "name": "identityFilter",
            "type": {
              "defined": {
                "name": "identityFilter"
              }
            }
          }
        ]
      }
    },
    {
      "name": "counterLimit",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "holdersLimit",
            "fields": [
              {
                "name": "max",
                "type": "u64"
              },
              {
                "name": "min",
                "type": "u64"
              },
              {
                "name": "counterId",
                "type": "u8"
              }
            ]
          },
          {
            "name": "groupedHoldersLimit",
            "fields": [
              {
                "name": "max",
                "type": "u64"
              },
              {
                "name": "min",
                "type": "u64"
              },
              {
                "name": "counters",
                "type": "bytes"
              }
            ]
          },
          {
            "name": "percentageLimit",
            "fields": [
              {
                "name": "higherCounterId",
                "type": "u8"
              },
              {
                "name": "lowerCounterId",
                "type": "u8"
              },
              {
                "name": "minPercentage",
                "type": "u8"
              },
              {
                "name": "maxPercentage",
                "type": "u8"
              }
            ]
          }
        ]
      }
    },
    {
      "name": "filterComparison",
      "repr": {
        "kind": "rust"
      },
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "or"
          },
          {
            "name": "and"
          }
        ]
      }
    },
    {
      "name": "filterData",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "level",
            "type": {
              "defined": {
                "name": "filterLevel"
              }
            }
          },
          {
            "name": "target",
            "type": {
              "defined": {
                "name": "filterTarget"
              }
            }
          },
          {
            "name": "mode",
            "type": {
              "defined": {
                "name": "filterMode"
              }
            }
          }
        ]
      }
    },
    {
      "name": "filterInner",
      "repr": {
        "kind": "rust"
      },
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "single",
            "fields": [
              {
                "defined": {
                  "name": "filterData"
                }
              }
            ]
          },
          {
            "name": "tuple",
            "fields": [
              {
                "defined": {
                  "name": "filterData"
                }
              },
              {
                "defined": {
                  "name": "filterComparison"
                }
              },
              {
                "defined": {
                  "name": "filterData"
                }
              }
            ]
          },
          {
            "name": "multiple",
            "fields": [
              {
                "defined": {
                  "name": "filterComparison"
                }
              },
              {
                "vec": {
                  "defined": {
                    "name": "filterData"
                  }
                }
              }
            ]
          }
        ]
      }
    },
    {
      "name": "filterLevel",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "level",
            "fields": [
              "u8"
            ]
          },
          {
            "name": "levelMappingAny",
            "fields": [
              "u8"
            ]
          },
          {
            "name": "levelMapping",
            "fields": [
              {
                "name": "source",
                "type": "u8"
              },
              {
                "name": "target",
                "type": "u8"
              }
            ]
          },
          {
            "name": "country",
            "fields": [
              "u8"
            ]
          },
          {
            "name": "countryMapping",
            "fields": [
              "u8"
            ]
          }
        ]
      }
    },
    {
      "name": "filterMode",
      "repr": {
        "kind": "rust"
      },
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "include"
          },
          {
            "name": "exclude"
          }
        ]
      }
    },
    {
      "name": "filterTarget",
      "repr": {
        "kind": "rust"
      },
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "sender"
          },
          {
            "name": "receiver"
          },
          {
            "name": "bothAnd"
          },
          {
            "name": "bothOr"
          }
        ]
      }
    },
    {
      "name": "identityAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "version",
            "docs": [
              "version of the account"
            ],
            "type": "u8"
          },
          {
            "name": "identityRegistry",
            "docs": [
              "identity registry to which the account belongs"
            ],
            "type": "pubkey"
          },
          {
            "name": "owner",
            "docs": [
              "owner of the identity account"
            ],
            "type": "pubkey"
          },
          {
            "name": "numWallets",
            "type": "u16"
          },
          {
            "name": "country",
            "type": "u8"
          },
          {
            "name": "levels",
            "type": {
              "vec": {
                "defined": {
                  "name": "identityLevel"
                }
              }
            }
          }
        ]
      }
    },
    {
      "name": "identityFilter",
      "repr": {
        "kind": "rust"
      },
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "simple",
            "fields": [
              {
                "defined": {
                  "name": "filterInner"
                }
              }
            ]
          },
          {
            "name": "ifThen",
            "fields": [
              {
                "defined": {
                  "name": "filterInner"
                }
              },
              {
                "defined": {
                  "name": "filterInner"
                }
              }
            ]
          }
        ]
      }
    },
    {
      "name": "identityLevel",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "level",
            "type": "u8"
          },
          {
            "name": "expiry",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "identityRegistryAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "version",
            "type": "u8"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "assetMint",
            "docs": [
              "corresponding asset mint"
            ],
            "type": "pubkey"
          },
          {
            "name": "authority",
            "docs": [
              "authority to manage the registry"
            ],
            "type": "pubkey"
          },
          {
            "name": "delegate",
            "docs": [
              "registry delegate"
            ],
            "type": "pubkey"
          },
          {
            "name": "allowMultipleWallets",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "issuancePolicies",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "disallowBackdating",
            "type": "bool"
          },
          {
            "name": "maxSupply",
            "type": "u64"
          },
          {
            "name": "lockupPeriods",
            "type": {
              "vec": {
                "defined": {
                  "name": "lockupPeriod"
                }
              }
            }
          }
        ]
      }
    },
    {
      "name": "levelExpiry",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "level",
            "type": "u8"
          },
          {
            "name": "expiry",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "lockupPeriod",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "time",
            "type": "u64"
          },
          {
            "name": "identityFilter",
            "type": {
              "defined": {
                "name": "identityFilter"
              }
            }
          }
        ]
      }
    },
    {
      "name": "newLevelsArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "levels",
            "type": {
              "vec": {
                "defined": {
                  "name": "levelExpiry"
                }
              }
            }
          }
        ]
      }
    },
    {
      "name": "policy",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "hash",
            "type": "string"
          },
          {
            "name": "identityFilter",
            "type": {
              "defined": {
                "name": "identityFilter"
              }
            }
          },
          {
            "name": "policyType",
            "type": {
              "defined": {
                "name": "policyType"
              }
            }
          }
        ]
      }
    },
    {
      "name": "policyEngineAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "version",
            "docs": [
              "version"
            ],
            "type": "u8"
          },
          {
            "name": "assetMint",
            "docs": [
              "asset mint"
            ],
            "type": "pubkey"
          },
          {
            "name": "authority",
            "docs": [
              "authority of the registry"
            ],
            "type": "pubkey"
          },
          {
            "name": "delegate",
            "docs": [
              "policy delegate"
            ],
            "type": "pubkey"
          },
          {
            "name": "maxTimeframe",
            "docs": [
              "max timeframe of all the policies"
            ],
            "type": "i64"
          },
          {
            "name": "enforcePolicyIssuance",
            "docs": [
              "enforce policy issuance"
            ],
            "type": "bool"
          },
          {
            "name": "policies",
            "docs": [
              "initial max len"
            ],
            "type": {
              "vec": {
                "defined": {
                  "name": "policy"
                }
              }
            }
          },
          {
            "name": "counters",
            "type": {
              "vec": {
                "defined": {
                  "name": "counter"
                }
              }
            }
          },
          {
            "name": "counterLimits",
            "type": {
              "vec": {
                "defined": {
                  "name": "counterLimit"
                }
              }
            }
          },
          {
            "name": "mapping",
            "type": {
              "array": [
                "u8",
                256
              ]
            }
          },
          {
            "name": "issuancePolicies",
            "type": {
              "defined": {
                "name": "issuancePolicies"
              }
            }
          }
        ]
      }
    },
    {
      "name": "policyType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "identityApproval"
          },
          {
            "name": "transactionAmountLimit",
            "fields": [
              {
                "name": "limit",
                "type": "u64"
              }
            ]
          },
          {
            "name": "transactionAmountVelocity",
            "fields": [
              {
                "name": "limit",
                "type": "u64"
              },
              {
                "name": "timeframe",
                "type": "i64"
              }
            ]
          },
          {
            "name": "transactionCountVelocity",
            "fields": [
              {
                "name": "limit",
                "type": "u64"
              },
              {
                "name": "timeframe",
                "type": "i64"
              }
            ]
          },
          {
            "name": "maxBalance",
            "fields": [
              {
                "name": "limit",
                "type": "u64"
              }
            ]
          },
          {
            "name": "minBalance",
            "fields": [
              {
                "name": "limit",
                "type": "u64"
              }
            ]
          },
          {
            "name": "minMaxBalance",
            "fields": [
              {
                "name": "min",
                "type": "u64"
              },
              {
                "name": "max",
                "type": "u64"
              }
            ]
          },
          {
            "name": "transferPause"
          },
          {
            "name": "forbiddenIdentityGroup"
          },
          {
            "name": "forceFullTransfer"
          },
          {
            "name": "blockFlowbackEndTime",
            "fields": [
              {
                "name": "time",
                "type": "i64"
              },
              {
                "name": "targetLevel",
                "type": "u8"
              }
            ]
          }
        ]
      }
    },
    {
      "name": "side",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "buy"
          },
          {
            "name": "sell"
          }
        ]
      }
    },
    {
      "name": "trackerAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "version",
            "type": "u8"
          },
          {
            "name": "assetMint",
            "type": "pubkey"
          },
          {
            "name": "identityAccount",
            "type": "pubkey"
          },
          {
            "name": "transfers",
            "type": {
              "vec": {
                "defined": {
                  "name": "transfer"
                }
              }
            }
          },
          {
            "name": "totalAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "transfer",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          },
          {
            "name": "side",
            "type": {
              "defined": {
                "name": "side"
              }
            }
          }
        ]
      }
    }
  ]
};
