/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/identity_registry.json`.
 */
export type IdentityRegistry = {
  "address": "GZsnjqT3c5zbHqsctrJ4EG4rbEfo7ZXyyUG7aDJNmxfA",
  "metadata": {
    "name": "identityRegistry",
    "version": "0.0.1",
    "spec": "0.1.0",
    "description": "The Identity Registry Program (IRP) manages the configurable issuance and tracking of on-chain identities to enable on-chain transaction permissioning.",
    "repository": "https://github.com/bridgesplit/rwa"
  },
  "instructions": [
    {
      "name": "addLevelToIdentityAccount",
      "docs": [
        "add level to identity account"
      ],
      "discriminator": [
        102,
        204,
        64,
        169,
        252,
        177,
        192,
        232
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
          "name": "identityRegistry"
        },
        {
          "name": "identityAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "identityRegistry"
              },
              {
                "kind": "account",
                "path": "identity_account.owner",
                "account": "identityAccount"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "policyEngineProgram"
        },
        {
          "name": "policyEngine",
          "writable": true
        },
        {
          "name": "trackerAccount"
        },
        {
          "name": "assetMint",
          "relations": [
            "identityRegistry"
          ]
        }
      ],
      "args": [
        {
          "name": "levels",
          "type": "bytes"
        },
        {
          "name": "expiries",
          "type": {
            "vec": "i64"
          }
        },
        {
          "name": "enforceLimits",
          "type": "bool"
        }
      ]
    },
    {
      "name": "attachWalletToIdentity",
      "docs": [
        "attach token account to identity account"
      ],
      "discriminator": [
        61,
        129,
        252,
        190,
        8,
        202,
        179,
        90
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "identityAccount",
          "writable": true
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
          "name": "walletIdentity",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "arg",
                "path": "wallet"
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
          "name": "wallet",
          "type": "pubkey"
        }
      ]
    },
    {
      "name": "changeCountry",
      "discriminator": [
        208,
        227,
        224,
        246,
        9,
        254,
        62,
        179
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
          "name": "identityRegistry"
        },
        {
          "name": "identityAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "identityRegistry"
              },
              {
                "kind": "account",
                "path": "identity_account.owner",
                "account": "identityAccount"
              }
            ]
          }
        },
        {
          "name": "policyEngineProgram"
        },
        {
          "name": "policyEngine",
          "writable": true
        },
        {
          "name": "trackerAccount"
        },
        {
          "name": "assetMint",
          "relations": [
            "identityRegistry"
          ]
        }
      ],
      "args": [
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
      "name": "createIdentityAccount",
      "docs": [
        "identity functions",
        "create identity account"
      ],
      "discriminator": [
        82,
        240,
        35,
        129,
        113,
        134,
        116,
        70
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
          "name": "identityRegistry"
        },
        {
          "name": "identityAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "identityRegistry"
              },
              {
                "kind": "arg",
                "path": "owner"
              }
            ]
          }
        },
        {
          "name": "walletIdentity",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "arg",
                "path": "owner"
              },
              {
                "kind": "account",
                "path": "identity_registry.asset_mint",
                "account": "identityRegistryAccount"
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
          "name": "owner",
          "type": "pubkey"
        },
        {
          "name": "level",
          "type": "u8"
        },
        {
          "name": "expiry",
          "type": "i64"
        },
        {
          "name": "country",
          "type": "u8"
        }
      ]
    },
    {
      "name": "createIdentityRegistry",
      "docs": [
        "registry functions",
        "create identity registry"
      ],
      "discriminator": [
        180,
        3,
        39,
        22,
        183,
        212,
        39,
        209
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
          "name": "identityRegistryAccount",
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
          "name": "allowMultipleWallets",
          "type": {
            "option": "bool"
          }
        }
      ]
    },
    {
      "name": "delegateIdentityRegsitry",
      "docs": [
        "delegate identity registry"
      ],
      "discriminator": [
        29,
        162,
        167,
        70,
        52,
        79,
        50,
        65
      ],
      "accounts": [
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "identityRegistryAccount",
          "writable": true
        }
      ],
      "args": [
        {
          "name": "delegate",
          "type": "pubkey"
        }
      ]
    },
    {
      "name": "detachWalletFromIdentity",
      "docs": [
        "detach token account from identity account"
      ],
      "discriminator": [
        166,
        70,
        236,
        254,
        166,
        116,
        201,
        50
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "walletIdentity",
          "writable": true
        },
        {
          "name": "identityAccount",
          "writable": true,
          "relations": [
            "walletIdentity"
          ]
        },
        {
          "name": "identityRegistry",
          "relations": [
            "identityAccount"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "refreshLevelToIdentityAccount",
      "docs": [
        "add level to identity account"
      ],
      "discriminator": [
        23,
        68,
        237,
        111,
        144,
        169,
        239,
        91
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
          "name": "identityRegistry"
        },
        {
          "name": "identityAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "identityRegistry"
              },
              {
                "kind": "account",
                "path": "identity_account.owner",
                "account": "identityAccount"
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "level",
          "type": "u8"
        },
        {
          "name": "expiry",
          "type": "i64"
        }
      ]
    },
    {
      "name": "removeLevelFromIdentityAccount",
      "docs": [
        "remove level from identity account"
      ],
      "discriminator": [
        194,
        231,
        187,
        54,
        197,
        136,
        170,
        55
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
          "name": "identityRegistry"
        },
        {
          "name": "identityAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "identityRegistry"
              },
              {
                "kind": "account",
                "path": "identity_account.owner",
                "account": "identityAccount"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "policyEngineProgram"
        },
        {
          "name": "policyEngine",
          "writable": true
        },
        {
          "name": "trackerAccount"
        },
        {
          "name": "assetMint"
        }
      ],
      "args": [
        {
          "name": "levels",
          "type": "bytes"
        },
        {
          "name": "enforceLimits",
          "type": "bool"
        }
      ]
    },
    {
      "name": "revokeIdentityAccount",
      "docs": [
        "revoke user identity account by closing account"
      ],
      "discriminator": [
        77,
        88,
        182,
        61,
        235,
        49,
        2,
        137
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
          "name": "identityRegistry",
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "identity_registry.asset_mint",
                "account": "identityRegistryAccount"
              }
            ]
          }
        },
        {
          "name": "identityAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "identityRegistry"
              },
              {
                "kind": "account",
                "path": "identity_account.owner",
                "account": "identityAccount"
              }
            ]
          },
          "relations": [
            "walletIdentity"
          ]
        },
        {
          "name": "walletIdentity",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "arg",
                "path": "owner"
              },
              {
                "kind": "account",
                "path": "identity_registry.asset_mint",
                "account": "identityRegistryAccount"
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "owner",
          "type": "pubkey"
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
      "name": "walletIdentity",
      "discriminator": [
        101,
        142,
        55,
        104,
        168,
        77,
        57,
        85
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "levelAlreadyPresent",
      "msg": "Identity level has already been attached to user"
    },
    {
      "code": 6001,
      "name": "maxLevelsExceeded",
      "msg": "Number of levels that can be attached to user has been exceeded"
    },
    {
      "code": 6002,
      "name": "levelNotFound",
      "msg": "Level to be removed not found"
    },
    {
      "code": 6003,
      "name": "unauthorizedSigner",
      "msg": "Unauthorized signer"
    },
    {
      "code": 6004,
      "name": "limitReached",
      "msg": "Identity limit reached"
    },
    {
      "code": 6005,
      "name": "tokenAccountAlreadyInitialized",
      "msg": "Token account is already initialized"
    },
    {
      "code": 6006,
      "name": "identityCreationRequired",
      "msg": "Identity creation must be enforced for this feature"
    },
    {
      "code": 6007,
      "name": "multipleWalletsNotAllowed",
      "msg": "Multiple wallets are not allowed"
    },
    {
      "code": 6008,
      "name": "walletAlreadyInUse",
      "msg": "Wallet already in use"
    },
    {
      "code": 6009,
      "name": "invalidLevel",
      "msg": "Invalid level"
    }
  ],
  "types": [
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
      "name": "walletIdentity",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "identityAccount",
            "type": "pubkey"
          },
          {
            "name": "wallet",
            "type": "pubkey"
          }
        ]
      }
    }
  ]
};
