#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, SetAuthority, Token, TokenAccount, Transfer, Mint};
/// The static program ID
pub static ID: anchor_lang::solana_program::pubkey::Pubkey =
    anchor_lang::solana_program::pubkey::Pubkey::new_from_array([
        81u8, 50u8, 169u8, 111u8, 159u8, 233u8, 240u8, 103u8, 44u8, 137u8, 134u8, 109u8, 165u8,
        132u8, 224u8, 154u8, 103u8, 117u8, 57u8, 209u8, 163u8, 34u8, 148u8, 16u8, 51u8, 183u8,
        140u8, 204u8, 101u8, 181u8, 154u8, 177u8,
    ]);
/// Confirms that a given pubkey is equivalent to the program ID
pub fn check_id(id: &anchor_lang::solana_program::pubkey::Pubkey) -> bool {
    id == &ID
}
/// Returns the program ID
pub fn id() -> anchor_lang::solana_program::pubkey::Pubkey {
    ID
}
static PROGRAMSEED: &[u8] = b"programauthority";
static POOLSEED: &[u8] = b"pool";
use token_studies::*;
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
    let (program_id, accounts, instruction_data) =
        unsafe { ::solana_program::entrypoint::deserialize(input) };
    match entry(&program_id, &accounts, &instruction_data) {
        Ok(()) => ::solana_program::entrypoint::SUCCESS,
        Err(error) => error.into(),
    }
}
/// The Anchor codegen exposes a programming model where a user defines
/// a set of methods inside of a `#[program]` module in a way similar
/// to writing RPC request handlers. The macro then generates a bunch of
/// code wrapping these user defined methods into something that can be
/// executed on Solana.
///
/// These methods fall into one of three categories, each of which
/// can be considered a different "namespace" of the program.
///
/// 1) Global methods - regular methods inside of the `#[program]`.
/// 2) State methods - associated methods inside a `#[state]` struct.
/// 3) Interface methods - methods inside a strait struct's
///    implementation of an `#[interface]` trait.
///
/// Care must be taken by the codegen to prevent collisions between
/// methods in these different namespaces. For this reason, Anchor uses
/// a variant of sighash to perform method dispatch, rather than
/// something like a simple enum variant discriminator.
///
/// The execution flow of the generated code can be roughly outlined:
///
/// * Start program via the entrypoint.
/// * Strip method identifier off the first 8 bytes of the instruction
///   data and invoke the identified method. The method identifier
///   is a variant of sighash. See docs.rs for `anchor_lang` for details.
/// * If the method identifier is an IDL identifier, execute the IDL
///   instructions, which are a special set of hardcoded instructions
///   baked into every Anchor program. Then exit.
/// * Otherwise, the method identifier is for a user defined
///   instruction, i.e., one of the methods in the user defined
///   `#[program]` module. Perform method dispatch, i.e., execute the
///   big match statement mapping method identifier to method handler
///   wrapper.
/// * Run the method handler wrapper. This wraps the code the user
///   actually wrote, deserializing the accounts, constructing the
///   context, invoking the user's code, and finally running the exit
///   routine, which typically persists account changes.
///
/// The `entry` function here, defines the standard entry to a Solana
/// program, where execution begins.
#[cfg(not(feature = "no-entrypoint"))]
pub fn entry(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    if data.len() < 8 {
        return Err(anchor_lang::__private::ErrorCode::InstructionMissing.into());
    }
    dispatch(program_id, accounts, data).map_err(|e| {
        ::solana_program::log::sol_log(&e.to_string());
        e
    })
}
pub mod program {
    use super::*;
    /// Type representing the program.
    pub struct TokenStudies;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for TokenStudies {
        #[inline]
        fn clone(&self) -> TokenStudies {
            match *self {
                TokenStudies => TokenStudies,
            }
        }
    }
    impl anchor_lang::AccountDeserialize for TokenStudies {
        fn try_deserialize(
            buf: &mut &[u8],
        ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError>
        {
            TokenStudies::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(
            _buf: &mut &[u8],
        ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError>
        {
            Ok(TokenStudies)
        }
    }
    impl anchor_lang::Id for TokenStudies {
        fn id() -> Pubkey {
            ID
        }
    }
}
/// Performs method dispatch.
///
/// Each method in an anchor program is uniquely defined by a namespace
/// and a rust identifier (i.e., the name given to the method). These
/// two pieces can be combined to creater a method identifier,
/// specifically, Anchor uses
///
/// Sha256("<namespace>::<rust-identifier>")[..8],
///
/// where the namespace can be one of three types. 1) "global" for a
/// regular instruction, 2) "state" for a state struct instruction
/// handler and 3) a trait namespace (used in combination with the
/// `#[interface]` attribute), which is defined by the trait name, e..
/// `MyTrait`.
///
/// With this 8 byte identifier, Anchor performs method dispatch,
/// matching the given 8 byte identifier to the associated method
/// handler, which leads to user defined code being eventually invoked.
fn dispatch(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let mut ix_data: &[u8] = data;
    let sighash: [u8; 8] = {
        let mut sighash: [u8; 8] = [0; 8];
        sighash.copy_from_slice(&ix_data[..8]);
        ix_data = &ix_data[8..];
        sighash
    };
    if true {
        if sighash == anchor_lang::idl::IDL_IX_TAG.to_le_bytes() {
            return __private::__idl::__idl_dispatch(program_id, accounts, &ix_data);
        }
    }
    match sighash {
        [13, 186, 25, 16, 218, 31, 90, 1] => {
            __private::__global::initialize_authority(program_id, accounts, ix_data)
        }
        [115, 230, 212, 211, 175, 49, 39, 169] => {
            __private::__global::add_pool(program_id, accounts, ix_data)
        }
        _ => Err(anchor_lang::__private::ErrorCode::InstructionFallbackNotFound.into()),
    }
}
/// Create a private module to not clutter the program's namespace.
/// Defines an entrypoint for each individual instruction handler
/// wrapper.
mod __private {
    use super::*;
    /// __idl mod defines handlers for injected Anchor IDL instructions.
    pub mod __idl {
        use super::*;
        #[inline(never)]
        #[cfg(not(feature = "no-idl"))]
        pub fn __idl_dispatch(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            idl_ix_data: &[u8],
        ) -> ProgramResult {
            let mut accounts = accounts;
            let mut data: &[u8] = idl_ix_data;
            let ix = anchor_lang::idl::IdlInstruction::deserialize(&mut data)
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            match ix {
                anchor_lang::idl::IdlInstruction::Create { data_len } => {
                    let mut accounts = anchor_lang::idl::IdlCreateAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                    )?;
                    __idl_create_account(program_id, &mut accounts, data_len)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::CreateBuffer => {
                    let mut accounts = anchor_lang::idl::IdlCreateBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                    )?;
                    __idl_create_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Write { data } => {
                    let mut accounts = anchor_lang::idl::IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                    )?;
                    __idl_write(program_id, &mut accounts, data)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetAuthority { new_authority } => {
                    let mut accounts = anchor_lang::idl::IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                    )?;
                    __idl_set_authority(program_id, &mut accounts, new_authority)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetBuffer => {
                    let mut accounts = anchor_lang::idl::IdlSetBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                    )?;
                    __idl_set_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
            }
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_create_account(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlCreateAccounts,
            data_len: u64,
        ) -> ProgramResult {
            if program_id != accounts.program.key {
                return Err(anchor_lang::__private::ErrorCode::IdlInstructionInvalidProgram.into());
            }
            let from = accounts.from.key;
            let (base, nonce) = Pubkey::find_program_address(&[], program_id);
            let seed = anchor_lang::idl::IdlAccount::seed();
            let owner = accounts.program.key;
            let to = Pubkey::create_with_seed(&base, seed, owner).unwrap();
            let space = 8 + 32 + 4 + data_len as usize;
            let rent = Rent::get()?;
            let lamports = rent.minimum_balance(space);
            let seeds = &[&[nonce][..]];
            let ix = anchor_lang::solana_program::system_instruction::create_account_with_seed(
                from,
                &to,
                &base,
                seed,
                lamports,
                space as u64,
                owner,
            );
            anchor_lang::solana_program::program::invoke_signed(
                &ix,
                &[
                    accounts.from.clone(),
                    accounts.to.clone(),
                    accounts.base.clone(),
                    accounts.system_program.clone(),
                ],
                &[seeds],
            )?;
            let mut idl_account = {
                let mut account_data = accounts.to.try_borrow_data()?;
                let mut account_data_slice: &[u8] = &account_data;
                anchor_lang::idl::IdlAccount::try_deserialize_unchecked(&mut account_data_slice)?
            };
            idl_account.authority = *accounts.from.key;
            let mut data = accounts.to.try_borrow_mut_data()?;
            let dst: &mut [u8] = &mut data;
            let mut cursor = std::io::Cursor::new(dst);
            idl_account.try_serialize(&mut cursor)?;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_create_buffer(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlCreateBuffer,
        ) -> ProgramResult {
            let mut buffer = &mut accounts.buffer;
            buffer.authority = *accounts.authority.key;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_write(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlAccounts,
            idl_data: Vec<u8>,
        ) -> ProgramResult {
            let mut idl = &mut accounts.idl;
            idl.data.extend(idl_data);
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_authority(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlAccounts,
            new_authority: Pubkey,
        ) -> ProgramResult {
            accounts.idl.authority = new_authority;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_buffer(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlSetBuffer,
        ) -> ProgramResult {
            accounts.idl.data = accounts.buffer.data.clone();
            Ok(())
        }
    }
    /// __state mod defines wrapped handlers for state instructions.
    pub mod __state {
        use super::*;
    }
    /// __interface mod defines wrapped handlers for `#[interface]` trait
    /// implementations.
    pub mod __interface {
        use super::*;
    }
    /// __global mod defines wrapped handlers for global instructions.
    pub mod __global {
        use super::*;
        #[inline(never)]
        pub fn initialize_authority(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::InitializeAuthority::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::InitializeAuthority {
                program_authority_bump,
            } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                InitializeAuthority::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            token_studies::initialize_authority(
                Context::new(program_id, &mut accounts, remaining_accounts),
                program_authority_bump,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn add_pool(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::AddPool::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::AddPool { pool_bump } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = AddPool::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            token_studies::add_pool(
                Context::new(program_id, &mut accounts, remaining_accounts),
                pool_bump,
            )?;
            accounts.exit(program_id)
        }
    }
}
pub mod token_studies {
    use super::*;
    pub fn initialize_authority(
        ctx: Context<InitializeAuthority>,
        program_authority_bump: u8,
    ) -> ProgramResult {
        let program_authority = &mut ctx.accounts.state;
        program_authority.bump = program_authority_bump;
        program_authority.authority = ctx.accounts.admin.key();
        Ok(())
    }
    pub fn add_pool(ctx: Context<AddPool>, pool_bump: u8) -> ProgramResult {
        Ok(())
    }
}
/// An Anchor generated module containing the program's set of
/// instructions, where each method handler in the `#[program]` mod is
/// associated with a struct defining the input arguments to the
/// method. These should be used directly, when one wants to serialize
/// Anchor instruction data, for example, when speciying
/// instructions on a client.
pub mod instruction {
    use super::*;
    /// Instruction struct definitions for `#[state]` methods.
    pub mod state {
        use super::*;
    }
    /// Instruction.
    pub struct InitializeAuthority {
        pub program_authority_bump: u8,
    }
    impl borsh::ser::BorshSerialize for InitializeAuthority
    where
        u8: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.program_authority_bump, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for InitializeAuthority
    where
        u8: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                program_authority_bump: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for InitializeAuthority {
        fn data(&self) -> Vec<u8> {
            let mut d = [13, 186, 25, 16, 218, 31, 90, 1].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct AddPool {
        pub pool_bump: u8,
    }
    impl borsh::ser::BorshSerialize for AddPool
    where
        u8: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.pool_bump, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for AddPool
    where
        u8: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                pool_bump: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for AddPool {
        fn data(&self) -> Vec<u8> {
            let mut d = [115, 230, 212, 211, 175, 49, 39, 169].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
}
/// An Anchor generated module, providing a set of structs
/// mirroring the structs deriving `Accounts`, where each field is
/// a `Pubkey`. This is useful for specifying accounts for a client.
pub mod accounts {
    pub use crate::__client_accounts_add_pool::*;
    pub use crate::__client_accounts_initialize_authority::*;
}
# [instruction (program_authority_bump : u8)]
pub struct InitializeAuthority<'info> {
    pub admin: Signer<'info>,
    # [account (init , seeds = [PROGRAMSEED] , bump = program_authority_bump , payer = admin)]
    pub state: Account<'info, ProgramAuthority>,
    rent: Sysvar<'info, Rent>,
    system_program: Program<'info, System>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for InitializeAuthority<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let mut ix_data = ix_data;
        struct __Args {
            program_authority_bump: u8,
        }
        impl borsh::ser::BorshSerialize for __Args
        where
            u8: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.program_authority_bump, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for __Args
        where
            u8: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    program_authority_bump: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        let __Args {
            program_authority_bump,
        } = __Args::deserialize(&mut ix_data)
            .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
        let admin: Signer = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let state = &accounts[0];
        *accounts = &accounts[1..];
        let rent: Sysvar<Rent> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let system_program: anchor_lang::Program<System> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let __anchor_rent = Rent::get()?;
        let state = {
            let space = 8 + ProgramAuthority::default().try_to_vec().unwrap().len();
            let payer = admin.to_account_info();
            let __current_lamports = state.to_account_info().lamports();
            if __current_lamports == 0 {
                let lamports = __anchor_rent.minimum_balance(space);
                anchor_lang::solana_program::program::invoke_signed(
                    &anchor_lang::solana_program::system_instruction::create_account(
                        payer.to_account_info().key,
                        state.to_account_info().key,
                        lamports,
                        space as u64,
                        program_id,
                    ),
                    &[
                        payer.to_account_info(),
                        state.to_account_info(),
                        system_program.to_account_info(),
                    ],
                    &[&[PROGRAMSEED, &[program_authority_bump]][..]],
                )?;
            } else {
                let required_lamports = __anchor_rent
                    .minimum_balance(space)
                    .max(1)
                    .saturating_sub(__current_lamports);
                if required_lamports > 0 {
                    anchor_lang::solana_program::program::invoke(
                        &anchor_lang::solana_program::system_instruction::transfer(
                            payer.to_account_info().key,
                            state.to_account_info().key,
                            required_lamports,
                        ),
                        &[
                            payer.to_account_info(),
                            state.to_account_info(),
                            system_program.to_account_info(),
                        ],
                    )?;
                }
                anchor_lang::solana_program::program::invoke_signed(
                    &anchor_lang::solana_program::system_instruction::allocate(
                        state.to_account_info().key,
                        space as u64,
                    ),
                    &[state.to_account_info(), system_program.to_account_info()],
                    &[&[PROGRAMSEED, &[program_authority_bump]][..]],
                )?;
                anchor_lang::solana_program::program::invoke_signed(
                    &anchor_lang::solana_program::system_instruction::assign(
                        state.to_account_info().key,
                        program_id,
                    ),
                    &[state.to_account_info(), system_program.to_account_info()],
                    &[&[PROGRAMSEED, &[program_authority_bump]][..]],
                )?;
            }
            let pa: anchor_lang::Account<ProgramAuthority> =
                anchor_lang::Account::try_from_unchecked(&state)?;
            pa
        };
        let (__program_signer, __bump) =
            anchor_lang::solana_program::pubkey::Pubkey::find_program_address(
                &[PROGRAMSEED],
                program_id,
            );
        if state.to_account_info().key != &__program_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
        }
        if __bump != program_authority_bump {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
        }
        if !state.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !__anchor_rent.is_exempt(
            state.to_account_info().lamports(),
            state.to_account_info().try_data_len()?,
        ) {
            return Err(anchor_lang::__private::ErrorCode::ConstraintRentExempt.into());
        }
        Ok(InitializeAuthority {
            admin,
            state,
            rent,
            system_program,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeAuthority<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.admin.to_account_infos());
        account_infos.extend(self.state.to_account_infos());
        account_infos.extend(self.rent.to_account_infos());
        account_infos.extend(self.system_program.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for InitializeAuthority<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.admin.to_account_metas(None));
        account_metas.extend(self.state.to_account_metas(None));
        account_metas.extend(self.rent.to_account_metas(None));
        account_metas.extend(self.system_program.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for InitializeAuthority<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.state, program_id)?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_initialize_authority {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct InitializeAuthority {
        pub admin: anchor_lang::solana_program::pubkey::Pubkey,
        pub state: anchor_lang::solana_program::pubkey::Pubkey,
        pub rent: anchor_lang::solana_program::pubkey::Pubkey,
        pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for InitializeAuthority
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.admin, writer)?;
            borsh::BorshSerialize::serialize(&self.state, writer)?;
            borsh::BorshSerialize::serialize(&self.rent, writer)?;
            borsh::BorshSerialize::serialize(&self.system_program, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for InitializeAuthority {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.admin, true,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.state, false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.rent, false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.system_program,
                    false,
                ),
            );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// `cpi::accounts` module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_initialize_authority {
    use super::*;
    pub struct InitializeAuthority<'info> {
        pub admin: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub state: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub system_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for InitializeAuthority<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.admin),
                    true,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.state),
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.rent),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.system_program),
                    false,
                ),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeAuthority<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.admin));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.state));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.rent));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.system_program,
            ));
            account_infos
        }
    }
}
# [instruction (pool_bump : u8)]
pub struct AddPool<'info> {
    pub admin: Signer<'info>,
    # [account (seeds = [PROGRAMSEED] , bump = state . bump , constraint = admin . key () == state . authority)]
    pub state: Account<'info, ProgramAuthority>,
    pub token_mint: Account<'info, Mint>,
    # [account (init , seeds = [POOLSEED , token_mint . key () . as_ref ()] , bump = pool_bump , payer = admin)]
    pub pool: Account<'info, Pool>,
    rent: Sysvar<'info, Rent>,
    system_program: Program<'info, System>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for AddPool<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let mut ix_data = ix_data;
        struct __Args {
            pool_bump: u8,
        }
        impl borsh::ser::BorshSerialize for __Args
        where
            u8: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.pool_bump, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for __Args
        where
            u8: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    pool_bump: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        let __Args { pool_bump } = __Args::deserialize(&mut ix_data)
            .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
        let admin: Signer = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let state: anchor_lang::Account<ProgramAuthority> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let token_mint: anchor_lang::Account<Mint> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let pool = &accounts[0];
        *accounts = &accounts[1..];
        let rent: Sysvar<Rent> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let system_program: anchor_lang::Program<System> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let __anchor_rent = Rent::get()?;
        let pool = {
            let space = 8 + Pool::default().try_to_vec().unwrap().len();
            let payer = admin.to_account_info();
            let __current_lamports = pool.to_account_info().lamports();
            if __current_lamports == 0 {
                let lamports = __anchor_rent.minimum_balance(space);
                anchor_lang::solana_program::program::invoke_signed(
                    &anchor_lang::solana_program::system_instruction::create_account(
                        payer.to_account_info().key,
                        pool.to_account_info().key,
                        lamports,
                        space as u64,
                        program_id,
                    ),
                    &[
                        payer.to_account_info(),
                        pool.to_account_info(),
                        system_program.to_account_info(),
                    ],
                    &[&[POOLSEED, token_mint.key().as_ref(), &[pool_bump]][..]],
                )?;
            } else {
                let required_lamports = __anchor_rent
                    .minimum_balance(space)
                    .max(1)
                    .saturating_sub(__current_lamports);
                if required_lamports > 0 {
                    anchor_lang::solana_program::program::invoke(
                        &anchor_lang::solana_program::system_instruction::transfer(
                            payer.to_account_info().key,
                            pool.to_account_info().key,
                            required_lamports,
                        ),
                        &[
                            payer.to_account_info(),
                            pool.to_account_info(),
                            system_program.to_account_info(),
                        ],
                    )?;
                }
                anchor_lang::solana_program::program::invoke_signed(
                    &anchor_lang::solana_program::system_instruction::allocate(
                        pool.to_account_info().key,
                        space as u64,
                    ),
                    &[pool.to_account_info(), system_program.to_account_info()],
                    &[&[POOLSEED, token_mint.key().as_ref(), &[pool_bump]][..]],
                )?;
                anchor_lang::solana_program::program::invoke_signed(
                    &anchor_lang::solana_program::system_instruction::assign(
                        pool.to_account_info().key,
                        program_id,
                    ),
                    &[pool.to_account_info(), system_program.to_account_info()],
                    &[&[POOLSEED, token_mint.key().as_ref(), &[pool_bump]][..]],
                )?;
            }
            let pa: anchor_lang::Account<Pool> = anchor_lang::Account::try_from_unchecked(&pool)?;
            pa
        };
        let (__program_signer, __bump) =
            anchor_lang::solana_program::pubkey::Pubkey::find_program_address(
                &[POOLSEED, token_mint.key().as_ref()],
                program_id,
            );
        if pool.to_account_info().key != &__program_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
        }
        if __bump != pool_bump {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
        }
        if !pool.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !__anchor_rent.is_exempt(
            pool.to_account_info().lamports(),
            pool.to_account_info().try_data_len()?,
        ) {
            return Err(anchor_lang::__private::ErrorCode::ConstraintRentExempt.into());
        }
        let __program_signer =
            Pubkey::create_program_address(&[PROGRAMSEED, &[state.bump]][..], program_id)
                .map_err(|_| anchor_lang::__private::ErrorCode::ConstraintSeeds)?;
        if state.to_account_info().key != &__program_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
        }
        if !(admin.key() == state.authority) {
            return Err(anchor_lang::__private::ErrorCode::ConstraintRaw.into());
        }
        Ok(AddPool {
            admin,
            state,
            token_mint,
            pool,
            rent,
            system_program,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for AddPool<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.admin.to_account_infos());
        account_infos.extend(self.state.to_account_infos());
        account_infos.extend(self.token_mint.to_account_infos());
        account_infos.extend(self.pool.to_account_infos());
        account_infos.extend(self.rent.to_account_infos());
        account_infos.extend(self.system_program.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for AddPool<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.admin.to_account_metas(None));
        account_metas.extend(self.state.to_account_metas(None));
        account_metas.extend(self.token_mint.to_account_metas(None));
        account_metas.extend(self.pool.to_account_metas(None));
        account_metas.extend(self.rent.to_account_metas(None));
        account_metas.extend(self.system_program.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for AddPool<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.pool, program_id)?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_add_pool {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct AddPool {
        pub admin: anchor_lang::solana_program::pubkey::Pubkey,
        pub state: anchor_lang::solana_program::pubkey::Pubkey,
        pub token_mint: anchor_lang::solana_program::pubkey::Pubkey,
        pub pool: anchor_lang::solana_program::pubkey::Pubkey,
        pub rent: anchor_lang::solana_program::pubkey::Pubkey,
        pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for AddPool
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.admin, writer)?;
            borsh::BorshSerialize::serialize(&self.state, writer)?;
            borsh::BorshSerialize::serialize(&self.token_mint, writer)?;
            borsh::BorshSerialize::serialize(&self.pool, writer)?;
            borsh::BorshSerialize::serialize(&self.rent, writer)?;
            borsh::BorshSerialize::serialize(&self.system_program, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for AddPool {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.admin, true,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.state, false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.token_mint,
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.pool, false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.rent, false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.system_program,
                    false,
                ),
            );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// `cpi::accounts` module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_add_pool {
    use super::*;
    pub struct AddPool<'info> {
        pub admin: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub state: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub token_mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub pool: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub system_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for AddPool<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.admin),
                    true,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.state),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.token_mint),
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.pool),
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.rent),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.system_program),
                    false,
                ),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for AddPool<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.admin));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.state));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.token_mint,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.pool));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.rent));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.system_program,
            ));
            account_infos
        }
    }
}
pub struct ProgramAuthority {
    pub bump: u8,
    pub authority: Pubkey,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::default::Default for ProgramAuthority {
    #[inline]
    fn default() -> ProgramAuthority {
        ProgramAuthority {
            bump: ::core::default::Default::default(),
            authority: ::core::default::Default::default(),
        }
    }
}
impl borsh::ser::BorshSerialize for ProgramAuthority
where
    u8: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.bump, writer)?;
        borsh::BorshSerialize::serialize(&self.authority, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for ProgramAuthority
where
    u8: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
{
    fn deserialize(buf: &mut &[u8]) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            bump: borsh::BorshDeserialize::deserialize(buf)?,
            authority: borsh::BorshDeserialize::deserialize(buf)?,
        })
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for ProgramAuthority {
    #[inline]
    fn clone(&self) -> ProgramAuthority {
        match *self {
            ProgramAuthority {
                bump: ref __self_0_0,
                authority: ref __self_0_1,
            } => ProgramAuthority {
                bump: ::core::clone::Clone::clone(&(*__self_0_0)),
                authority: ::core::clone::Clone::clone(&(*__self_0_1)),
            },
        }
    }
}
#[automatically_derived]
impl anchor_lang::AccountSerialize for ProgramAuthority {
    fn try_serialize<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> std::result::Result<(), ProgramError> {
        writer
            .write_all(&[38, 198, 188, 60, 171, 210, 169, 38])
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotSerialize)?;
        AnchorSerialize::serialize(self, writer)
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotSerialize)?;
        Ok(())
    }
}
#[automatically_derived]
impl anchor_lang::AccountDeserialize for ProgramAuthority {
    fn try_deserialize(buf: &mut &[u8]) -> std::result::Result<Self, ProgramError> {
        if buf.len() < [38, 198, 188, 60, 171, 210, 169, 38].len() {
            return Err(anchor_lang::__private::ErrorCode::AccountDiscriminatorNotFound.into());
        }
        let given_disc = &buf[..8];
        if &[38, 198, 188, 60, 171, 210, 169, 38] != given_disc {
            return Err(anchor_lang::__private::ErrorCode::AccountDiscriminatorMismatch.into());
        }
        Self::try_deserialize_unchecked(buf)
    }
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> std::result::Result<Self, ProgramError> {
        let mut data: &[u8] = &buf[8..];
        AnchorDeserialize::deserialize(&mut data)
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotDeserialize.into())
    }
}
#[automatically_derived]
impl anchor_lang::Discriminator for ProgramAuthority {
    fn discriminator() -> [u8; 8] {
        [38, 198, 188, 60, 171, 210, 169, 38]
    }
}
#[automatically_derived]
impl anchor_lang::Owner for ProgramAuthority {
    fn owner() -> Pubkey {
        crate::ID
    }
}
pub struct Pool {
    pub bump: u8,
    pub token_account_address: Pubkey,
    pub voucher_account_address: Pubkey,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::default::Default for Pool {
    #[inline]
    fn default() -> Pool {
        Pool {
            bump: ::core::default::Default::default(),
            token_account_address: ::core::default::Default::default(),
            voucher_account_address: ::core::default::Default::default(),
        }
    }
}
impl borsh::ser::BorshSerialize for Pool
where
    u8: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.bump, writer)?;
        borsh::BorshSerialize::serialize(&self.token_account_address, writer)?;
        borsh::BorshSerialize::serialize(&self.voucher_account_address, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for Pool
where
    u8: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
{
    fn deserialize(buf: &mut &[u8]) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            bump: borsh::BorshDeserialize::deserialize(buf)?,
            token_account_address: borsh::BorshDeserialize::deserialize(buf)?,
            voucher_account_address: borsh::BorshDeserialize::deserialize(buf)?,
        })
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Pool {
    #[inline]
    fn clone(&self) -> Pool {
        match *self {
            Pool {
                bump: ref __self_0_0,
                token_account_address: ref __self_0_1,
                voucher_account_address: ref __self_0_2,
            } => Pool {
                bump: ::core::clone::Clone::clone(&(*__self_0_0)),
                token_account_address: ::core::clone::Clone::clone(&(*__self_0_1)),
                voucher_account_address: ::core::clone::Clone::clone(&(*__self_0_2)),
            },
        }
    }
}
#[automatically_derived]
impl anchor_lang::AccountSerialize for Pool {
    fn try_serialize<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> std::result::Result<(), ProgramError> {
        writer
            .write_all(&[241, 154, 109, 4, 17, 177, 109, 188])
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotSerialize)?;
        AnchorSerialize::serialize(self, writer)
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotSerialize)?;
        Ok(())
    }
}
#[automatically_derived]
impl anchor_lang::AccountDeserialize for Pool {
    fn try_deserialize(buf: &mut &[u8]) -> std::result::Result<Self, ProgramError> {
        if buf.len() < [241, 154, 109, 4, 17, 177, 109, 188].len() {
            return Err(anchor_lang::__private::ErrorCode::AccountDiscriminatorNotFound.into());
        }
        let given_disc = &buf[..8];
        if &[241, 154, 109, 4, 17, 177, 109, 188] != given_disc {
            return Err(anchor_lang::__private::ErrorCode::AccountDiscriminatorMismatch.into());
        }
        Self::try_deserialize_unchecked(buf)
    }
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> std::result::Result<Self, ProgramError> {
        let mut data: &[u8] = &buf[8..];
        AnchorDeserialize::deserialize(&mut data)
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotDeserialize.into())
    }
}
#[automatically_derived]
impl anchor_lang::Discriminator for Pool {
    fn discriminator() -> [u8; 8] {
        [241, 154, 109, 4, 17, 177, 109, 188]
    }
}
#[automatically_derived]
impl anchor_lang::Owner for Pool {
    fn owner() -> Pubkey {
        crate::ID
    }
}
