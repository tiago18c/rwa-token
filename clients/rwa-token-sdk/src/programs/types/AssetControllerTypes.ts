/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/asset_controller.json`.
 */
export type AssetController = {
  "address": "2fJZj6eLMvdGkHTePvbXGjh9x1hHSMvkBPhTcAAPvBMa",
  "metadata": {
    "name": "assetController",
    "version": "0.0.1",
    "spec": "0.1.0",
    "description": "The Asset Controller Program (ACP) enables core asset management functionality for newly issued assets, including transfer controls and transaction privacy."
  },
  "instructions": [
    {
      "name": "burnTokens",
      "docs": [
        "burn shares of the rwa asset"
      ],
      "discriminator": [
        76,
        15,
        51,
        254,
        229,
        215,
        121,
        66
      ],
      "accounts": [
        {
          "name": "owner",
          "signer": true
        },
        {
          "name": "assetMint",
          "writable": true
        },
        {
          "name": "tokenAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "owner"
              },
              {
                "kind": "account",
                "path": "tokenProgram"
              },
              {
                "kind": "account",
                "path": "assetMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "closeMintAccount",
      "docs": [
        "close mint account"
      ],
      "discriminator": [
        14,
        121,
        72,
        246,
        96,
        224,
        42,
        162
      ],
      "accounts": [
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "assetMint",
          "writable": true
        },
        {
          "name": "assetController",
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
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        }
      ],
      "args": []
    },
    {
      "name": "closeTokenAccount",
      "docs": [
        "close a token account"
      ],
      "discriminator": [
        132,
        172,
        24,
        60,
        100,
        156,
        135,
        97
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "owner",
          "signer": true
        },
        {
          "name": "assetMint"
        },
        {
          "name": "tokenAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "owner"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "assetMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        }
      ],
      "args": []
    },
    {
      "name": "createAssetController",
      "docs": [
        "create an rwa asset"
      ],
      "discriminator": [
        97,
        185,
        6,
        250,
        248,
        242,
        68,
        105
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "authority"
        },
        {
          "name": "assetController",
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
          "name": "assetMint",
          "writable": true,
          "signer": true
        },
        {
          "name": "extraMetasAccount",
          "writable": true
        },
        {
          "name": "policyEngineAccount",
          "writable": true
        },
        {
          "name": "identityRegistryAccount",
          "writable": true
        },
        {
          "name": "dataRegistryAccount",
          "writable": true
        },
        {
          "name": "policyEngine",
          "address": "4wyusm2nxncvNyiVej5mZ1k1mVPhfG41oNvHx4BWiYMq"
        },
        {
          "name": "identityRegistry",
          "address": "7Zis9Cg1pa3PMRCMfJBgzoQThoBA21QMrkmEnx6nZdQQ"
        },
        {
          "name": "dataRegistry",
          "address": "8Bp1xoRscjuHoG1KT41zAaujGTx2fyB2uzTt8GTeWZX8"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
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
      "args": [
        {
          "name": "args",
          "type": {
            "defined": {
              "name": "createAssetControllerArgs"
            }
          }
        }
      ]
    },
    {
      "name": "disableMemoTransfer",
      "docs": [
        "memo transfer disable"
      ],
      "discriminator": [
        68,
        156,
        197,
        9,
        43,
        91,
        114,
        19
      ],
      "accounts": [
        {
          "name": "owner",
          "signer": true
        },
        {
          "name": "tokenAccount",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
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
      "name": "enableMemoTransfer",
      "docs": [
        "memo transfer enable"
      ],
      "discriminator": [
        186,
        78,
        97,
        172,
        71,
        172,
        99,
        0
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "owner",
          "signer": true
        },
        {
          "name": "assetMint"
        },
        {
          "name": "tokenAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "owner"
              },
              {
                "kind": "account",
                "path": "tokenProgram"
              },
              {
                "kind": "account",
                "path": "assetMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
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
      "name": "freezeTokenAccount",
      "docs": [
        "freeze token account"
      ],
      "discriminator": [
        138,
        168,
        178,
        109,
        205,
        224,
        209,
        93
      ],
      "accounts": [
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "assetMint",
          "writable": true
        },
        {
          "name": "assetController",
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
          "name": "tokenAccount",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        }
      ],
      "args": []
    },
    {
      "name": "issueTokens",
      "docs": [
        "issue shares of the rwa asset"
      ],
      "discriminator": [
        40,
        207,
        145,
        106,
        249,
        54,
        23,
        179
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "assetMint",
          "writable": true,
          "relations": [
            "identityRegistry",
            "trackerAccount"
          ]
        },
        {
          "name": "assetController",
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
          "name": "to"
        },
        {
          "name": "tokenAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "to"
              },
              {
                "kind": "account",
                "path": "tokenProgram"
              },
              {
                "kind": "account",
                "path": "assetMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "identityRegistry",
          "relations": [
            "identityAccount"
          ]
        },
        {
          "name": "identityAccount"
        },
        {
          "name": "trackerAccount",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "policyEngineProgram",
          "address": "4wyusm2nxncvNyiVej5mZ1k1mVPhfG41oNvHx4BWiYMq"
        },
        {
          "name": "policyEngine",
          "writable": true
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "revokeTokens",
      "docs": [
        "revoke shares of the rwa asset"
      ],
      "discriminator": [
        215,
        42,
        15,
        134,
        173,
        80,
        33,
        21
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "assetMint",
          "writable": true
        },
        {
          "name": "assetController",
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
          "name": "authorityTokenAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "assetController"
              },
              {
                "kind": "account",
                "path": "tokenProgram"
              },
              {
                "kind": "account",
                "path": "assetMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "revokeTokenAccount",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "seizeTokens",
      "docs": [
        "seize shares of the rwa asset"
      ],
      "discriminator": [
        79,
        30,
        69,
        54,
        78,
        1,
        16,
        23
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "assetMint"
        },
        {
          "name": "assetController",
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
          "name": "destinationTokenAccount",
          "writable": true
        },
        {
          "name": "sourceTokenAccount",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "thawTokenAccount",
      "docs": [
        "thaw token account"
      ],
      "discriminator": [
        199,
        172,
        96,
        93,
        244,
        252,
        137,
        171
      ],
      "accounts": [
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "assetMint",
          "writable": true,
          "relations": [
            "identityRegistryAccount"
          ]
        },
        {
          "name": "assetController",
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
          "name": "identityRegistryAccount",
          "writable": true
        },
        {
          "name": "tokenAccount",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        }
      ],
      "args": []
    },
    {
      "name": "updateInterestBearingMintRate",
      "docs": [
        "interest bearing mint rate update"
      ],
      "discriminator": [
        29,
        174,
        109,
        163,
        227,
        75,
        2,
        144
      ],
      "accounts": [
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "assetMint",
          "writable": true
        },
        {
          "name": "assetController",
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
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
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
      "args": [
        {
          "name": "rate",
          "type": "i16"
        }
      ]
    },
    {
      "name": "updateMetadata",
      "docs": [
        "edit metadata of the rwa asset"
      ],
      "discriminator": [
        170,
        182,
        43,
        239,
        97,
        78,
        225,
        186
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
          "name": "assetMint",
          "writable": true
        },
        {
          "name": "assetController",
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
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
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
      "args": [
        {
          "name": "args",
          "type": {
            "defined": {
              "name": "updateAssetMetadataArgs"
            }
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "assetControllerAccount",
      "discriminator": [
        70,
        136,
        149,
        138,
        12,
        87,
        52,
        105
      ]
    },
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
  "events": [
    {
      "name": "assetMetadataEvent",
      "discriminator": [
        90,
        19,
        200,
        229,
        103,
        82,
        218,
        16
      ]
    },
    {
      "name": "extensionMetadataEvent",
      "discriminator": [
        22,
        198,
        253,
        69,
        234,
        122,
        248,
        117
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "transferMintNotApproved",
      "msg": "Transfer hasnt been approved for the asset mint"
    },
    {
      "code": 6001,
      "name": "transferFromNotApproved",
      "msg": "Transfer hasnt been approved for from account"
    },
    {
      "code": 6002,
      "name": "transferToNotApproved",
      "msg": "Transfer hasnt been approved for to account"
    },
    {
      "code": 6003,
      "name": "transferAmountNotApproved",
      "msg": "Transfer hasnt been approved for the specified amount"
    },
    {
      "code": 6004,
      "name": "invalidPolicyAccount",
      "msg": "Invalid policy account passed"
    },
    {
      "code": 6005,
      "name": "transferSlotNotApproved",
      "msg": "Invalid slot for approve account"
    },
    {
      "code": 6006,
      "name": "transferHistoryFull",
      "msg": "Transfer history is full"
    },
    {
      "code": 6007,
      "name": "unauthorized",
      "msg": "unauthorized"
    },
    {
      "code": 6008,
      "name": "invalidPdaPassedIn",
      "msg": "Pda passed in for transfer is wrong"
    },
    {
      "code": 6009,
      "name": "tokenAccountNotInitialized",
      "msg": "Token account is not initialized"
    },
    {
      "code": 6010,
      "name": "tokenAccountAlreadyInitialized",
      "msg": "Token account is already initialized"
    }
  ],
  "types": [
    {
      "name": "assetControllerAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "version",
            "type": "u8"
          },
          {
            "name": "assetMint",
            "docs": [
              "mint pubkey"
            ],
            "type": "pubkey"
          },
          {
            "name": "authority",
            "docs": [
              "authority has the ability to change delegate, freeze token accounts, etc."
            ],
            "type": "pubkey"
          },
          {
            "name": "delegate",
            "docs": [
              "delegate has the ability to generate tranasction approval accounts,",
              "by default points to self, which allows any programs to generate an approval account",
              "update to any other account to control cpis"
            ],
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "assetMetadataEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mint",
            "type": "string"
          },
          {
            "name": "name",
            "type": {
              "option": "string"
            }
          },
          {
            "name": "symbol",
            "type": {
              "option": "string"
            }
          },
          {
            "name": "uri",
            "type": {
              "option": "string"
            }
          },
          {
            "name": "decimals",
            "type": {
              "option": "u8"
            }
          }
        ]
      }
    },
    {
      "name": "comparisionType",
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
          },
          {
            "name": "except"
          }
        ]
      }
    },
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
      "name": "counterpartyFilter",
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
            "name": "both"
          }
        ]
      }
    },
    {
      "name": "createAssetControllerArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "decimals",
            "type": "u8"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "symbol",
            "type": "string"
          },
          {
            "name": "uri",
            "type": "string"
          },
          {
            "name": "delegate",
            "type": {
              "option": "pubkey"
            }
          },
          {
            "name": "interestRate",
            "type": {
              "option": "i16"
            }
          },
          {
            "name": "allowMultipleWallets",
            "type": {
              "option": "bool"
            }
          },
          {
            "name": "enforcePolicyIssuance",
            "type": {
              "option": "bool"
            }
          }
        ]
      }
    },
    {
      "name": "extensionMetadataEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "address",
            "type": "string"
          },
          {
            "name": "extensionType",
            "type": "u8"
          },
          {
            "name": "metadata",
            "type": "bytes"
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
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "identityLevels",
            "type": {
              "array": [
                "u8",
                10
              ]
            }
          },
          {
            "name": "comparisionType",
            "type": {
              "defined": {
                "name": "comparisionType"
              }
            }
          },
          {
            "name": "counterpartyFilter",
            "type": {
              "defined": {
                "name": "counterpartyFilter"
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
    },
    {
      "name": "updateAssetMetadataArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "name",
            "type": {
              "option": "string"
            }
          },
          {
            "name": "symbol",
            "type": {
              "option": "string"
            }
          },
          {
            "name": "uri",
            "type": {
              "option": "string"
            }
          }
        ]
      }
    }
  ]
};
