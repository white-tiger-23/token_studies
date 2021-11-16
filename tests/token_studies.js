const anchor = require('@project-serum/anchor');
const spl = require('@solana/spl-token');
const { BN } = require('bn.js');

describe('token_studies', () => {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.local();
  anchor.setProvider(provider);
  let programAuthority;
  let programAuthorityBump;
  const tokenstudies = anchor.workspace.TokenStudies;
  let tokenMintAuthority;
  let tokenMint;
  let poolAddress, poolBump;
  let poolTokenAddress;
  let voucherMintAddress;
  let userTokenAccount;
  const TOKEN_DECIMALS = 6;

  before(async () => {
    const [_programAuthorityAddress, _programAuthorityBump] = await anchor.web3.PublicKey.findProgramAddress(
                                                      ["programauthority"], 
                                                      tokenstudies.programId);
    //console.log("Program ID :", tokenstudies.programId);
    programAddress = _programAuthorityAddress;
    programAuthorityBump = _programAuthorityBump;
    
    //Setup the token mint account 
    tokenMintAuthority = anchor.web3.Keypair.generate();
    tokenMint = await spl.Token.createMint(
      provider.connection,
      provider.wallet.payer,
      tokenMintAuthority.publicKey,
      null,
      TOKEN_DECIMALS,
      spl.TOKEN_PROGRAM_ID
    );
    
    //console.log("Token mint authority : ", (await tokenMint.getMintInfo()).mintAuthority.toBase58());
    //console.log("Token mint authority pubkey: ", tokenMintAuthority.publicKey.toBase58());
    
    
    //Airdrop some sol to the mint authority
    let airdropSignature = await provider.connection.requestAirdrop(tokenMintAuthority.publicKey, 
                                                                    100*anchor.web3.LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction(airdropSignature);


    //Check the balance of token account
    userTokenAccount = await tokenMint.getOrCreateAssociatedAccountInfo(provider.wallet.publicKey);
    console.log("User token account : ", userTokenAccount.address.toString());
    console.log("User token balance : ", userTokenAccount.amount.toNumber());


    //Mint some tokens to the user
    await tokenMint.mintTo(
      userTokenAccount.address,
      tokenMintAuthority,
      [],
      100 * 10 ** TOKEN_DECIMALS,
    );


    //Find the PDA for the pool
    const[_poolAddress, _poolBump] = await anchor.web3.PublicKey.findProgramAddress(["pool", tokenMint.publicKey.toBuffer()],
                                                                                    tokenstudies.programId);
    poolAddress = _poolAddress;
    poolBump = _poolBump;

    //Find the address of the pool token
    const[_poolTokenAddress] = await anchor.web3.PublicKey.findProgramAddress(["token", tokenMint.publicKey.toBuffer()],
                                                                                tokenstudies.programId);
    poolTokenAddress = _poolTokenAddress;

    const[_voucherMintAddress] = await anchor.web3.PublicKey.findProgramAddress(["voucher", tokenMint.publicKey.toBuffer()],
                                                                                  tokenstudies.programId);
    voucherMintAddress = _voucherMintAddress;

    console.log("Pool address : ", poolAddress.toString());
    console.log("Pool bump : ", poolBump.toString());
  });


  it('Authority initialized!', async () => {
    console.log("------------Initializing program authority using admin account--------------");
    console.log("Program authority address :", programAddress.toBase58());
    console.log("Program authority bump :", programAuthorityBump);
    await tokenstudies.rpc.initializeAuthority(
      new anchor.BN(programAuthorityBump),
      {
        accounts: {
          admin: provider.wallet.publicKey,
          state: programAddress,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          systemProgram: anchor.web3.SystemProgram.programId
        },
      }
    );
    
    //Now read information from the program state account
    const programStateAccount = await tokenstudies.account.programAuthority.fetch(programAddress);
    console.log("Program bump : ", programStateAccount.bump);
    console.log("Program admin : ", programStateAccount.authority.toBase58());
    console.log("Provider waller pubkey : ", provider.wallet.publicKey.toBase58());


    console.log("---------Adding pool to program--------");
    await tokenstudies.rpc.addPool(
      new anchor.BN(poolBump),
      {
        accounts : {
          admin: provider.wallet.publicKey,
          state: programAddress,
          tokenMint: tokenMint.publicKey,
          pool: poolAddress,
          poolToken: poolTokenAddress,
          voucherMint: voucherMintAddress,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          tokenProgram: spl.TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId 
        }
      }
    );
    const poolAccount = await tokenstudies.account.pool.fetch(poolAddress);
    console.log("Pool bump : ", poolAccount.bump);
    console.log("Pool mint : ", poolAccount.tokenMintAddress.toString());
    console.log("Token mint as given to program : ", tokenMint.publicKey.toString());

    const poolTokenAccount = await tokenMint.getAccountInfo(poolTokenAddress);
    console.log("Pool token balance : ", poolTokenAccount.amount.toString());
    console.log("Pool token mint address : ", poolTokenAccount.mint.toString());

    let voucherMintAccountInfo = await provider.connection.getAccountInfo(voucherMintAddress);
    let voucherMintInfo = spl.MintLayout.decode(voucherMintAccountInfo.data);
    let voucherMintAuthority = new anchor.web3.PublicKey(voucherMintInfo.mintAuthority);
    console.log("Voucher mint authority: ", voucherMintAuthority.toString());
    
    console.log("---------Depositing user tokens into pool--------");
    let userVoucherAddress = await spl.Token.getAssociatedTokenAddress(
      spl.ASSOCIATED_TOKEN_PROGRAM_ID,
      spl.TOKEN_PROGRAM_ID,
      voucherMintAddress,
      provider.wallet.publicKey
    );
    
    userTokenAccount = await tokenMint.getOrCreateAssociatedAccountInfo(provider.wallet.publicKey);
    // let userVoucherAccountInfo = await provider.connection.getAccountInfo(userVoucherAddress);
    // let userVoucherAccount = spl.AccountLayout.decode(userVoucherAccountInfo.data);
    // userVoucherAccount.amount = spl.u64.fromBuffer(userVoucherAccount.amount);
    console.log("User token account balance before deposit : ", userTokenAccount.amount.toNumber());
    //console.log("User voucher account : ", userVoucherAccount.amount.toNumber());
    let amount_deposited = 10 * 10 ** TOKEN_DECIMALS;

    await tokenstudies.rpc.deposit(
      new anchor.BN(amount_deposited),
      {
        accounts :{
          depositor: provider.wallet.publicKey,
          state: programAddress,
          pool: poolAddress,
          depositorTokenAccount: userTokenAccount.address,
          poolTokenAccount: poolTokenAddress,
          voucherMint: voucherMintAddress,
          depositorVoucherAccount: userVoucherAddress,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          tokenProgram: spl.TOKEN_PROGRAM_ID,
          associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId  
        }
      }
    );
    
    userTokenAccount = await tokenMint.getOrCreateAssociatedAccountInfo(provider.wallet.publicKey);
    userVoucherAccountInfo = await provider.connection.getAccountInfo(userVoucherAddress);
    userVoucherAccount = spl.AccountLayout.decode(userVoucherAccountInfo.data);
    userVoucherAccount.amount = spl.u64.fromBuffer(userVoucherAccount.amount);
    console.log("User token account balance after deposit : ", userTokenAccount.amount.toNumber());
    console.log("User voucher account balance after deposit : ", userVoucherAccount.amount.toNumber());
    

    console.log("---------Withdrawing user tokens from pool--------");
    userTokenAccount = await tokenMint.getOrCreateAssociatedAccountInfo(provider.wallet.publicKey);
    console.log("User token account balance before withdrawal : ", userTokenAccount.amount.toNumber());
    userVoucherAccount = spl.AccountLayout.decode((await provider.connection.getAccountInfo(userVoucherAddress)).data);
    userVoucherAccount.amount = spl.u64.fromBuffer(userVoucherAccount.amount);
    console.log("User voucher account balance before withdrawal : ", userVoucherAccount.amount.toNumber());
    
    let amount_withdrawn = 10 * 10 **TOKEN_DECIMALS;

    //Make the rpc call to withdraw user tokens from the pool
    await tokenstudies.rpc.withdraw(
      new anchor.BN(amount_withdrawn),
      {
        accounts : {
          depositor: provider.wallet.publicKey,
          state: programAddress,
          pool: poolAddress,
          depositorTokenAccount: userTokenAccount.address,
          poolTokenAccount: poolTokenAddress,
          voucherMint: voucherMintAddress,
          depositorVoucherAccount: userVoucherAddress,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          tokenProgram: spl.TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId
        }
      }
    )

    userTokenAccount = await tokenMint.getOrCreateAssociatedAccountInfo(provider.wallet.publicKey);
    console.log("User token account balance after withdrawal : ", userTokenAccount.amount.toNumber());
    userVoucherAccount = spl.AccountLayout.decode((await provider.connection.getAccountInfo(userVoucherAddress)).data);
    userVoucherAccount.amount = spl.u64.fromBuffer(userVoucherAccount.amount);
    console.log("User voucher account balance after withdrawal : ", userVoucherAccount.amount.toNumber());
    

  });

});
