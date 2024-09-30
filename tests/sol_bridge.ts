import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolBridge } from "../target/types/sol_bridge";

import { TOKEN_PROGRAM_ID, createAccount, createInitializeMintInstruction, MINT_SIZE, getMinimumBalanceForRentExemptMint, createMint, createAssociatedTokenAccount, getAssociatedTokenAddress, ASSOCIATED_TOKEN_PROGRAM_ID, mintTo, mintToChecked, getAccount, getMint, getAssociatedTokenAddressSync, createAssociatedTokenAccountInstruction, createMintToCheckedInstruction } from "@solana/spl-token";
import * as bs58 from "bs58";
import { SystemProgram, Keypair, PublicKey, Transaction, SYSVAR_RENT_PUBKEY, SYSVAR_CLOCK_PUBKEY, Connection, clusterApiUrl, sendAndConfirmTransaction } from "@solana/web3.js";
import assert from "assert";
import {
  PythSolanaReceiver,
  InstructionWithEphemeralSigners,
} from "@pythnetwork/pyth-solana-receiver";
import { PriceServiceConnection } from "@pythnetwork/price-service-client";
// import Wallet from "@project-serum/anchor";
import { Wallet } from '@project-serum/anchor';

describe("sol_bridge", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolBridge as Program<SolBridge>;

  let bridge, vault: PublicKey;
  let bridgeBump, vaultBump: Number;

  // Bmed1qoe6u8VxmJ5p6SW77fb7LiSqWmQdTtKTz5dyh62
  let owner = Keypair.fromSecretKey(bs58.decode("2LU9Gir9pDVEsUWrRHLUUdPaVM642EmMGubgyZg2LNYk1uyD4LNRR5HshCENmfTUD3nPMeN7FCJKxEdu48YSEpta"));

  let user = Keypair.fromSecretKey(bs58.decode("3wYcRWgA7kpb7E931PTrbNo8LfnCBNvck7KsK2MscrA8WbiiuzsbwumFb7EeqV6S8Cpc5u7zjPtxaTRL13xaAY5P"));

  type Event = anchor.IdlEvents<typeof program["idl"]>;

  it("Get PDA", async() => {
    [bridge, bridgeBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("BRIDGE_SEED")
      ],
      program.programId
    );

    [vault, vaultBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("VAULT_SEED")
      ],
      program.programId
    );
  });
  /*
  it("Is initialized!", async () => {
    // Add your test here.
    const protocolFee = 100;
    const tx = await program.rpc.initialize(
      new anchor.BN(protocolFee),
      {
        accounts: {
          bridge,
          owner: owner.publicKey,
          vault,
          systemProgram: SystemProgram.programId
        },
        signers: [owner]
      }
    );
    console.log("tx->", tx);
  });

  it("set protocol fee", async() => {
    const protocolFee = 10; // 10 USDC
    const tx = await program.rpc.setProtocolFee(
     new anchor.BN(protocolFee),
     {
       accounts: {
         owner: owner.publicKey,
         bridge,
       },
       signers: [owner]
     }
    );
    console.log("tx->", tx);
  });
  it("add bridgeable token to the bridge", async() => {
    let bridgeData = await program.account.bridge.fetch(bridge);
    const tokenId = 1;
    const targetChainSelector = 1;
    const tokenAddress = new PublicKey("8NtheYSKWDkCgWoc8HScQFkcCTF1FiFEbbriosZLNmtE");

    try {
      let listenerId: number;
      const event = await new Promise<Event[E]>(async (res) => {
        listenerId = program.addEventListener("AddTokenEvent", (event) => {
          res(event);
        });
        const tx = await program.rpc.addToken(
          tokenId, 
          targetChainSelector,
          tokenAddress, {
            accounts: {
              owner: owner.publicKey,
              bridge
            },
            signers: [owner]
          }
        );
        console.log("tx->", tx);
        bridgeData = await program.account.bridge.fetch(bridge);
        console.log("tokens->", bridgeData);
      });
      await program.removeEventListener(listenerId);
      console.log(event);
    } catch (error) {
      console.log(error);
    }
  });

  it("add bridgeable token to the bridge", async() => {
    let bridgeData = await program.account.bridge.fetch(bridge);
    console.log("tokens->", bridgeData);
    const tokenId = 2;
    const targetChainSelector = 1;
    const tokenAddress = new PublicKey("5hyJ6h3ABjF7zEBhc32LWT5ZUCkNx4AZkdRzKC1MUHRb");

    try {
      let listenerId: number;
      const event = await new Promise<Event[E]>(async (res) => {
        listenerId = program.addEventListener("AddTokenEvent", (event) => {
          res(event);
        });
        const tx = await program.rpc.addToken(
          tokenId, 
          targetChainSelector,
          tokenAddress, {
            accounts: {
              owner: owner.publicKey,
              bridge
            },
            signers: [owner]
          }
        );
        console.log("tx->", tx);
        bridgeData = await program.account.bridge.fetch(bridge);
        console.log("tokens->", bridgeData);
      });
      await program.removeEventListener(listenerId);
      console.log(event);
    } catch (error) {
      console.log(error);
    }
  });

  it("remove bridgeable token from the bridge", async() => {
    let bridgeData = await program.account.bridge.fetch(bridge);
    console.log("tokens->", bridgeData);
    try {
      const tokenId = 2;
      const targetChainSelector = 1;
      let listenerId: number;
      const event = await new Promise<Event[E]>(async (res) => {
        listenerId = program.addEventListener("RemoveTokenEvent", (event) => {
          res(event);
        });
        const tx = await program.rpc.removeToken(
          tokenId, 
          targetChainSelector,
          {
            accounts: {
              owner: owner.publicKey,
              bridge
            },
            signers: [owner]
          }
        );
        console.log("tx->", tx);
        bridgeData = await program.account.bridge.fetch(bridge);
        console.log("tokens->", bridgeData.tokens);
      });
      await program.removeEventListener(listenerId);
      console.log(event);
    } catch (error) {
      console.log(error);
    }
  });

  it("add liquidity by owner", async() => {
    const tokenMint = new PublicKey("8NtheYSKWDkCgWoc8HScQFkcCTF1FiFEbbriosZLNmtE");

    const tokenId = 1;
    const targetChainSelector = 1;
    const amount = 100000000;

    const tokenAccount = await getAssociatedTokenAddress(
      tokenMint,
      owner.publicKey
    );

    const [bridgeTokenAccount, _] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("BRIDGE_TOKEN_VAULT_SEED"),
        tokenMint.toBuffer()
      ],
      program.programId
    );
    try {
      let listenerId: number;
      const event = await new Promise<Event[E]>(async (res) => {
        listenerId = program.addEventListener("AddLiquidityEvent", (event) => {
          res(event);
        });
        const tx = await program.rpc.addLiquidity(
          tokenId,
          targetChainSelector,
          new anchor.BN(amount), {
            accounts: {
              user: owner.publicKey,
              bridge,
              tokenMint,
              tokenAccount,
              bridgeTokenAccount,
              tokenProgram: TOKEN_PROGRAM_ID,
              systemProgram: SystemProgram.programId
            },
            signers: [owner]
          }
        );
        console.log("tx->", tx);
        const bridgeData = await program.account.bridge.fetch(bridge);
        console.log("tokens->", bridgeData);
      });
      await program.removeEventListener(listenerId);
      console.log(event);
    } catch (error) {
      console.log(error);
    }
  });

  it("update target token's balance", async() => {
    let bridgeData = await program.account.bridge.fetch(bridge);
    console.log("tokens->", bridgeData.tokens);
    const tokenId = 1;
    const targetChainSelector = 1;
    const tokenAmount = 1000000000;

    try {
      const tx = await program.rpc.updateTokenBalance(
        tokenId, 
        targetChainSelector,
        new anchor.BN(tokenAmount), 
        true,
        {
          accounts: {
            owner: owner.publicKey,
            bridge
          },
          signers: [owner]
        }
      );
      console.log("tx->", tx);
      bridgeData = await program.account.bridge.fetch(bridge);
      console.log("tokens->", bridgeData);
    } catch (error) {
      console.log(error);
    }
  });
  */

  it("send tokens to the bridge", async() => {
    const tokenMint = new PublicKey("8NtheYSKWDkCgWoc8HScQFkcCTF1FiFEbbriosZLNmtE");

    const tokenId = 1;
    const targetChainSelector = 1;
    const sendAmount = 10000000;

    const tokenAccount = await getAssociatedTokenAddress(
      tokenMint,
      user.publicKey
    );

    const [bridgeTokenAccount, _] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("BRIDGE_TOKEN_VAULT_SEED"),
        tokenMint.toBuffer()
      ],
      program.programId
    );

    const SOL_PRICE_FEED_ID =
      "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
    const HERMES_URL = "https://hermes.pyth.network/";
    const DEVNET_RPC_URL = "https://api.devnet.solana.com";

    const priceServiceConnection = new PriceServiceConnection(HERMES_URL, {
      priceFeedRequestConfig: { binary: true },
    });

    const wallet = new Wallet(owner);

    const priceUpdateData = await priceServiceConnection.getLatestVaas([
      SOL_PRICE_FEED_ID,
    ]);

    const pythSolanaReceiver = new PythSolanaReceiver({
      connection: program.provider.connection,
      wallet: wallet,
    });

    const transactionBuilder = pythSolanaReceiver.newTransactionBuilder({
      closeUpdateAccounts: true,
    });
    await transactionBuilder.addPostPriceUpdates([priceUpdateData[0]]);

    await transactionBuilder.addPriceConsumerInstructions(
      async (
        getPriceUpdateAccount: (priceFeedId: string) => PublicKey
      ): Promise<InstructionWithEphemeralSigners[]> => {
        return [
          {
            instruction: await program.methods
              .send(
                tokenId,
                targetChainSelector,
                new anchor.BN(sendAmount),
              )
              .accounts({
                user: user.publicKey,
                bridge,
                vault,
                tokenMint,
                tokenAccount,
                bridgeTokenAccount,
                priceUpdate: getPriceUpdateAccount(SOL_PRICE_FEED_ID),
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
              })
              .instruction(),
            signers: [],
          },
        ];
      }
    );

    await pythSolanaReceiver.provider.sendAll(
      await transactionBuilder.buildVersionedTransactions({
        computeUnitPriceMicroLamports: 50000,
      }),
      { skipPreflight: true }
    );
  });
  /*

  it("message receive", async() => {
    const tokenMint = new PublicKey("8NtheYSKWDkCgWoc8HScQFkcCTF1FiFEbbriosZLNmtE");

    const tokenId = 1;
    const targetChainSelector = 1;
    const sendAmount = 10000000;

    const tokenAccount = await getAssociatedTokenAddress(
      tokenMint,
      user.publicKey
    );

    const [bridgeTokenAccount, _] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("BRIDGE_TOKEN_VAULT_SEED"),
        tokenMint.toBuffer()
      ],
      program.programId
    );

    try {
      let listenerId: number;
      const event = await new Promise<Event[E]>(async (res) => {
        listenerId = program.addEventListener("MessageReceivedEvent", (event) => {
          res(event);
        });
        const tx = await program.rpc.messageReceive(
          tokenId,
          targetChainSelector,
          new anchor.BN(sendAmount),
          {
            accounts: {
              owner: owner.publicKey,
              bridge,
              tokenMint,
              user: user.publicKey,
              userTokenAccount:tokenAccount,
              bridgeTokenAccount,
              associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
              tokenProgram: TOKEN_PROGRAM_ID,
              systemProgram: SystemProgram.programId
            },
            signers:[owner]
          }
        );
        console.log("tx->", tx);
      });
      await program.removeEventListener(listenerId);
      console.log(event);
    } catch (error) {
      console.log(error);
    }
  });

  it("withdraw Token", async() => {
    const tokenMint = new PublicKey("8NtheYSKWDkCgWoc8HScQFkcCTF1FiFEbbriosZLNmtE");

    const tokenId = 1;
    const targetChainSelector = 1;
    const withdrawAmount = 10000000;

    const tokenAccount = await getAssociatedTokenAddress(
      tokenMint,
      owner.publicKey
    );

    const [bridgeTokenAccount, _] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("BRIDGE_TOKEN_VAULT_SEED"),
        tokenMint.toBuffer()
      ],
      program.programId
    );

    try {
      let listenerId: number;
      const event = await new Promise<Event[E]>(async (res) => {
        listenerId = program.addEventListener("WithdrawTokenEvent", (event) => {
          res(event);
        });
        const tx = await program.rpc.withdrawToken(
          tokenId,
          targetChainSelector,
          new anchor.BN(withdrawAmount),{
            accounts: {
              bridge,
              owner: owner.publicKey,
              tokenMint,
              bridgeTokenAccount,
              beneficiaryTokenAccount: tokenAccount,
              tokenProgram: TOKEN_PROGRAM_ID,
              associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
              systemProgram: SystemProgram.programId
            },
            signers: [owner]
          }
        );
        console.log("tx->", tx);
      });
      await program.removeEventListener(listenerId);
      console.log(event);
    } catch (error) {
      console.log(error);
    }
  });

  it("withdraw fee sol", async() => {
    const withdrawAmount = 100000;

    try {
      let listenerId: number;
      const event = await new Promise<Event[E]>(async (res) => {
        listenerId = program.addEventListener("WithdrawEvent", (event) => {
          res(event);
        });
        const tx = await program.rpc.withdraw(
          new anchor.BN(withdrawAmount),{
            accounts: {
              owner: owner.publicKey,
              bridge,
              vault,
              beneficiary: owner.publicKey,
              systemProgram: SystemProgram.programId
            },
            signers: [owner]
          }
        );
        console.log("tx->", tx);
      });
      await program.removeEventListener(listenerId);
      console.log(event);
    } catch (error) {
      console.log(error);
    }
  });
  */
});
