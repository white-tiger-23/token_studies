const anchor = require('@project-serum/anchor');
const spl = require('@solana/spl-token');
const { BN } = require('bn.js');
const fs = require('fs');

const idl = JSON.parse(fs.readFileSync('./target/idl/token_studies.json', 'utf8'));
const keypair_json = JSON.parse(fs.readFileSync('../../../.config/solana/id.json', 'utf8'));


async function initializePool() {
    const programId = new anchor.web3.PublicKey(idl.metadata.address);
    const [programAddress, programAddressBump] = await anchor.web3.PublicKey.findProgramAddress(["programauthority"], programId);
    const opts = {
        preflightCommitment: "processed"
    }
    const network = "http://127.0.0.1:8899";
    const connection = new anchor.web3.Connection(network, opts.preflightCommitment);
    wallet = new anchor.Wallet(anchor.web3.Keypair.fromSecretKey(Buffer.from(keypair_json)));
    const provider = new anchor.Provider(connection, wallet, opts.preflightCommitment);
    const tokenstudies = new anchor.Program(idl, programId, provider);
    
    if (!await provider.connection.getAccountInfo(programAddress)){
        try {
            await tokenstudies.rpc.initializeAuthority(
                new anchor.BN(programAddressBump),
                {
                accounts: {
                    admin: provider.wallet.publicKey,
                    state: programAddress,
                    rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                    systemProgram: anchor.web3.SystemProgram.programId
                },
                }
            );

        } catch (err) {
            console.log("Transaction failed with error : ", err);
        }
    }
    const programStateAccount = await tokenstudies.account.programAuthority.fetch(programAddress);
    console.log("Program state admin : ", programStateAccount.authority.toString());
    console.log("Admin provided to rpc call : ", provider.wallet.publicKey.toString());
    console.log("Program state bump : ", programStateAccount.bump);

}

initializePool();