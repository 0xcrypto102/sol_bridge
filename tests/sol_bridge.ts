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

  let chainSelector = 1601511254; // test value, you can modify value in your product

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
  
  it("Get Token ID", async() => {
    [bridge, bridgeBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("BRIDGE_SEED")
      ],
      program.programId
    );
    const bridgeData = await program.account.bridge.fetch(bridge);

    const tokenIds = bridgeData.tokenIds;
    const tokenAddresses = bridgeData.tokenAddresses;
    const targetTokenAddresses = bridgeData.targetTokenAddresses;
    const targetChainSelectors = bridgeData.targetChainSelectors;

    const remoteChainSelector = 56;
    const localToken = new PublicKey("8NtheYSKWDkCgWoc8HScQFkcCTF1FiFEbbriosZLNmtE");
    const remoteToken = "0x55d398326f99059fF775485246999027B3197955"; // bsc usdt address

    for(let i = 0; i<tokenIds.length; i++) {
      if(targetTokenAddresses[i] == remoteToken && Number(targetChainSelectors[i]) == remoteChainSelector && tokenAddresses[i].toString() == localToken.toString()) {
        const tokenId = tokenIds[i];

        console.log(tokenId)
      }
    }
  });

  */
  it("Is initialized!", async () => {
    // Add your test here.
    const protocolFee = 100;
    const tx = await program.rpc.initialize(
      new anchor.BN(protocolFee),
      new anchor.BN(chainSelector),
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
    const protocolFee = 10000000; // 0.01 Sol
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
/*
  it("add bridgeable token to the bridge", async() => {
    let bridgeData = await program.account.bridge.fetch(bridge);
    const remoteChainSelector = 56;
    const localToken = new PublicKey("8NtheYSKWDkCgWoc8HScQFkcCTF1FiFEbbriosZLNmtE");
    const remoteToken = "0x55d398326f99059fF775485246999027B3197955"; // bsc usdt address

    try {
      let listenerId: number;
      const event = await new Promise<Event[E]>(async (res) => {
        listenerId = program.addEventListener("AddTokenEvent", (event) => {
          res(event);
        });
        const tx = await program.rpc.addToken(
          localToken, 
          new anchor.BN(remoteChainSelector),
          remoteToken, {
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
    const remoteChainSelector = 1;
    const localToken = new PublicKey("5hyJ6h3ABjF7zEBhc32LWT5ZUCkNx4AZkdRzKC1MUHRb");
    const remoteToken = "0xdac17f958d2ee523a2206206994597c13d831ec7"; //eth usdt address

    try {
      let listenerId: number;
      const event = await new Promise<Event[E]>(async (res) => {
        listenerId = program.addEventListener("AddTokenEvent", (event) => {
          res(event);
        });
        const tx = await program.rpc.addToken(
          localToken, 
          new anchor.BN(remoteChainSelector),
          remoteToken, {
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
      const remoteChainSelector = 1;
      const localToken = new PublicKey("5hyJ6h3ABjF7zEBhc32LWT5ZUCkNx4AZkdRzKC1MUHRb");
      const remoteToken = "0xdac17f958d2ee523a2206206994597c13d831ec7"; //eth usdt address
  
      let listenerId: number;
      const event = await new Promise<Event[E]>(async (res) => {
        listenerId = program.addEventListener("RemoveTokenEvent", (event) => {
          res(event);
        });
        const tx = await program.rpc.removeToken(
          localToken,
          new anchor.BN(remoteChainSelector),
          remoteToken, 
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

    const remoteChainSelector = 56;
    const localToken = new PublicKey("8NtheYSKWDkCgWoc8HScQFkcCTF1FiFEbbriosZLNmtE");
    const remoteToken = "0x55d398326f99059fF775485246999027B3197955"; // bsc usdt address

    const amount = 100000000;

    const tokenAccount = await getAssociatedTokenAddress(
      localToken,
      owner.publicKey
    );

    const [bridgeTokenAccount, _] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("BRIDGE_TOKEN_VAULT_SEED"),
        localToken.toBuffer()
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
          new anchor.BN(amount), 
          new anchor.BN(remoteChainSelector),
          remoteToken, {
            accounts: {
              user: owner.publicKey,
              bridge,
              localToken,
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

    const remoteChainSelector = 56;
    const localToken = new PublicKey("8NtheYSKWDkCgWoc8HScQFkcCTF1FiFEbbriosZLNmtE");
    const remoteToken = "0x55d398326f99059fF775485246999027B3197955"; // bsc usdt address
    const tokenAmount = 1000000000;

    try {
      const tx = await program.rpc.updateTokenBalance(
        localToken,
        new anchor.BN(remoteChainSelector),
        remoteToken,
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

  it("send tokens to the bridge", async() => {
    const localToken = new PublicKey("8NtheYSKWDkCgWoc8HScQFkcCTF1FiFEbbriosZLNmtE");
    const remoteToken = "0x55d398326f99059fF775485246999027B3197955"; // bsc usdt address
    const remoteBridge = "brigeaddress0x2394290389082395234"; // test value, modify this value in product
    const remoteChainSelector = 56;
    const sendAmount = 10000000;

    const tokenAccount = await getAssociatedTokenAddress(
      localToken,
      user.publicKey
    );

    const [bridgeTokenAccount, _] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("BRIDGE_TOKEN_VAULT_SEED"),
        localToken.toBuffer()
      ],
      program.programId
    );

    const tx = await program.rpc.send(
      new anchor.BN(sendAmount),
      remoteBridge,
      new anchor.BN(remoteChainSelector),
      remoteToken,
      {
      accounts: {
        user: user.publicKey,
        bridge,
        vault,
        tokenMint: localToken,
        tokenAccount,
        bridgeTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      },
      signers: [user]
    });
    console.log("tx->", tx);
  });

  it("message receive", async() => {
    const localToken = new PublicKey("5SUDTjKUQ6RBZ5nED3VcMCtUKAFhmJ4b5Ar4Yodpn7au");

    const tokenId = '64373931313736393231353231316439646438656234356161643466316333626166616434316362393765356231373039373365646162366336666535376537';
    const remoteChainSelector = Number('b8159170038f96fb');
    const sendAmount = 10000000;

    const tokenAccount = await getAssociatedTokenAddress(
      localToken,
      user.publicKey
    );

    const [bridgeTokenAccount, _] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("BRIDGE_TOKEN_VAULT_SEED"),
        localToken.toBuffer()
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
          new anchor.BN(remoteChainSelector),
          new anchor.BN(sendAmount),
          {
            accounts: {
              owner: owner.publicKey,
              bridge,
              tokenMint: localToken,
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
    const localToken = new PublicKey("8NtheYSKWDkCgWoc8HScQFkcCTF1FiFEbbriosZLNmtE");

    const tokenId = '62363130373235323438643362363237633363386366386236666634616637663939646436353736376165316537663630653161626361653263363132643565';
    const withdrawAmount = 10000000;

    const tokenAccount = await getAssociatedTokenAddress(
      localToken,
      owner.publicKey
    );

    const [bridgeTokenAccount, _] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("BRIDGE_TOKEN_VAULT_SEED"),
        localToken.toBuffer()
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
          new anchor.BN(withdrawAmount),{
            accounts: {
              bridge,
              owner: owner.publicKey,
              tokenMint: localToken,
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
