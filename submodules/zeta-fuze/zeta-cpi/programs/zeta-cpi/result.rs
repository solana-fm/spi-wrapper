#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use anchor_lang::prelude::*;
use rust_decimal::prelude::*;
pub mod context {
    use crate::zeta_context::*;
    use crate::*;
    pub struct InitializeMarginAccountCaller<'info> {
        pub zeta_program: AccountInfo<'info>,
        pub initialize_margin_cpi_accounts: InitializeMarginAccount<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info> for InitializeMarginAccountCaller<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
            ix_data: &[u8],
        ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError>
        {
            let zeta_program: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let initialize_margin_cpi_accounts: InitializeMarginAccount<'info> =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            Ok(InitializeMarginAccountCaller {
                zeta_program,
                initialize_margin_cpi_accounts,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeMarginAccountCaller<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.zeta_program.to_account_infos());
            account_infos.extend(self.initialize_margin_cpi_accounts.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for InitializeMarginAccountCaller<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.zeta_program.to_account_metas(None));
            account_metas.extend(self.initialize_margin_cpi_accounts.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for InitializeMarginAccountCaller<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
            anchor_lang::AccountsExit::exit(&self.initialize_margin_cpi_accounts, program_id)?;
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
    pub(crate) mod __client_accounts_initialize_margin_account_caller {
        use super::*;
        use anchor_lang::prelude::borsh;
        pub use __client_accounts_initialize_margin_account::InitializeMarginAccount;
        pub struct InitializeMarginAccountCaller {
            pub zeta_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub initialize_margin_cpi_accounts:
                __client_accounts_initialize_margin_account::InitializeMarginAccount,
        }
        impl borsh::ser::BorshSerialize for InitializeMarginAccountCaller
        where
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            __client_accounts_initialize_margin_account::InitializeMarginAccount:
                borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.zeta_program, writer)?;
                borsh::BorshSerialize::serialize(&self.initialize_margin_cpi_accounts, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for InitializeMarginAccountCaller {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.zeta_program,
                        false,
                    ),
                );
                account_metas.extend(self.initialize_margin_cpi_accounts.to_account_metas(None));
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
    pub(crate) mod __cpi_client_accounts_initialize_margin_account_caller {
        use super::*;
        pub use __cpi_client_accounts_initialize_margin_account::InitializeMarginAccount;
        pub struct InitializeMarginAccountCaller<'info> {
            pub zeta_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub initialize_margin_cpi_accounts:
                __cpi_client_accounts_initialize_margin_account::InitializeMarginAccount<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for InitializeMarginAccountCaller<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.zeta_program),
                        false,
                    ),
                );
                account_metas.extend(self.initialize_margin_cpi_accounts.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeMarginAccountCaller<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.zeta_program,
                ));
                account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(
                    &self.initialize_margin_cpi_accounts,
                ));
                account_infos
            }
        }
    }
    pub struct DepositCaller<'info> {
        pub zeta_program: AccountInfo<'info>,
        pub deposit_cpi_accounts: Deposit<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info> for DepositCaller<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
            ix_data: &[u8],
        ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError>
        {
            let zeta_program: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let deposit_cpi_accounts: Deposit<'info> =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            Ok(DepositCaller {
                zeta_program,
                deposit_cpi_accounts,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for DepositCaller<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.zeta_program.to_account_infos());
            account_infos.extend(self.deposit_cpi_accounts.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for DepositCaller<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.zeta_program.to_account_metas(None));
            account_metas.extend(self.deposit_cpi_accounts.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for DepositCaller<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
            anchor_lang::AccountsExit::exit(&self.deposit_cpi_accounts, program_id)?;
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
    pub(crate) mod __client_accounts_deposit_caller {
        use super::*;
        use anchor_lang::prelude::borsh;
        pub use __client_accounts_deposit::Deposit;
        pub struct DepositCaller {
            pub zeta_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub deposit_cpi_accounts: __client_accounts_deposit::Deposit,
        }
        impl borsh::ser::BorshSerialize for DepositCaller
        where
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            __client_accounts_deposit::Deposit: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.zeta_program, writer)?;
                borsh::BorshSerialize::serialize(&self.deposit_cpi_accounts, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for DepositCaller {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.zeta_program,
                        false,
                    ),
                );
                account_metas.extend(self.deposit_cpi_accounts.to_account_metas(None));
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
    pub(crate) mod __cpi_client_accounts_deposit_caller {
        use super::*;
        pub use __cpi_client_accounts_deposit::Deposit;
        pub struct DepositCaller<'info> {
            pub zeta_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub deposit_cpi_accounts: __cpi_client_accounts_deposit::Deposit<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for DepositCaller<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.zeta_program),
                        false,
                    ),
                );
                account_metas.extend(self.deposit_cpi_accounts.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for DepositCaller<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.zeta_program,
                ));
                account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(
                    &self.deposit_cpi_accounts,
                ));
                account_infos
            }
        }
    }
    pub struct WithdrawCaller<'info> {
        pub zeta_program: AccountInfo<'info>,
        pub withdraw_cpi_accounts: Withdraw<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info> for WithdrawCaller<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
            ix_data: &[u8],
        ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError>
        {
            let zeta_program: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let withdraw_cpi_accounts: Withdraw<'info> =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            Ok(WithdrawCaller {
                zeta_program,
                withdraw_cpi_accounts,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for WithdrawCaller<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.zeta_program.to_account_infos());
            account_infos.extend(self.withdraw_cpi_accounts.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for WithdrawCaller<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.zeta_program.to_account_metas(None));
            account_metas.extend(self.withdraw_cpi_accounts.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for WithdrawCaller<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
            anchor_lang::AccountsExit::exit(&self.withdraw_cpi_accounts, program_id)?;
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
    pub(crate) mod __client_accounts_withdraw_caller {
        use super::*;
        use anchor_lang::prelude::borsh;
        pub use __client_accounts_withdraw::Withdraw;
        pub struct WithdrawCaller {
            pub zeta_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub withdraw_cpi_accounts: __client_accounts_withdraw::Withdraw,
        }
        impl borsh::ser::BorshSerialize for WithdrawCaller
        where
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            __client_accounts_withdraw::Withdraw: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.zeta_program, writer)?;
                borsh::BorshSerialize::serialize(&self.withdraw_cpi_accounts, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for WithdrawCaller {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.zeta_program,
                        false,
                    ),
                );
                account_metas.extend(self.withdraw_cpi_accounts.to_account_metas(None));
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
    pub(crate) mod __cpi_client_accounts_withdraw_caller {
        use super::*;
        pub use __cpi_client_accounts_withdraw::Withdraw;
        pub struct WithdrawCaller<'info> {
            pub zeta_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub withdraw_cpi_accounts: __cpi_client_accounts_withdraw::Withdraw<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for WithdrawCaller<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.zeta_program),
                        false,
                    ),
                );
                account_metas.extend(self.withdraw_cpi_accounts.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for WithdrawCaller<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.zeta_program,
                ));
                account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(
                    &self.withdraw_cpi_accounts,
                ));
                account_infos
            }
        }
    }
    pub struct InitializeOpenOrdersCaller<'info> {
        pub zeta_program: AccountInfo<'info>,
        pub initialize_open_orders_cpi_accounts: InitializeOpenOrders<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info> for InitializeOpenOrdersCaller<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
            ix_data: &[u8],
        ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError>
        {
            let zeta_program: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let initialize_open_orders_cpi_accounts: InitializeOpenOrders<'info> =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            Ok(InitializeOpenOrdersCaller {
                zeta_program,
                initialize_open_orders_cpi_accounts,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeOpenOrdersCaller<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.zeta_program.to_account_infos());
            account_infos.extend(self.initialize_open_orders_cpi_accounts.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for InitializeOpenOrdersCaller<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.zeta_program.to_account_metas(None));
            account_metas.extend(
                self.initialize_open_orders_cpi_accounts
                    .to_account_metas(None),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for InitializeOpenOrdersCaller<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
            anchor_lang::AccountsExit::exit(&self.initialize_open_orders_cpi_accounts, program_id)?;
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
    pub(crate) mod __client_accounts_initialize_open_orders_caller {
        use super::*;
        use anchor_lang::prelude::borsh;
        pub use __client_accounts_initialize_open_orders::InitializeOpenOrders;
        pub struct InitializeOpenOrdersCaller {
            pub zeta_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub initialize_open_orders_cpi_accounts:
                __client_accounts_initialize_open_orders::InitializeOpenOrders,
        }
        impl borsh::ser::BorshSerialize for InitializeOpenOrdersCaller
        where
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            __client_accounts_initialize_open_orders::InitializeOpenOrders:
                borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.zeta_program, writer)?;
                borsh::BorshSerialize::serialize(
                    &self.initialize_open_orders_cpi_accounts,
                    writer,
                )?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for InitializeOpenOrdersCaller {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.zeta_program,
                        false,
                    ),
                );
                account_metas.extend(
                    self.initialize_open_orders_cpi_accounts
                        .to_account_metas(None),
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
    pub(crate) mod __cpi_client_accounts_initialize_open_orders_caller {
        use super::*;
        pub use __cpi_client_accounts_initialize_open_orders::InitializeOpenOrders;
        pub struct InitializeOpenOrdersCaller<'info> {
            pub zeta_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub initialize_open_orders_cpi_accounts:
                __cpi_client_accounts_initialize_open_orders::InitializeOpenOrders<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for InitializeOpenOrdersCaller<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.zeta_program),
                        false,
                    ),
                );
                account_metas.extend(
                    self.initialize_open_orders_cpi_accounts
                        .to_account_metas(None),
                );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeOpenOrdersCaller<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.zeta_program,
                ));
                account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(
                    &self.initialize_open_orders_cpi_accounts,
                ));
                account_infos
            }
        }
    }
    pub struct PlaceOrderCaller<'info> {
        pub zeta_program: AccountInfo<'info>,
        pub place_order_cpi_accounts: PlaceOrder<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info> for PlaceOrderCaller<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
            ix_data: &[u8],
        ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError>
        {
            let zeta_program: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let place_order_cpi_accounts: PlaceOrder<'info> =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            Ok(PlaceOrderCaller {
                zeta_program,
                place_order_cpi_accounts,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for PlaceOrderCaller<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.zeta_program.to_account_infos());
            account_infos.extend(self.place_order_cpi_accounts.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for PlaceOrderCaller<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.zeta_program.to_account_metas(None));
            account_metas.extend(self.place_order_cpi_accounts.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for PlaceOrderCaller<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
            anchor_lang::AccountsExit::exit(&self.place_order_cpi_accounts, program_id)?;
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
    pub(crate) mod __client_accounts_place_order_caller {
        use super::*;
        use anchor_lang::prelude::borsh;
        pub use __client_accounts_place_order::PlaceOrder;
        pub struct PlaceOrderCaller {
            pub zeta_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub place_order_cpi_accounts: __client_accounts_place_order::PlaceOrder,
        }
        impl borsh::ser::BorshSerialize for PlaceOrderCaller
        where
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            __client_accounts_place_order::PlaceOrder: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.zeta_program, writer)?;
                borsh::BorshSerialize::serialize(&self.place_order_cpi_accounts, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for PlaceOrderCaller {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.zeta_program,
                        false,
                    ),
                );
                account_metas.extend(self.place_order_cpi_accounts.to_account_metas(None));
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
    pub(crate) mod __cpi_client_accounts_place_order_caller {
        use super::*;
        pub use __cpi_client_accounts_place_order::PlaceOrder;
        pub struct PlaceOrderCaller<'info> {
            pub zeta_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub place_order_cpi_accounts: __cpi_client_accounts_place_order::PlaceOrder<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for PlaceOrderCaller<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.zeta_program),
                        false,
                    ),
                );
                account_metas.extend(self.place_order_cpi_accounts.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for PlaceOrderCaller<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.zeta_program,
                ));
                account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(
                    &self.place_order_cpi_accounts,
                ));
                account_infos
            }
        }
    }
    pub struct CancelOrderCaller<'info> {
        pub zeta_program: AccountInfo<'info>,
        pub cancel_order_cpi_accounts: CancelOrder<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info> for CancelOrderCaller<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
            ix_data: &[u8],
        ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError>
        {
            let zeta_program: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let cancel_order_cpi_accounts: CancelOrder<'info> =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            Ok(CancelOrderCaller {
                zeta_program,
                cancel_order_cpi_accounts,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for CancelOrderCaller<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.zeta_program.to_account_infos());
            account_infos.extend(self.cancel_order_cpi_accounts.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for CancelOrderCaller<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.zeta_program.to_account_metas(None));
            account_metas.extend(self.cancel_order_cpi_accounts.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for CancelOrderCaller<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
            anchor_lang::AccountsExit::exit(&self.cancel_order_cpi_accounts, program_id)?;
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
    pub(crate) mod __client_accounts_cancel_order_caller {
        use super::*;
        use anchor_lang::prelude::borsh;
        pub use __client_accounts_cancel_order::CancelOrder;
        pub struct CancelOrderCaller {
            pub zeta_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub cancel_order_cpi_accounts: __client_accounts_cancel_order::CancelOrder,
        }
        impl borsh::ser::BorshSerialize for CancelOrderCaller
        where
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            __client_accounts_cancel_order::CancelOrder: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.zeta_program, writer)?;
                borsh::BorshSerialize::serialize(&self.cancel_order_cpi_accounts, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for CancelOrderCaller {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.zeta_program,
                        false,
                    ),
                );
                account_metas.extend(self.cancel_order_cpi_accounts.to_account_metas(None));
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
    pub(crate) mod __cpi_client_accounts_cancel_order_caller {
        use super::*;
        pub use __cpi_client_accounts_cancel_order::CancelOrder;
        pub struct CancelOrderCaller<'info> {
            pub zeta_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub cancel_order_cpi_accounts: __cpi_client_accounts_cancel_order::CancelOrder<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for CancelOrderCaller<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.zeta_program),
                        false,
                    ),
                );
                account_metas.extend(self.cancel_order_cpi_accounts.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for CancelOrderCaller<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.zeta_program,
                ));
                account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(
                    &self.cancel_order_cpi_accounts,
                ));
                account_infos
            }
        }
    }
    pub struct ReadProgramData<'info> {
        pub state: AccountInfo<'info>,
        pub zeta_group: AccountInfo<'info>,
        pub margin_account: AccountInfo<'info>,
        pub greeks: AccountInfo<'info>,
        pub oracle: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info> for ReadProgramData<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
            ix_data: &[u8],
        ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError>
        {
            let state: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let zeta_group: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let margin_account: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let greeks: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let oracle: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            Ok(ReadProgramData {
                state,
                zeta_group,
                margin_account,
                greeks,
                oracle,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for ReadProgramData<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.state.to_account_infos());
            account_infos.extend(self.zeta_group.to_account_infos());
            account_infos.extend(self.margin_account.to_account_infos());
            account_infos.extend(self.greeks.to_account_infos());
            account_infos.extend(self.oracle.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for ReadProgramData<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.state.to_account_metas(None));
            account_metas.extend(self.zeta_group.to_account_metas(None));
            account_metas.extend(self.margin_account.to_account_metas(None));
            account_metas.extend(self.greeks.to_account_metas(None));
            account_metas.extend(self.oracle.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for ReadProgramData<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
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
    pub(crate) mod __client_accounts_read_program_data {
        use super::*;
        use anchor_lang::prelude::borsh;
        pub struct ReadProgramData {
            pub state: anchor_lang::solana_program::pubkey::Pubkey,
            pub zeta_group: anchor_lang::solana_program::pubkey::Pubkey,
            pub margin_account: anchor_lang::solana_program::pubkey::Pubkey,
            pub greeks: anchor_lang::solana_program::pubkey::Pubkey,
            pub oracle: anchor_lang::solana_program::pubkey::Pubkey,
        }
        impl borsh::ser::BorshSerialize for ReadProgramData
        where
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
                borsh::BorshSerialize::serialize(&self.state, writer)?;
                borsh::BorshSerialize::serialize(&self.zeta_group, writer)?;
                borsh::BorshSerialize::serialize(&self.margin_account, writer)?;
                borsh::BorshSerialize::serialize(&self.greeks, writer)?;
                borsh::BorshSerialize::serialize(&self.oracle, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for ReadProgramData {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.state, false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.zeta_group,
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.margin_account,
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.greeks,
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.oracle,
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
    pub(crate) mod __cpi_client_accounts_read_program_data {
        use super::*;
        pub struct ReadProgramData<'info> {
            pub state: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub zeta_group: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub margin_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub greeks: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub oracle: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for ReadProgramData<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.state),
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.zeta_group),
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.margin_account),
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.greeks),
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.oracle),
                        false,
                    ),
                );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for ReadProgramData<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.state));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.zeta_group,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.margin_account,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.greeks));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.oracle));
                account_infos
            }
        }
    }
}
pub mod pyth_client {
    use crate::*;
    use anchor_lang::prelude::AccountInfo;
    use bytemuck::{cast_slice_mut, from_bytes_mut, try_cast_slice_mut, Pod, Zeroable};
    use std::cell::RefMut;
    #[repr(C)]
    pub struct AccKey {
        pub val: [u8; 32],
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for AccKey {
        #[inline]
        fn default() -> AccKey {
            AccKey {
                val: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for AccKey {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for AccKey {
        #[inline]
        fn clone(&self) -> AccKey {
            {
                let _: ::core::clone::AssertParamIsClone<[u8; 32]>;
                *self
            }
        }
    }
    #[repr(C)]
    pub enum PriceStatus {
        Unknown,
        Trading,
        Halted,
        Auction,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for PriceStatus {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for PriceStatus {
        #[inline]
        fn clone(&self) -> PriceStatus {
            {
                *self
            }
        }
    }
    impl Default for PriceStatus {
        fn default() -> Self {
            PriceStatus::Trading
        }
    }
    #[repr(C)]
    pub enum CorpAction {
        NoCorpAct,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for CorpAction {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for CorpAction {
        #[inline]
        fn clone(&self) -> CorpAction {
            {
                *self
            }
        }
    }
    impl Default for CorpAction {
        fn default() -> Self {
            CorpAction::NoCorpAct
        }
    }
    #[repr(C)]
    pub struct PriceInfo {
        pub price: i64,
        pub conf: u64,
        pub status: PriceStatus,
        pub corp_act: CorpAction,
        pub pub_slot: u64,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for PriceInfo {
        #[inline]
        fn default() -> PriceInfo {
            PriceInfo {
                price: ::core::default::Default::default(),
                conf: ::core::default::Default::default(),
                status: ::core::default::Default::default(),
                corp_act: ::core::default::Default::default(),
                pub_slot: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for PriceInfo {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for PriceInfo {
        #[inline]
        fn clone(&self) -> PriceInfo {
            {
                let _: ::core::clone::AssertParamIsClone<i64>;
                let _: ::core::clone::AssertParamIsClone<u64>;
                let _: ::core::clone::AssertParamIsClone<PriceStatus>;
                let _: ::core::clone::AssertParamIsClone<CorpAction>;
                let _: ::core::clone::AssertParamIsClone<u64>;
                *self
            }
        }
    }
    #[repr(C)]
    pub struct PriceComp {
        publisher: AccKey,
        agg: PriceInfo,
        latest: PriceInfo,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for PriceComp {
        #[inline]
        fn default() -> PriceComp {
            PriceComp {
                publisher: ::core::default::Default::default(),
                agg: ::core::default::Default::default(),
                latest: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for PriceComp {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for PriceComp {
        #[inline]
        fn clone(&self) -> PriceComp {
            {
                let _: ::core::clone::AssertParamIsClone<AccKey>;
                let _: ::core::clone::AssertParamIsClone<PriceInfo>;
                let _: ::core::clone::AssertParamIsClone<PriceInfo>;
                *self
            }
        }
    }
    #[repr(C)]
    pub enum PriceType {
        Unknown,
        Price,
        TWAP,
        Volatility,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for PriceType {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for PriceType {
        #[inline]
        fn clone(&self) -> PriceType {
            {
                *self
            }
        }
    }
    impl Default for PriceType {
        fn default() -> Self {
            PriceType::Price
        }
    }
    #[repr(C)]
    pub struct Price {
        pub magic: u32,
        pub ver: u32,
        pub atype: u32,
        pub size: u32,
        pub ptype: PriceType,
        pub expo: i32,
        pub num: u32,
        pub unused: u32,
        pub curr_slot: u64,
        pub valid_slot: u64,
        pub twap: i64,
        pub avol: u64,
        pub drv0: i64,
        pub drv1: i64,
        pub drv2: i64,
        pub drv3: i64,
        pub drv4: i64,
        pub drv5: i64,
        pub prod: AccKey,
        pub next: AccKey,
        pub agg_pub: AccKey,
        pub agg: PriceInfo,
        pub comp: [PriceComp; 32],
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for Price {
        #[inline]
        fn default() -> Price {
            Price {
                magic: ::core::default::Default::default(),
                ver: ::core::default::Default::default(),
                atype: ::core::default::Default::default(),
                size: ::core::default::Default::default(),
                ptype: ::core::default::Default::default(),
                expo: ::core::default::Default::default(),
                num: ::core::default::Default::default(),
                unused: ::core::default::Default::default(),
                curr_slot: ::core::default::Default::default(),
                valid_slot: ::core::default::Default::default(),
                twap: ::core::default::Default::default(),
                avol: ::core::default::Default::default(),
                drv0: ::core::default::Default::default(),
                drv1: ::core::default::Default::default(),
                drv2: ::core::default::Default::default(),
                drv3: ::core::default::Default::default(),
                drv4: ::core::default::Default::default(),
                drv5: ::core::default::Default::default(),
                prod: ::core::default::Default::default(),
                next: ::core::default::Default::default(),
                agg_pub: ::core::default::Default::default(),
                agg: ::core::default::Default::default(),
                comp: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Price {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Price {
        #[inline]
        fn clone(&self) -> Price {
            {
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<PriceType>;
                let _: ::core::clone::AssertParamIsClone<i32>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<u64>;
                let _: ::core::clone::AssertParamIsClone<u64>;
                let _: ::core::clone::AssertParamIsClone<i64>;
                let _: ::core::clone::AssertParamIsClone<u64>;
                let _: ::core::clone::AssertParamIsClone<i64>;
                let _: ::core::clone::AssertParamIsClone<i64>;
                let _: ::core::clone::AssertParamIsClone<i64>;
                let _: ::core::clone::AssertParamIsClone<i64>;
                let _: ::core::clone::AssertParamIsClone<i64>;
                let _: ::core::clone::AssertParamIsClone<i64>;
                let _: ::core::clone::AssertParamIsClone<AccKey>;
                let _: ::core::clone::AssertParamIsClone<AccKey>;
                let _: ::core::clone::AssertParamIsClone<AccKey>;
                let _: ::core::clone::AssertParamIsClone<PriceInfo>;
                let _: ::core::clone::AssertParamIsClone<[PriceComp; 32]>;
                *self
            }
        }
    }
    impl Price {
        #[inline]
        pub fn load<'a>(price_feed: &'a AccountInfo) -> Result<RefMut<'a, Price>> {
            let account_data: RefMut<'a, [u8]>;
            let state: RefMut<'a, Self>;
            account_data = RefMut::map(price_feed.try_borrow_mut_data().unwrap(), |data| *data);
            state = RefMut::map(account_data, |data| {
                from_bytes_mut(cast_slice_mut::<u8, u8>(try_cast_slice_mut(data).unwrap()))
            });
            Ok(state)
        }
    }
    #[cfg(target_endian = "little")]
    unsafe impl Zeroable for Price {}
    #[cfg(target_endian = "little")]
    unsafe impl Pod for Price {}
}
pub mod zeta_account {
    use crate::*;
    use bytemuck::{Pod, Zeroable};
    use std::convert::{From, TryFrom, TryInto};
    #[repr(packed)]
    pub struct ProductGreeks {
        pub delta: u64,
        pub vega: AnchorDecimal,
        pub volatility: AnchorDecimal,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for ProductGreeks {
        #[inline]
        fn default() -> ProductGreeks {
            ProductGreeks {
                delta: ::core::default::Default::default(),
                vega: ::core::default::Default::default(),
                volatility: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ProductGreeks {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for ProductGreeks {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for ProductGreeks {
        #[inline]
        fn clone(&self) -> ProductGreeks {
            {
                let _: ::core::clone::AssertParamIsClone<u64>;
                let _: ::core::clone::AssertParamIsClone<AnchorDecimal>;
                let _: ::core::clone::AssertParamIsClone<AnchorDecimal>;
                *self
            }
        }
    }
    #[repr(packed)]
    pub struct AnchorDecimal {
        pub flags: u32,
        pub hi: u32,
        pub lo: u32,
        pub mid: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for AnchorDecimal {
        #[inline]
        fn default() -> AnchorDecimal {
            AnchorDecimal {
                flags: ::core::default::Default::default(),
                hi: ::core::default::Default::default(),
                lo: ::core::default::Default::default(),
                mid: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl AnchorDecimal {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for AnchorDecimal {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for AnchorDecimal {
        #[inline]
        fn clone(&self) -> AnchorDecimal {
            {
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                *self
            }
        }
    }
    impl From<Decimal> for AnchorDecimal {
        fn from(decimal: Decimal) -> AnchorDecimal {
            AnchorDecimal {
                flags: decimal.flags,
                hi: decimal.hi,
                lo: decimal.lo,
                mid: decimal.mid,
            }
        }
    }
    impl From<AnchorDecimal> for Decimal {
        fn from(decimal: AnchorDecimal) -> Decimal {
            Decimal {
                flags: decimal.flags,
                hi: decimal.hi,
                lo: decimal.lo,
                mid: decimal.mid,
            }
        }
    }
    #[cfg(target_endian = "little")]
    unsafe impl Zeroable for AnchorDecimal {}
    #[cfg(target_endian = "little")]
    unsafe impl Pod for AnchorDecimal {}
    #[repr(packed)]
    pub struct Greeks {
        pub nonce: u8,
        pub mark_prices: [u64; 46],
        pub _mark_prices_padding: [u64; 92],
        pub product_greeks: [ProductGreeks; 22],
        pub _product_greeks_padding: [ProductGreeks; 44],
        pub update_timestamp: [u64; 2],
        pub _update_timestamp_padding: [u64; 4],
        pub retreat_expiration_timestamp: [u64; 2],
        pub _retreat_expiration_timestamp_padding: [u64; 4],
        pub interest_rate: [i64; 2],
        pub _interest_rate_padding: [i64; 4],
        pub nodes: [u64; 5],
        pub volatility: [u64; 10],
        pub _volatility_padding: [u64; 20],
        pub node_keys: [Pubkey; 138],
        pub _padding: [u8; 1647],
    }
    #[automatically_derived]
    impl Greeks {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Greeks {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Greeks {
        #[inline]
        fn clone(&self) -> Greeks {
            {
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<[u64; 46]>;
                let _: ::core::clone::AssertParamIsClone<[u64; 92]>;
                let _: ::core::clone::AssertParamIsClone<[ProductGreeks; 22]>;
                let _: ::core::clone::AssertParamIsClone<[ProductGreeks; 44]>;
                let _: ::core::clone::AssertParamIsClone<[u64; 2]>;
                let _: ::core::clone::AssertParamIsClone<[u64; 4]>;
                let _: ::core::clone::AssertParamIsClone<[u64; 2]>;
                let _: ::core::clone::AssertParamIsClone<[u64; 4]>;
                let _: ::core::clone::AssertParamIsClone<[i64; 2]>;
                let _: ::core::clone::AssertParamIsClone<[i64; 4]>;
                let _: ::core::clone::AssertParamIsClone<[u64; 5]>;
                let _: ::core::clone::AssertParamIsClone<[u64; 10]>;
                let _: ::core::clone::AssertParamIsClone<[u64; 20]>;
                let _: ::core::clone::AssertParamIsClone<[Pubkey; 138]>;
                let _: ::core::clone::AssertParamIsClone<[u8; 1647]>;
                *self
            }
        }
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for Greeks {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for Greeks {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for Greeks {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for Greeks {
        fn discriminator() -> [u8; 8] {
            [247, 213, 170, 154, 43, 243, 146, 254]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for Greeks {
        fn try_deserialize(buf: &mut &[u8]) -> std::result::Result<Self, ProgramError> {
            if buf.len() < [247, 213, 170, 154, 43, 243, 146, 254].len() {
                return Err(anchor_lang::__private::ErrorCode::AccountDiscriminatorNotFound.into());
            }
            let given_disc = &buf[..8];
            if &[247, 213, 170, 154, 43, 243, 146, 254] != given_disc {
                return Err(anchor_lang::__private::ErrorCode::AccountDiscriminatorMismatch.into());
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> std::result::Result<Self, ProgramError> {
            let data: &[u8] = &buf[8..];
            let account = anchor_lang::__private::bytemuck::from_bytes(data);
            Ok(*account)
        }
    }
    #[automatically_derived]
    impl anchor_lang::Owner for Greeks {
        fn owner() -> Pubkey {
            crate::ID
        }
    }
    impl Greeks {
        pub fn get_mark_prices_slice(&self, expiry_index: usize) -> &[u64] {
            let head = expiry_index * NUM_PRODUCTS_PER_SERIES;
            &self.mark_prices[head..head + NUM_PRODUCTS_PER_SERIES]
        }
        pub fn get_product_greeks_slice(&self, expiry_index: usize) -> &[ProductGreeks] {
            let head = expiry_index * NUM_STRIKES;
            &self.product_greeks[head..head + NUM_STRIKES]
        }
        pub fn get_volatility_slice(&self, expiry_index: usize) -> &[u64] {
            let head = expiry_index * VOLATILITY_POINTS;
            &self.volatility[head..head + VOLATILITY_POINTS]
        }
        pub fn get_futures_price(&self, expiry_index: usize) -> u64 {
            self.mark_prices[expiry_index * NUM_PRODUCTS_PER_SERIES + NUM_PRODUCTS_PER_SERIES - 1]
        }
    }
    #[repr(packed)]
    pub struct ZetaGroup {
        pub nonce: u8,
        pub vault_nonce: u8,
        pub insurance_vault_nonce: u8,
        pub front_expiry_index: u8,
        pub halt_state: HaltState,
        pub underlying_mint: Pubkey,
        pub oracle: Pubkey,
        pub greeks: Pubkey,
        pub pricing_parameters: PricingParameters,
        pub margin_parameters: MarginParameters,
        pub products: [Product; 46],
        pub products_padding: [Product; 92],
        pub expiry_series: [ExpirySeries; 2],
        pub expiry_series_padding: [ExpirySeries; 4],
        pub total_insurance_vault_deposits: u64,
        pub padding: [u8; 1063],
    }
    #[automatically_derived]
    impl ZetaGroup {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for ZetaGroup {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for ZetaGroup {
        #[inline]
        fn clone(&self) -> ZetaGroup {
            {
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<HaltState>;
                let _: ::core::clone::AssertParamIsClone<Pubkey>;
                let _: ::core::clone::AssertParamIsClone<Pubkey>;
                let _: ::core::clone::AssertParamIsClone<Pubkey>;
                let _: ::core::clone::AssertParamIsClone<PricingParameters>;
                let _: ::core::clone::AssertParamIsClone<MarginParameters>;
                let _: ::core::clone::AssertParamIsClone<[Product; 46]>;
                let _: ::core::clone::AssertParamIsClone<[Product; 92]>;
                let _: ::core::clone::AssertParamIsClone<[ExpirySeries; 2]>;
                let _: ::core::clone::AssertParamIsClone<[ExpirySeries; 4]>;
                let _: ::core::clone::AssertParamIsClone<u64>;
                let _: ::core::clone::AssertParamIsClone<[u8; 1063]>;
                *self
            }
        }
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for ZetaGroup {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for ZetaGroup {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for ZetaGroup {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for ZetaGroup {
        fn discriminator() -> [u8; 8] {
            [121, 17, 210, 107, 109, 235, 14, 12]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for ZetaGroup {
        fn try_deserialize(buf: &mut &[u8]) -> std::result::Result<Self, ProgramError> {
            if buf.len() < [121, 17, 210, 107, 109, 235, 14, 12].len() {
                return Err(anchor_lang::__private::ErrorCode::AccountDiscriminatorNotFound.into());
            }
            let given_disc = &buf[..8];
            if &[121, 17, 210, 107, 109, 235, 14, 12] != given_disc {
                return Err(anchor_lang::__private::ErrorCode::AccountDiscriminatorMismatch.into());
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> std::result::Result<Self, ProgramError> {
            let data: &[u8] = &buf[8..];
            let account = anchor_lang::__private::bytemuck::from_bytes(data);
            Ok(*account)
        }
    }
    #[automatically_derived]
    impl anchor_lang::Owner for ZetaGroup {
        fn owner() -> Pubkey {
            crate::ID
        }
    }
    #[repr(packed)]
    pub struct HaltState {
        halted: bool,
        spot_price: u64,
        timestamp: u64,
        mark_prices_set: [bool; 2],
        _mark_prices_set_padding: [bool; 4],
        market_nodes_cleaned: [bool; 2],
        _market_nodes_cleaned_padding: [bool; 4],
        market_cleaned: [bool; 46],
        _market_cleaned_padding: [bool; 92],
    }
    #[automatically_derived]
    impl HaltState {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for HaltState {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for HaltState {
        #[inline]
        fn clone(&self) -> HaltState {
            {
                let _: ::core::clone::AssertParamIsClone<bool>;
                let _: ::core::clone::AssertParamIsClone<u64>;
                let _: ::core::clone::AssertParamIsClone<u64>;
                let _: ::core::clone::AssertParamIsClone<[bool; 2]>;
                let _: ::core::clone::AssertParamIsClone<[bool; 4]>;
                let _: ::core::clone::AssertParamIsClone<[bool; 2]>;
                let _: ::core::clone::AssertParamIsClone<[bool; 4]>;
                let _: ::core::clone::AssertParamIsClone<[bool; 46]>;
                let _: ::core::clone::AssertParamIsClone<[bool; 92]>;
                *self
            }
        }
    }
    #[repr(packed)]
    pub struct PricingParameters {
        pub option_trade_normalizer: AnchorDecimal,
        pub future_trade_normalizer: AnchorDecimal,
        pub max_volatility_retreat: AnchorDecimal,
        pub max_interest_retreat: AnchorDecimal,
        pub max_delta: u64,
        pub min_delta: u64,
        pub padding: [u8; 32],
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for PricingParameters {
        #[inline]
        fn default() -> PricingParameters {
            PricingParameters {
                option_trade_normalizer: ::core::default::Default::default(),
                future_trade_normalizer: ::core::default::Default::default(),
                max_volatility_retreat: ::core::default::Default::default(),
                max_interest_retreat: ::core::default::Default::default(),
                max_delta: ::core::default::Default::default(),
                min_delta: ::core::default::Default::default(),
                padding: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl PricingParameters {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for PricingParameters {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for PricingParameters {
        #[inline]
        fn clone(&self) -> PricingParameters {
            {
                let _: ::core::clone::AssertParamIsClone<AnchorDecimal>;
                let _: ::core::clone::AssertParamIsClone<AnchorDecimal>;
                let _: ::core::clone::AssertParamIsClone<AnchorDecimal>;
                let _: ::core::clone::AssertParamIsClone<AnchorDecimal>;
                let _: ::core::clone::AssertParamIsClone<u64>;
                let _: ::core::clone::AssertParamIsClone<u64>;
                let _: ::core::clone::AssertParamIsClone<[u8; 32]>;
                *self
            }
        }
    }
    #[repr(packed)]
    pub struct MarginParameters {
        pub future_margin_initial: u64,
        pub future_margin_maintenance: u64,
        pub option_mark_percentage_long_initial: u64,
        pub option_spot_percentage_long_initial: u64,
        pub option_spot_percentage_short_initial: u64,
        pub option_dynamic_percentage_short_initial: u64,
        pub option_mark_percentage_long_maintenance: u64,
        pub option_spot_percentage_long_maintenance: u64,
        pub option_spot_percentage_short_maintenance: u64,
        pub option_dynamic_percentage_short_maintenance: u64,
        pub option_short_put_cap_percentage: u64,
        pub padding: [u8; 32],
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for MarginParameters {
        #[inline]
        fn default() -> MarginParameters {
            MarginParameters {
                future_margin_initial: ::core::default::Default::default(),
                future_margin_maintenance: ::core::default::Default::default(),
                option_mark_percentage_long_initial: ::core::default::Default::default(),
                option_spot_percentage_long_initial: ::core::default::Default::default(),
                option_spot_percentage_short_initial: ::core::default::Default::default(),
                option_dynamic_percentage_short_initial: ::core::default::Default::default(),
                option_mark_percentage_long_maintenance: ::core::default::Default::default(),
                option_spot_percentage_long_maintenance: ::core::default::Default::default(),
                option_spot_percentage_short_maintenance: ::core::default::Default::default(),
                option_dynamic_percentage_short_maintenance: ::core::default::Default::default(),
                option_short_put_cap_percentage: ::core::default::Default::default(),
                padding: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl MarginParameters {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for MarginParameters {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for MarginParameters {
        #[inline]
        fn clone(&self) -> MarginParameters {
            {
                let _: ::core::clone::AssertParamIsClone<u64>;
                let _: ::core::clone::AssertParamIsClone<u64>;
                let _: ::core::clone::AssertParamIsClone<u64>;
                let _: ::core::clone::AssertParamIsClone<u64>;
                let _: ::core::clone::AssertParamIsClone<u64>;
                let _: ::core::clone::AssertParamIsClone<u64>;
                let _: ::core::clone::AssertParamIsClone<u64>;
                let _: ::core::clone::AssertParamIsClone<u64>;
                let _: ::core::clone::AssertParamIsClone<u64>;
                let _: ::core::clone::AssertParamIsClone<u64>;
                let _: ::core::clone::AssertParamIsClone<u64>;
                let _: ::core::clone::AssertParamIsClone<[u8; 32]>;
                *self
            }
        }
    }
    impl ZetaGroup {
        pub fn get_strike(&self, index: usize) -> Result<u64> {
            self.products[index].strike.get_strike()
        }
        pub fn get_products_slice(&self, expiry_index: usize) -> &[Product] {
            let head = expiry_index * NUM_PRODUCTS_PER_SERIES;
            &self.products[head..head + NUM_PRODUCTS_PER_SERIES]
        }
        pub fn get_product_and_expiry_index_by_key(
            &self,
            market: &Pubkey,
        ) -> Result<(usize, usize)> {
            let index = self
                .products
                .binary_search_by_key(&market, |product| &product.market);
            match index {
                Err(_) => {
                    ::solana_program::log::sol_log(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Error thrown at ", ":"],
                            &match (&"programs/zeta-cpi/src/zeta_account.rs", &184u32) {
                                args => [
                                    ::core::fmt::ArgumentV1::new_display(args.0),
                                    ::core::fmt::ArgumentV1::new_display(args.1),
                                ],
                            },
                        ));
                        res
                    });
                    Err(ErrorCode::InvalidProductMarketKey.into())
                }
                Ok(i) => Ok((i, self.get_expiry_index_by_product_index(i))),
            }
        }
        pub fn get_product_index_by_key(&self, market: &Pubkey) -> Result<usize> {
            let index = self
                .products
                .binary_search_by_key(&market, |product| &product.market);
            match index {
                Err(_) => {
                    ::solana_program::log::sol_log(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Error thrown at ", ":"],
                            &match (&"programs/zeta-cpi/src/zeta_account.rs", &195u32) {
                                args => [
                                    ::core::fmt::ArgumentV1::new_display(args.0),
                                    ::core::fmt::ArgumentV1::new_display(args.1),
                                ],
                            },
                        ));
                        res
                    });
                    Err(ErrorCode::InvalidProductMarketKey.into())
                }
                Ok(i) => Ok(i),
            }
        }
        pub fn get_expiry_series_by_key(&self, market: &Pubkey) -> Result<&ExpirySeries> {
            let index = self
                .products
                .binary_search_by_key(&market, |product| &product.market);
            match index {
                Err(_) => {
                    ::solana_program::log::sol_log(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Error thrown at ", ":"],
                            &match (&"programs/zeta-cpi/src/zeta_account.rs", &206u32) {
                                args => [
                                    ::core::fmt::ArgumentV1::new_display(args.0),
                                    ::core::fmt::ArgumentV1::new_display(args.1),
                                ],
                            },
                        ));
                        res
                    });
                    Err(ErrorCode::InvalidProductMarketKey.into())
                }
                Ok(i) => Ok(self.get_expiry_series_by_product_index(i)),
            }
        }
        pub fn get_expiry_series_by_product_index(&self, index: usize) -> &ExpirySeries {
            &self.expiry_series[self.get_expiry_index_by_product_index(index)]
        }
        pub fn get_expiry_index_by_product_index(&self, index: usize) -> usize {
            if !(index < self.products.len()) {
                ::core::panicking::panic("assertion failed: index < self.products.len()")
            };
            let expiry_index = index.checked_div(NUM_PRODUCTS_PER_SERIES).unwrap();
            if !(expiry_index < self.expiry_series.len()) {
                ::core::panicking::panic(
                    "assertion failed: expiry_index < self.expiry_series.len()",
                )
            };
            expiry_index
        }
        /// This function should validate an expiry index is:
        /// 1. Live
        /// 2. Strike is set
        /// 3. Pricing update was within the required intervals.
        pub fn validate_series_tradeable(
            &self,
            expiry_index: usize,
            current_timestamp: u64,
        ) -> Result<()> {
            let series_status = self.expiry_series[expiry_index].status()?;
            if series_status != ExpirySeriesStatus::Live {
                ::solana_program::log::sol_log(&{
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Series status = "],
                        &[::core::fmt::ArgumentV1::new_debug(&series_status)],
                    ));
                    res
                });
                return {
                    ::solana_program::log::sol_log(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Error thrown at ", ":"],
                            &match (&"programs/zeta-cpi/src/zeta_account.rs", &234u32) {
                                args => [
                                    ::core::fmt::ArgumentV1::new_display(args.0),
                                    ::core::fmt::ArgumentV1::new_display(args.1),
                                ],
                            },
                        ));
                        res
                    });
                    Err(ErrorCode::MarketNotLive.into())
                };
            }
            let products = self.get_products_slice(expiry_index);
            if !products[0].strike.is_set() {
                return {
                    ::solana_program::log::sol_log(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Error thrown at ", ":"],
                            &match (&"programs/zeta-cpi/src/zeta_account.rs", &241u32) {
                                args => [
                                    ::core::fmt::ArgumentV1::new_display(args.0),
                                    ::core::fmt::ArgumentV1::new_display(args.1),
                                ],
                            },
                        ));
                        res
                    });
                    Err(ErrorCode::ProductStrikeUninitialized.into())
                };
            }
            Ok(())
        }
        pub fn get_back_expiry_index(&self) -> usize {
            match self.front_expiry_index {
                0 => (self.expiry_series.len() - 1),
                _ => (self.front_expiry_index - 1).into(),
            }
        }
        pub fn get_back_expiry_ts(&self) -> u64 {
            self.expiry_series[self.get_back_expiry_index()].expiry_ts
        }
        pub fn get_previous_expiry_index(&self, expiry_index: usize) -> usize {
            match expiry_index {
                0 => (self.expiry_series.len() - 1),
                _ => (expiry_index - 1).into(),
            }
        }
    }
    #[repr(packed)]
    pub struct ExpirySeries {
        pub active_ts: u64,
        pub expiry_ts: u64,
        pub dirty: bool,
        pub padding: [u8; 15],
    }
    #[automatically_derived]
    impl ExpirySeries {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for ExpirySeries {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for ExpirySeries {
        #[inline]
        fn clone(&self) -> ExpirySeries {
            {
                let _: ::core::clone::AssertParamIsClone<u64>;
                let _: ::core::clone::AssertParamIsClone<u64>;
                let _: ::core::clone::AssertParamIsClone<bool>;
                let _: ::core::clone::AssertParamIsClone<[u8; 15]>;
                *self
            }
        }
    }
    impl ExpirySeries {
        pub fn status(&self) -> Result<ExpirySeriesStatus> {
            if self.active_ts == u64::default() || self.expiry_ts == u64::default() {
                return Ok(ExpirySeriesStatus::Uninitialized);
            };
            let clock = Clock::get()?;
            let current_ts = clock.unix_timestamp as u64;
            if self.dirty {
                Ok(ExpirySeriesStatus::ExpiredDirty)
            } else if current_ts < self.active_ts {
                Ok(ExpirySeriesStatus::Initialized)
            } else if current_ts >= self.active_ts && current_ts < self.expiry_ts {
                Ok(ExpirySeriesStatus::Live)
            } else {
                Ok(ExpirySeriesStatus::Expired)
            }
        }
    }
    #[repr(packed)]
    pub struct Strike {
        pub is_set: bool,
        pub value: u64,
    }
    #[automatically_derived]
    impl Strike {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Strike {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Strike {
        #[inline]
        fn clone(&self) -> Strike {
            {
                let _: ::core::clone::AssertParamIsClone<bool>;
                let _: ::core::clone::AssertParamIsClone<u64>;
                *self
            }
        }
    }
    impl Strike {
        pub fn is_set(&self) -> bool {
            self.is_set
        }
        pub fn get_strike(&self) -> Result<u64> {
            if !self.is_set() {
                return Err(ErrorCode::ProductStrikeUninitialized.into());
            }
            Ok(self.value)
        }
    }
    #[repr(packed)]
    pub struct Product {
        pub market: Pubkey,
        pub strike: Strike,
        pub dirty: bool,
        pub kind: Kind,
    }
    #[automatically_derived]
    impl Product {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Product {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Product {
        #[inline]
        fn clone(&self) -> Product {
            {
                let _: ::core::clone::AssertParamIsClone<Pubkey>;
                let _: ::core::clone::AssertParamIsClone<Strike>;
                let _: ::core::clone::AssertParamIsClone<bool>;
                let _: ::core::clone::AssertParamIsClone<Kind>;
                *self
            }
        }
    }
    #[repr(packed)]
    pub struct Position {
        pub position: i64,
        pub cost_of_trades: u64,
        pub closing_orders: u64,
        pub opening_orders: [u64; 2],
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for Position {
        #[inline]
        fn default() -> Position {
            Position {
                position: ::core::default::Default::default(),
                cost_of_trades: ::core::default::Default::default(),
                closing_orders: ::core::default::Default::default(),
                opening_orders: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl Position {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Position {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Position {
        #[inline]
        fn clone(&self) -> Position {
            {
                let _: ::core::clone::AssertParamIsClone<i64>;
                let _: ::core::clone::AssertParamIsClone<u64>;
                let _: ::core::clone::AssertParamIsClone<u64>;
                let _: ::core::clone::AssertParamIsClone<[u64; 2]>;
                *self
            }
        }
    }
    impl Position {
        pub fn check_open(&self, side: Side) -> bool {
            (side == Side::Bid && self.position >= 0) || (side == Side::Ask && self.position <= 0)
        }
        pub fn has_active_orders(&self) -> bool {
            self.opening_orders[0] != 0 || self.opening_orders[1] != 0 || self.closing_orders != 0
        }
        pub fn get_initial_margin(
            &self,
            mark_price: u64,
            product: &Product,
            spot: u64,
            margin_parameters: &MarginParameters,
        ) -> u64 {
            let initial_margin_requirement: u128 = match product.strike.get_strike() {
                Ok(strike) => {
                    let mut long_init_margin: u128 = 0;
                    let mut short_init_margin: u128 = 0;
                    if self.opening_orders[0] > 0 {
                        long_init_margin = (self.opening_orders[0] as u128)
                            .checked_mul(
                                get_initial_margin_per_lot(
                                    spot,
                                    strike,
                                    mark_price,
                                    product.kind,
                                    Side::Bid,
                                    margin_parameters,
                                )
                                .unwrap()
                                .try_into()
                                .unwrap(),
                            )
                            .unwrap();
                    }
                    if self.opening_orders[1] > 0 {
                        short_init_margin = (self.opening_orders[1] as u128)
                            .checked_mul(
                                get_initial_margin_per_lot(
                                    spot,
                                    strike,
                                    mark_price,
                                    product.kind,
                                    Side::Ask,
                                    margin_parameters,
                                )
                                .unwrap()
                                .try_into()
                                .unwrap(),
                            )
                            .unwrap();
                    }
                    long_init_margin
                        .checked_add(short_init_margin)
                        .unwrap()
                        .checked_div(POSITION_PRECISION_DENOMINATOR)
                        .unwrap()
                }
                Err(_) => 0,
            };
            initial_margin_requirement.try_into().unwrap()
        }
        pub fn get_maintenance_margin(
            &self,
            mark_price: u64,
            product: &Product,
            spot: u64,
            margin_parameters: &MarginParameters,
        ) -> u64 {
            let maintenance_margin_requirement = match product.strike.get_strike() {
                Ok(strike) => {
                    let mut margin: u128 = 0;
                    if self.position != 0 {
                        margin = (self.position.abs() as u128)
                            .checked_mul(
                                get_maintenance_margin_per_lot(
                                    spot,
                                    strike,
                                    mark_price,
                                    product.kind,
                                    self.position >= 0,
                                    margin_parameters,
                                )
                                .unwrap()
                                .try_into()
                                .unwrap(),
                            )
                            .unwrap()
                            .checked_div(POSITION_PRECISION_DENOMINATOR)
                            .unwrap()
                    }
                    margin
                }
                Err(_) => 0,
            };
            maintenance_margin_requirement.try_into().unwrap()
        }
        pub fn get_unrealized_pnl(&self, mark_price: u64) -> i64 {
            if self.position == 0 {
                0
            } else if self.position > 0 {
                (self.position as i128)
                    .checked_mul(mark_price as i128)
                    .unwrap()
                    .checked_div(POSITION_PRECISION_DENOMINATOR as i128)
                    .unwrap()
                    .checked_sub(self.cost_of_trades as i128)
                    .unwrap()
                    .try_into()
                    .unwrap()
            } else {
                (self.position as i128)
                    .checked_mul(mark_price as i128)
                    .unwrap()
                    .checked_div(POSITION_PRECISION_DENOMINATOR as i128)
                    .unwrap()
                    .checked_add(self.cost_of_trades as i128)
                    .unwrap()
                    .try_into()
                    .unwrap()
            }
        }
    }
    #[repr(packed)]
    pub struct MarginAccount {
        pub authority: Pubkey,
        pub nonce: u8,
        pub balance: u64,
        pub force_cancel_flag: bool,
        pub open_orders_nonce: [u8; 138],
        pub series_expiry: [u64; 6],
        pub positions: [Position; 46],
        pub positions_padding: [Position; 92],
        pub rebalance_amount: i64,
        pub _padding: [u8; 388],
    }
    #[automatically_derived]
    impl MarginAccount {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for MarginAccount {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for MarginAccount {
        #[inline]
        fn clone(&self) -> MarginAccount {
            {
                let _: ::core::clone::AssertParamIsClone<Pubkey>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<u64>;
                let _: ::core::clone::AssertParamIsClone<bool>;
                let _: ::core::clone::AssertParamIsClone<[u8; 138]>;
                let _: ::core::clone::AssertParamIsClone<[u64; 6]>;
                let _: ::core::clone::AssertParamIsClone<[Position; 46]>;
                let _: ::core::clone::AssertParamIsClone<[Position; 92]>;
                let _: ::core::clone::AssertParamIsClone<i64>;
                let _: ::core::clone::AssertParamIsClone<[u8; 388]>;
                *self
            }
        }
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for MarginAccount {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for MarginAccount {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for MarginAccount {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for MarginAccount {
        fn discriminator() -> [u8; 8] {
            [133, 220, 173, 213, 179, 211, 43, 238]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for MarginAccount {
        fn try_deserialize(buf: &mut &[u8]) -> std::result::Result<Self, ProgramError> {
            if buf.len() < [133, 220, 173, 213, 179, 211, 43, 238].len() {
                return Err(anchor_lang::__private::ErrorCode::AccountDiscriminatorNotFound.into());
            }
            let given_disc = &buf[..8];
            if &[133, 220, 173, 213, 179, 211, 43, 238] != given_disc {
                return Err(anchor_lang::__private::ErrorCode::AccountDiscriminatorMismatch.into());
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> std::result::Result<Self, ProgramError> {
            let data: &[u8] = &buf[8..];
            let account = anchor_lang::__private::bytemuck::from_bytes(data);
            Ok(*account)
        }
    }
    #[automatically_derived]
    impl anchor_lang::Owner for MarginAccount {
        fn owner() -> Pubkey {
            crate::ID
        }
    }
    impl MarginAccount {
        pub fn get_positions_slice(&self, expiry_index: usize) -> &[Position] {
            let head = expiry_index * NUM_PRODUCTS_PER_SERIES;
            &self.positions[head..head + NUM_PRODUCTS_PER_SERIES]
        }
        pub fn get_initial_margin(
            &self,
            greeks: &Greeks,
            zeta_group: &ZetaGroup,
            spot: u64,
        ) -> u64 {
            let initial_margin_requirement = self
                .positions
                .iter()
                .enumerate()
                .map(|(i, position)| {
                    position.get_initial_margin(
                        greeks.mark_prices[i],
                        &zeta_group.products[i],
                        spot,
                        &zeta_group.margin_parameters,
                    )
                })
                .sum();
            ::solana_program::log::sol_log(&{
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["Total Initial margin requirement = "],
                    &[::core::fmt::ArgumentV1::new_display(
                        &initial_margin_requirement,
                    )],
                ));
                res
            });
            initial_margin_requirement
        }
        pub fn get_maintenance_margin(
            &self,
            greeks: &Greeks,
            zeta_group: &ZetaGroup,
            spot: u64,
        ) -> u64 {
            let maintenance_margin_requirement = self
                .positions
                .iter()
                .enumerate()
                .map(|(i, position)| {
                    position.get_maintenance_margin(
                        greeks.mark_prices[i],
                        &zeta_group.products[i],
                        spot,
                        &zeta_group.margin_parameters,
                    )
                })
                .sum();
            ::solana_program::log::sol_log(&{
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["Total Maintenance requirement = "],
                    &[::core::fmt::ArgumentV1::new_display(
                        &maintenance_margin_requirement,
                    )],
                ));
                res
            });
            maintenance_margin_requirement
        }
        pub fn get_margin_requirement(
            &self,
            greeks: &Greeks,
            zeta_group: &ZetaGroup,
            spot: u64,
        ) -> u64 {
            self.get_initial_margin(greeks, zeta_group, spot)
                .checked_add(self.get_maintenance_margin(greeks, zeta_group, spot))
                .unwrap()
        }
        pub fn get_unrealized_pnl(&self, greeks: &Greeks) -> i64 {
            self.positions
                .iter()
                .enumerate()
                .map(|(i, position)| {
                    (position.get_unrealized_pnl(greeks.mark_prices[i]) as i128) as i64
                })
                .sum()
        }
        pub fn check_margin_requirement(
            &self,
            greeks: &Greeks,
            zeta_group: &ZetaGroup,
            native_spot: u64,
        ) -> bool {
            let pnl = self.get_unrealized_pnl(&greeks);
            let margin_requirement =
                i64::try_from(self.get_margin_requirement(&greeks, &zeta_group, native_spot))
                    .unwrap();
            let buffer = i64::try_from(self.balance)
                .unwrap()
                .checked_add(pnl)
                .unwrap()
                .checked_sub(margin_requirement)
                .unwrap();
            ::solana_program::log::sol_log(&{
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &[
                        "MarginAccount: Pnl = ",
                        ", margin_requirement = ",
                        ", buffer = ",
                        ", balance = ",
                    ],
                    &[
                        ::core::fmt::ArgumentV1::new_display(&pnl),
                        ::core::fmt::ArgumentV1::new_display(&margin_requirement),
                        ::core::fmt::ArgumentV1::new_display(&buffer),
                        ::core::fmt::ArgumentV1::new_display(&self.balance),
                    ],
                ));
                res
            });
            buffer > 0
        }
        pub fn has_active_orders(&self) -> bool {
            let has_active_orders = self
                .positions
                .iter()
                .find(|position| position.has_active_orders());
            match has_active_orders {
                Some(_) => true,
                None => false,
            }
        }
    }
    #[repr(u8)]
    pub enum ExpirySeriesStatus {
        Uninitialized = 0,
        Initialized = 1,
        Live = 2,
        Expired = 3,
        ExpiredDirty = 4,
    }
    impl ::core::marker::StructuralPartialEq for ExpirySeriesStatus {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for ExpirySeriesStatus {
        #[inline]
        fn eq(&self, other: &ExpirySeriesStatus) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for ExpirySeriesStatus {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&ExpirySeriesStatus::Uninitialized,) => {
                    ::core::fmt::Formatter::write_str(f, "Uninitialized")
                }
                (&ExpirySeriesStatus::Initialized,) => {
                    ::core::fmt::Formatter::write_str(f, "Initialized")
                }
                (&ExpirySeriesStatus::Live,) => ::core::fmt::Formatter::write_str(f, "Live"),
                (&ExpirySeriesStatus::Expired,) => ::core::fmt::Formatter::write_str(f, "Expired"),
                (&ExpirySeriesStatus::ExpiredDirty,) => {
                    ::core::fmt::Formatter::write_str(f, "ExpiredDirty")
                }
            }
        }
    }
    #[repr(u8)]
    pub enum Kind {
        Uninitialized = 0,
        Call = 1,
        Put = 2,
        Future = 3,
    }
    impl ::core::marker::StructuralPartialEq for Kind {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Kind {
        #[inline]
        fn eq(&self, other: &Kind) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Kind {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Kind::Uninitialized,) => ::core::fmt::Formatter::write_str(f, "Uninitialized"),
                (&Kind::Call,) => ::core::fmt::Formatter::write_str(f, "Call"),
                (&Kind::Put,) => ::core::fmt::Formatter::write_str(f, "Put"),
                (&Kind::Future,) => ::core::fmt::Formatter::write_str(f, "Future"),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Kind {
        #[inline]
        fn clone(&self) -> Kind {
            {
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Kind {}
    impl borsh::ser::BorshSerialize for Kind {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> core::result::Result<(), borsh::maybestd::io::Error> {
            let variant_idx: u8 = match self {
                Kind::Uninitialized => 0u8,
                Kind::Call => 1u8,
                Kind::Put => 2u8,
                Kind::Future => 3u8,
            };
            writer.write_all(&variant_idx.to_le_bytes())?;
            match self {
                Kind::Uninitialized => {}
                Kind::Call => {}
                Kind::Put => {}
                Kind::Future => {}
            }
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Kind {
        fn deserialize(buf: &mut &[u8]) -> core::result::Result<Self, borsh::maybestd::io::Error> {
            let variant_idx: u8 = borsh::BorshDeserialize::deserialize(buf)?;
            let return_value = match variant_idx {
                0u8 => Kind::Uninitialized,
                1u8 => Kind::Call,
                2u8 => Kind::Put,
                3u8 => Kind::Future,
                _ => {
                    let msg = {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Unexpected variant index: "],
                            &[::core::fmt::ArgumentV1::new_debug(&variant_idx)],
                        ));
                        res
                    };
                    return Err(borsh::maybestd::io::Error::new(
                        borsh::maybestd::io::ErrorKind::InvalidInput,
                        msg,
                    ));
                }
            };
            Ok(return_value)
        }
    }
    #[repr(u8)]
    pub enum Side {
        Uninitialized = 0,
        Bid = 1,
        Ask = 2,
    }
    impl borsh::ser::BorshSerialize for Side {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> core::result::Result<(), borsh::maybestd::io::Error> {
            let variant_idx: u8 = match self {
                Side::Uninitialized => 0u8,
                Side::Bid => 1u8,
                Side::Ask => 2u8,
            };
            writer.write_all(&variant_idx.to_le_bytes())?;
            match self {
                Side::Uninitialized => {}
                Side::Bid => {}
                Side::Ask => {}
            }
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Side {
        fn deserialize(buf: &mut &[u8]) -> core::result::Result<Self, borsh::maybestd::io::Error> {
            let variant_idx: u8 = borsh::BorshDeserialize::deserialize(buf)?;
            let return_value = match variant_idx {
                0u8 => Side::Uninitialized,
                1u8 => Side::Bid,
                2u8 => Side::Ask,
                _ => {
                    let msg = {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Unexpected variant index: "],
                            &[::core::fmt::ArgumentV1::new_debug(&variant_idx)],
                        ));
                        res
                    };
                    return Err(borsh::maybestd::io::Error::new(
                        borsh::maybestd::io::ErrorKind::InvalidInput,
                        msg,
                    ));
                }
            };
            Ok(return_value)
        }
    }
    impl ::core::marker::StructuralPartialEq for Side {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Side {
        #[inline]
        fn eq(&self, other: &Side) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
    impl ::core::marker::StructuralEq for Side {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for Side {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Side {
        #[inline]
        fn clone(&self) -> Side {
            {
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Side {}
}
pub mod zeta_client {
    use crate::zeta_constants::*;
    use crate::zeta_context::*;
    use crate::*;
    use cpi_interface::global_interface;
    /// Zeta Program Client
    /// Defines a clean interface and set of helper functions to make CPI calls to the Zeta Program
    pub trait ZetaInterface<'info, T: Accounts<'info>> {
        fn initialize_margin_account(ctx: Context<T>, nonce: u8) -> ProgramResult;
        fn deposit(ctx: Context<T>, amount: u64) -> ProgramResult;
        fn withdraw(ctx: Context<T>, amount: u64) -> ProgramResult;
        fn initialize_open_orders(ctx: Context<T>, nonce: u8, _map_nonce: u8) -> ProgramResult;
        fn place_order(
            ctx: Context<T>,
            price: u64,
            size: u64,
            side: Side,
            client_order_id: Option<u64>,
        ) -> ProgramResult;
        fn cancel_order(ctx: Context<T>, side: Side, order_id: u128) -> ProgramResult;
    }
    /// Anchor generated module for invoking programs implementing an
    /// `#[global_interface]` via CPI.
    mod zeta_interface {
        use super::*;
        pub fn initialize_margin_account<
            'a,
            'b,
            'c,
            'info,
            T: anchor_lang::Accounts<'info>
                + anchor_lang::ToAccountMetas
                + anchor_lang::ToAccountInfos<'info>,
        >(
            ctx: anchor_lang::CpiContext<'a, 'b, 'c, 'info, T>,
            nonce: u8,
        ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
            struct Args {
                nonce: u8,
            }
            impl borsh::ser::BorshSerialize for Args
            where
                u8: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.nonce, writer)?;
                    Ok(())
                }
            }
            impl borsh::de::BorshDeserialize for Args
            where
                u8: borsh::BorshDeserialize,
            {
                fn deserialize(
                    buf: &mut &[u8],
                ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                    Ok(Self {
                        nonce: borsh::BorshDeserialize::deserialize(buf)?,
                    })
                }
            }
            let ix = {
                let ix = Args { nonce };
                let mut ix_data = anchor_lang::AnchorSerialize::try_to_vec(&ix)
                    .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotSerialize)?;
                let mut data = [67, 235, 66, 102, 167, 171, 120, 197].to_vec();
                data.append(&mut ix_data);
                let accounts = ctx.to_account_metas(None);
                anchor_lang::solana_program::instruction::Instruction {
                    program_id: *ctx.program.key,
                    accounts,
                    data,
                }
            };
            let mut acc_infos = ctx.to_account_infos();
            acc_infos.push(ctx.program.clone());
            anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
        }
        pub fn deposit<
            'a,
            'b,
            'c,
            'info,
            T: anchor_lang::Accounts<'info>
                + anchor_lang::ToAccountMetas
                + anchor_lang::ToAccountInfos<'info>,
        >(
            ctx: anchor_lang::CpiContext<'a, 'b, 'c, 'info, T>,
            amount: u64,
        ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
            struct Args {
                amount: u64,
            }
            impl borsh::ser::BorshSerialize for Args
            where
                u64: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.amount, writer)?;
                    Ok(())
                }
            }
            impl borsh::de::BorshDeserialize for Args
            where
                u64: borsh::BorshDeserialize,
            {
                fn deserialize(
                    buf: &mut &[u8],
                ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                    Ok(Self {
                        amount: borsh::BorshDeserialize::deserialize(buf)?,
                    })
                }
            }
            let ix = {
                let ix = Args { amount };
                let mut ix_data = anchor_lang::AnchorSerialize::try_to_vec(&ix)
                    .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotSerialize)?;
                let mut data = [242, 35, 198, 137, 82, 225, 242, 182].to_vec();
                data.append(&mut ix_data);
                let accounts = ctx.to_account_metas(None);
                anchor_lang::solana_program::instruction::Instruction {
                    program_id: *ctx.program.key,
                    accounts,
                    data,
                }
            };
            let mut acc_infos = ctx.to_account_infos();
            acc_infos.push(ctx.program.clone());
            anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
        }
        pub fn withdraw<
            'a,
            'b,
            'c,
            'info,
            T: anchor_lang::Accounts<'info>
                + anchor_lang::ToAccountMetas
                + anchor_lang::ToAccountInfos<'info>,
        >(
            ctx: anchor_lang::CpiContext<'a, 'b, 'c, 'info, T>,
            amount: u64,
        ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
            struct Args {
                amount: u64,
            }
            impl borsh::ser::BorshSerialize for Args
            where
                u64: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.amount, writer)?;
                    Ok(())
                }
            }
            impl borsh::de::BorshDeserialize for Args
            where
                u64: borsh::BorshDeserialize,
            {
                fn deserialize(
                    buf: &mut &[u8],
                ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                    Ok(Self {
                        amount: borsh::BorshDeserialize::deserialize(buf)?,
                    })
                }
            }
            let ix = {
                let ix = Args { amount };
                let mut ix_data = anchor_lang::AnchorSerialize::try_to_vec(&ix)
                    .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotSerialize)?;
                let mut data = [183, 18, 70, 156, 148, 109, 161, 34].to_vec();
                data.append(&mut ix_data);
                let accounts = ctx.to_account_metas(None);
                anchor_lang::solana_program::instruction::Instruction {
                    program_id: *ctx.program.key,
                    accounts,
                    data,
                }
            };
            let mut acc_infos = ctx.to_account_infos();
            acc_infos.push(ctx.program.clone());
            anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
        }
        pub fn initialize_open_orders<
            'a,
            'b,
            'c,
            'info,
            T: anchor_lang::Accounts<'info>
                + anchor_lang::ToAccountMetas
                + anchor_lang::ToAccountInfos<'info>,
        >(
            ctx: anchor_lang::CpiContext<'a, 'b, 'c, 'info, T>,
            nonce: u8,
            _map_nonce: u8,
        ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
            struct Args {
                nonce: u8,
                _map_nonce: u8,
            }
            impl borsh::ser::BorshSerialize for Args
            where
                u8: borsh::ser::BorshSerialize,
                u8: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.nonce, writer)?;
                    borsh::BorshSerialize::serialize(&self._map_nonce, writer)?;
                    Ok(())
                }
            }
            impl borsh::de::BorshDeserialize for Args
            where
                u8: borsh::BorshDeserialize,
                u8: borsh::BorshDeserialize,
            {
                fn deserialize(
                    buf: &mut &[u8],
                ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                    Ok(Self {
                        nonce: borsh::BorshDeserialize::deserialize(buf)?,
                        _map_nonce: borsh::BorshDeserialize::deserialize(buf)?,
                    })
                }
            }
            let ix = {
                let ix = Args { nonce, _map_nonce };
                let mut ix_data = anchor_lang::AnchorSerialize::try_to_vec(&ix)
                    .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotSerialize)?;
                let mut data = [55, 234, 16, 82, 100, 42, 126, 192].to_vec();
                data.append(&mut ix_data);
                let accounts = ctx.to_account_metas(None);
                anchor_lang::solana_program::instruction::Instruction {
                    program_id: *ctx.program.key,
                    accounts,
                    data,
                }
            };
            let mut acc_infos = ctx.to_account_infos();
            acc_infos.push(ctx.program.clone());
            anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
        }
        pub fn place_order<
            'a,
            'b,
            'c,
            'info,
            T: anchor_lang::Accounts<'info>
                + anchor_lang::ToAccountMetas
                + anchor_lang::ToAccountInfos<'info>,
        >(
            ctx: anchor_lang::CpiContext<'a, 'b, 'c, 'info, T>,
            price: u64,
            size: u64,
            side: Side,
            client_order_id: Option<u64>,
        ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
            struct Args {
                price: u64,
                size: u64,
                side: Side,
                client_order_id: Option<u64>,
            }
            impl borsh::ser::BorshSerialize for Args
            where
                u64: borsh::ser::BorshSerialize,
                u64: borsh::ser::BorshSerialize,
                Side: borsh::ser::BorshSerialize,
                Option<u64>: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.price, writer)?;
                    borsh::BorshSerialize::serialize(&self.size, writer)?;
                    borsh::BorshSerialize::serialize(&self.side, writer)?;
                    borsh::BorshSerialize::serialize(&self.client_order_id, writer)?;
                    Ok(())
                }
            }
            impl borsh::de::BorshDeserialize for Args
            where
                u64: borsh::BorshDeserialize,
                u64: borsh::BorshDeserialize,
                Side: borsh::BorshDeserialize,
                Option<u64>: borsh::BorshDeserialize,
            {
                fn deserialize(
                    buf: &mut &[u8],
                ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                    Ok(Self {
                        price: borsh::BorshDeserialize::deserialize(buf)?,
                        size: borsh::BorshDeserialize::deserialize(buf)?,
                        side: borsh::BorshDeserialize::deserialize(buf)?,
                        client_order_id: borsh::BorshDeserialize::deserialize(buf)?,
                    })
                }
            }
            let ix = {
                let ix = Args {
                    price,
                    size,
                    side,
                    client_order_id,
                };
                let mut ix_data = anchor_lang::AnchorSerialize::try_to_vec(&ix)
                    .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotSerialize)?;
                let mut data = [51, 194, 155, 175, 109, 130, 96, 106].to_vec();
                data.append(&mut ix_data);
                let accounts = ctx.to_account_metas(None);
                anchor_lang::solana_program::instruction::Instruction {
                    program_id: *ctx.program.key,
                    accounts,
                    data,
                }
            };
            let mut acc_infos = ctx.to_account_infos();
            acc_infos.push(ctx.program.clone());
            anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
        }
        pub fn cancel_order<
            'a,
            'b,
            'c,
            'info,
            T: anchor_lang::Accounts<'info>
                + anchor_lang::ToAccountMetas
                + anchor_lang::ToAccountInfos<'info>,
        >(
            ctx: anchor_lang::CpiContext<'a, 'b, 'c, 'info, T>,
            side: Side,
            order_id: u128,
        ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
            struct Args {
                side: Side,
                order_id: u128,
            }
            impl borsh::ser::BorshSerialize for Args
            where
                Side: borsh::ser::BorshSerialize,
                u128: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.side, writer)?;
                    borsh::BorshSerialize::serialize(&self.order_id, writer)?;
                    Ok(())
                }
            }
            impl borsh::de::BorshDeserialize for Args
            where
                Side: borsh::BorshDeserialize,
                u128: borsh::BorshDeserialize,
            {
                fn deserialize(
                    buf: &mut &[u8],
                ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                    Ok(Self {
                        side: borsh::BorshDeserialize::deserialize(buf)?,
                        order_id: borsh::BorshDeserialize::deserialize(buf)?,
                    })
                }
            }
            let ix = {
                let ix = Args { side, order_id };
                let mut ix_data = anchor_lang::AnchorSerialize::try_to_vec(&ix)
                    .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotSerialize)?;
                let mut data = [95, 129, 237, 240, 8, 49, 223, 132].to_vec();
                data.append(&mut ix_data);
                let accounts = ctx.to_account_metas(None);
                anchor_lang::solana_program::instruction::Instruction {
                    program_id: *ctx.program.key,
                    accounts,
                    data,
                }
            };
            let mut acc_infos = ctx.to_account_infos();
            acc_infos.push(ctx.program.clone());
            anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
        }
    }
    pub fn initialize_margin_account<'info>(
        zeta_program: AccountInfo<'info>,
        cpi_accounts: InitializeMarginAccount<'info>,
    ) -> ProgramResult {
        let (_pda, nonce) = Pubkey::find_program_address(
            &[
                MARGIN_SEED.as_ref(),
                cpi_accounts.zeta_group.key.as_ref(),
                cpi_accounts.authority.key.as_ref(),
            ],
            &zeta_program.key.clone(),
        );
        let cpi_ctx = CpiContext::new(zeta_program, cpi_accounts);
        zeta_interface::initialize_margin_account(cpi_ctx, nonce)
    }
    pub fn deposit<'info>(
        zeta_program: AccountInfo<'info>,
        cpi_accounts: Deposit<'info>,
        amount: u64,
    ) -> ProgramResult {
        let cpi_ctx = CpiContext::new(zeta_program, cpi_accounts);
        zeta_interface::deposit(cpi_ctx, amount)
    }
    pub fn withdraw<'info>(
        zeta_program: AccountInfo<'info>,
        cpi_accounts: Withdraw<'info>,
        amount: u64,
    ) -> ProgramResult {
        let cpi_ctx = CpiContext::new(zeta_program, cpi_accounts);
        zeta_interface::withdraw(cpi_ctx, amount)
    }
    pub fn initialize_open_orders<'info>(
        zeta_program: AccountInfo<'info>,
        cpi_accounts: InitializeOpenOrders<'info>,
    ) -> ProgramResult {
        let (_, nonce) = Pubkey::find_program_address(
            &[
                OPEN_ORDERS_SEED.as_bytes(),
                cpi_accounts.dex_program.key.as_ref(),
                cpi_accounts.market.key.as_ref(),
                cpi_accounts.authority.key.as_ref(),
            ],
            &zeta_program.key.clone(),
        );
        let (_, map_nonce) = Pubkey::find_program_address(
            &[cpi_accounts.open_orders.key.as_ref()],
            &zeta_program.key.clone(),
        );
        let cpi_ctx = CpiContext::new(zeta_program, cpi_accounts);
        zeta_interface::initialize_open_orders(cpi_ctx, nonce, map_nonce)
    }
    pub fn place_order<'info>(
        zeta_program: AccountInfo<'info>,
        cpi_accounts: PlaceOrder<'info>,
        price: u64,
        size: u64,
        side: Side,
        client_order_id: Option<u64>,
    ) -> ProgramResult {
        let cpi_ctx = CpiContext::new(zeta_program, cpi_accounts);
        zeta_interface::place_order(cpi_ctx, price, size, side, client_order_id)
    }
    pub fn cancel_order<'info>(
        zeta_program: AccountInfo<'info>,
        cpi_accounts: CancelOrder<'info>,
        side: Side,
        order_id: u128,
    ) -> ProgramResult {
        let cpi_ctx = CpiContext::new(zeta_program, cpi_accounts);
        zeta_interface::cancel_order(cpi_ctx, side, order_id)
    }
}
pub mod zeta_constants {
    pub const STATE_SEED: &str = "state";
    pub const GREEKS_SEED: &str = "greeks";
    pub const MARKET_NODE_SEED: &str = "market-node";
    pub const OPEN_ORDERS_SEED: &str = "open-orders";
    pub const VAULT_SEED: &str = "vault";
    pub const SERUM_VAULT_SEED: &str = "serum-vault";
    pub const ZETA_VAULT_SEED: &str = "zeta-vault";
    pub const ZETA_GROUP_SEED: &str = "zeta-group";
    pub const ZETA_INSURANCE_VAULT_SEED: &str = "zeta-insurance-vault";
    pub const WHITELIST_INSURANCE_SEED: &str = "whitelist-insurance";
    pub const USER_INSURANCE_DEPOSIT_SEED: &str = "user-insurance-deposit";
    pub const WHITELIST_TRADING_FEES_SEED: &str = "whitelist-trading-fees";
    pub const SETTLEMENT_SEED: &str = "settlement";
    pub const MARGIN_SEED: &str = "margin";
    pub const UNDERLYING_SEED: &str = "underlying";
    pub const SERUM_SEED: &str = "serum";
    pub const MINT_AUTH_SEED: &str = "mint-auth";
    pub const BASE_MINT_SEED: &str = "base-mint";
    pub const QUOTE_MINT_SEED: &str = "quote-mint";
    pub const MARKET_SEED: &str = "market";
    pub const MARKET_INDEXES_SEED: &str = "market-indexes";
    pub const SOCIALIZED_LOSS_SEED: &str = "socialized-loss";
    pub const PLATFORM_PRECISION: u32 = 6;
    pub const HALT_SPOT_PRICE_PRECISION: u32 = 6;
    pub const PRICING_PRECISION: u32 = 12;
    pub const POSITION_PRECISION: u32 = 3;
    pub const EVENT_CRANK_LIMIT: u16 = 25;
    pub const DEFAULT_MINT_LOT_SIZE: u64 = 1;
    pub const DISCRIMINATOR_SIZE: usize = 8;
    pub const MARK_PRICE_PERCENTAGE: u128 = 100;
    pub const PRICE_BAND_MULTIPLE: u64 = 10;
    pub const NUM_STRIKES: usize = 11;
    pub const NUM_PRODUCTS_PER_SERIES: usize = NUM_STRIKES * 2 + 1;
    pub const SERIES_FUTURE_INDEX: usize = NUM_PRODUCTS_PER_SERIES - 1;
    pub const ACTIVE_EXPIRIES: usize = 2;
    pub const TOTAL_EXPIRIES: usize = 6;
    pub const ACTIVE_MARKETS: usize = ACTIVE_EXPIRIES * NUM_PRODUCTS_PER_SERIES;
    pub const TOTAL_MARKETS: usize = TOTAL_EXPIRIES * NUM_PRODUCTS_PER_SERIES;
    pub const MARKET_INDEX_LIMIT: usize = 40;
    pub const MAX_INTEREST_RATE: i64 = 1_000_000_000_000;
    pub const MIN_INTEREST_RATE: i64 = -1_000_000_000_000;
    pub const MIN_VOLATILITY: u64 = 100_000_000_000;
    pub const MAX_VOLATILITY: u64 = 5_000_000_000_000;
    pub const MAX_VOLATILITY_RETREAT_PERCENT: u64 = 5;
    pub const MAX_INTEREST_RATE_RETREAT: i64 = 20_000_000_000;
    pub const VOLATILITY_POINTS: usize = 5;
    pub const SECONDS_IN_A_YEAR: u64 = 31_536_000;
    pub const NATIVE_PRECISION_DENOMINATOR: u128 = 100_000_000;
    pub const POSITION_PRECISION_DENOMINATOR: u128 = 1_000;
    pub const FUTURE_MARGIN_INITIAL: u128 = 15_000_000;
    pub const FUTURE_MARGIN_MAINTENANCE: u128 = 7_500_000;
    pub const OPTION_MARK_PCT_LONG_INITIAL: u128 = 100_000_000;
    pub const OPTION_SPOT_PCT_LONG_INITIAL: u128 = 15_000_000;
    pub const OPTION_SPOT_PCT_SHORT_INITIAL: u128 = 10_000_000;
    pub const OPTION_BASE_PCT_SHORT_INITIAL: u128 = 25_000_000;
    pub const OPTION_MARK_PCT_LONG_MAINTENANCE: u128 = 100_000_000;
    pub const OPTION_SPOT_PCT_LONG_MAINTENANCE: u128 = 7_500_000;
    pub const OPTION_SPOT_PCT_SHORT_MAINTENANCE: u128 = 12_500_000;
    pub const OPTION_BASE_PCT_SHORT_MAINTENANCE: u128 = 5_000_000;
    #[cfg(not(feature = "epoch-offset"))]
    pub const EPOCH_OFFSET: u64 = 0;
    pub const EPSILON_ERROR: u64 = 1_000;
}
pub mod zeta_context {
    use crate::*;
    use anchor_spl::token::Token;
    /// Zeta Context
    /// Leave this as is, it defines the instruction context for the zeta program
    pub struct InitializeMarginAccount<'info> {
        #[account(mut)]
        pub margin_account: AccountInfo<'info>,
        #[account(mut)]
        pub authority: Signer<'info>,
        pub zeta_program: AccountInfo<'info>,
        pub system_program: Program<'info, System>,
        pub zeta_group: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info> for InitializeMarginAccount<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
            ix_data: &[u8],
        ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError>
        {
            let margin_account: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let authority: Signer =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let zeta_program: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let system_program: anchor_lang::Program<System> =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let zeta_group: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            if !margin_account.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            if !authority.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            Ok(InitializeMarginAccount {
                margin_account,
                authority,
                zeta_program,
                system_program,
                zeta_group,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeMarginAccount<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.margin_account.to_account_infos());
            account_infos.extend(self.authority.to_account_infos());
            account_infos.extend(self.zeta_program.to_account_infos());
            account_infos.extend(self.system_program.to_account_infos());
            account_infos.extend(self.zeta_group.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for InitializeMarginAccount<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.margin_account.to_account_metas(None));
            account_metas.extend(self.authority.to_account_metas(None));
            account_metas.extend(self.zeta_program.to_account_metas(None));
            account_metas.extend(self.system_program.to_account_metas(None));
            account_metas.extend(self.zeta_group.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for InitializeMarginAccount<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
            anchor_lang::AccountsExit::exit(&self.margin_account, program_id)?;
            anchor_lang::AccountsExit::exit(&self.authority, program_id)?;
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
    pub(crate) mod __client_accounts_initialize_margin_account {
        use super::*;
        use anchor_lang::prelude::borsh;
        pub struct InitializeMarginAccount {
            pub margin_account: anchor_lang::solana_program::pubkey::Pubkey,
            pub authority: anchor_lang::solana_program::pubkey::Pubkey,
            pub zeta_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub zeta_group: anchor_lang::solana_program::pubkey::Pubkey,
        }
        impl borsh::ser::BorshSerialize for InitializeMarginAccount
        where
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
                borsh::BorshSerialize::serialize(&self.margin_account, writer)?;
                borsh::BorshSerialize::serialize(&self.authority, writer)?;
                borsh::BorshSerialize::serialize(&self.zeta_program, writer)?;
                borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                borsh::BorshSerialize::serialize(&self.zeta_group, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for InitializeMarginAccount {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.margin_account,
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.authority,
                    true,
                ));
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.zeta_program,
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.system_program,
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.zeta_group,
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
    pub(crate) mod __cpi_client_accounts_initialize_margin_account {
        use super::*;
        pub struct InitializeMarginAccount<'info> {
            pub margin_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub zeta_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub system_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub zeta_group: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for InitializeMarginAccount<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.margin_account),
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.authority),
                    true,
                ));
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.zeta_program),
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.system_program),
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_r eadonly(
                        anchor_lang::Key::key(&self.zeta_group),
                        false,
                    ),
                );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeMarginAccount<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.margin_account,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.authority));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.zeta_program,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.system_program,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.zeta_group,
                ));
                account_infos
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'info> ::core::clone::Clone for InitializeMarginAccount<'info> {
        #[inline]
        fn clone(&self) -> InitializeMarginAccount<'info> {
            match *self {
                InitializeMarginAccount {
                    margin_account: ref __self_0_0,
                    authority: ref __self_0_1,
                    zeta_program: ref __self_0_2,
                    system_program: ref __self_0_3,
                    zeta_group: ref __self_0_4,
                } => InitializeMarginAccount {
                    margin_account: ::core::clone::Clone::clone(&(*__self_0_0)),
                    authority: ::core::clone::Clone::clone(&(*__self_0_1)),
                    zeta_program: ::core::clone::Clone::clone(&(*__self_0_2)),
                    system_program: ::core::clone::Clone::clone(&(*__self_0_3)),
                    zeta_group: ::core::clone::Clone::clone(&(*__self_0_4)),
                },
            }
        }
    }
    pub struct Deposit<'info> {
        pub zeta_group: AccountInfo<'info>,
        #[account(mut)]
        pub margin_account: AccountInfo<'info>,
        #[account(mut)]
        pub vault: AccountInfo<'info>,
        #[account(mut)]
        pub user_token_account: AccountInfo<'info>,
        #[account(mut)]
        pub socialized_loss_account: AccountInfo<'info>,
        pub authority: Signer<'info>,
        pub token_program: Program<'info, Token>,
        pub state: AccountInfo<'info>,
        pub greeks: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info> for Deposit<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
            ix_data: &[u8],
        ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError>
        {
            let zeta_group: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let margin_account: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let vault: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let user_token_account: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let socialized_loss_account: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let authority: Signer =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let token_program: anchor_lang::Program<Token> =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let state: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let greeks: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            if !margin_account.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            if !vault.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            if !user_token_account.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            if !socialized_loss_account.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            Ok(Deposit {
                zeta_group,
                margin_account,
                vault,
                user_token_account,
                socialized_loss_account,
                authority,
                token_program,
                state,
                greeks,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for Deposit<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.zeta_group.to_account_infos());
            account_infos.extend(self.margin_account.to_account_infos());
            account_infos.extend(self.vault.to_account_infos());
            account_infos.extend(self.user_token_account.to_account_infos());
            account_infos.extend(self.socialized_loss_account.to_account_infos());
            account_infos.extend(self.authority.to_account_infos());
            account_infos.extend(self.token_program.to_account_infos());
            account_infos.extend(self.state.to_account_infos());
            account_infos.extend(self.greeks.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for Deposit<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.zeta_group.to_account_metas(None));
            account_metas.extend(self.margin_account.to_account_metas(None));
            account_metas.extend(self.vault.to_account_metas(None));
            account_metas.extend(self.user_token_account.to_account_metas(None));
            account_metas.extend(self.socialized_loss_account.to_account_metas(None));
            account_metas.extend(self.authority.to_account_metas(None));
            account_metas.extend(self.token_program.to_account_metas(None));
            account_metas.extend(self.state.to_account_metas(None));
            account_metas.extend(self.greeks.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for Deposit<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
            anchor_lang::AccountsExit::exit(&self.margin_account, program_id)?;
            anchor_lang::AccountsExit::exit(&self.vault, program_id)?;
            anchor_lang::AccountsExit::exit(&self.user_token_account, program_id)?;
            anchor_lang::AccountsExit::exit(&self.socialized_loss_account, program_id)?;
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
    pub(crate) mod __client_accounts_deposit {
        use super::*;
        use anchor_lang::prelude::borsh;
        pub struct Deposit {
            pub zeta_group: anchor_lang::solana_program::pubkey::Pubkey,
            pub margin_account: anchor_lang::solana_program::pubkey::Pubkey,
            pub vault: anchor_lang::solana_program::pubkey::Pubkey,
            pub user_token_account: anchor_lang::solana_program::pubkey::Pubkey,
            pub socialized_loss_account: anchor_lang::solana_program::pubkey::Pubkey,
            pub authority: anchor_lang::solana_program::pubkey::Pubkey,
            pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub state: anchor_lang::solana_program::pubkey::Pubkey,
            pub greeks: anchor_lang::solana_program::pubkey::Pubkey,
        }
        impl borsh::ser::BorshSerialize for Deposit
        where
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
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
                borsh::BorshSerialize::serialize(&self.zeta_group, writer)?;
                borsh::BorshSerialize::serialize(&self.margin_account, writer)?;
                borsh::BorshSerialize::serialize(&self.vault, writer)?;
                borsh::BorshSerialize::serialize(&self.user_token_account, writer)?;
                borsh::BorshSerialize::serialize(&self.socialized_loss_account, writer)?;
                borsh::BorshSerialize::serialize(&self.authority, writer)?;
                borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                borsh::BorshSerialize::serialize(&self.state, writer)?;
                borsh::BorshSerialize::serialize(&self.greeks, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for Deposit {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.zeta_group,
                        false,
                    ),
                );
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.margin_account,
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.vault, false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.user_token_account,
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.socialized_loss_account,
                    false,
                ));
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.authority,
                        true,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.token_program,
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.state, false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.greeks,
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
    pub(crate) mod __cpi_client_accounts_deposit {
        use super::*;
        pub struct Deposit<'info> {
            pub zeta_group: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub margin_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub user_token_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub socialized_loss_account:
                anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub state: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub greeks: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for Deposit<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.zeta_group),
                        false,
                    ),
                );
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.margin_account),
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.vault),
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.user_token_account),
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.socialized_loss_account),
                    false,
                ));
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.authority),
                        true,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.token_program),
                        false,
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
                        anchor_lang::Key::key(&self.greeks),
                        false,
                    ),
                );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for Deposit<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.zeta_group,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.margin_account,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vault));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.user_token_account,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.socialized_loss_account,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.authority));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.token_program,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.state));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.greeks));
                account_infos
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'info> ::core::clone::Clone for Deposit<'info> {
        #[inline]
        fn clone(&self) -> Deposit<'info> {
            match *self {
                Deposit {
                    zeta_group: ref __self_0_0,
                    margin_account: ref __self_0_1,
                    vault: ref __self_0_2,
                    user_token_account: ref __self_0_3,
                    socialized_loss_account: ref __self_0_4,
                    authority: ref __self_0_5,
                    token_program: ref __self_0_6,
                    state: ref __self_0_7,
                    greeks: ref __self_0_8,
                } => Deposit {
                    zeta_group: ::core::clone::Clone::clone(&(*__self_0_0)),
                    margin_account: ::core::clone::Clone::clone(&(*__self_0_1)),
                    vault: ::core::clone::Clone::clone(&(*__self_0_2)),
                    user_token_account: ::core::clone::Clone::clone(&(*__self_0_3)),
                    socialized_loss_account: ::core::clone::Clone::clone(&(*__self_0_4)),
                    authority: ::core::clone::Clone::clone(&(*__self_0_5)),
                    token_program: ::core::clone::Clone::clone(&(*__self_0_6)),
                    state: ::core::clone::Clone::clone(&(*__self_0_7)),
                    greeks: ::core::clone::Clone::clone(&(*__self_0_8)),
                },
            }
        }
    }
    pub struct Withdraw<'info> {
        pub state: AccountInfo<'info>,
        pub zeta_group: AccountInfo<'info>,
        #[account(mut)]
        pub vault: AccountInfo<'info>,
        #[account(mut)]
        pub margin_account: AccountInfo<'info>,
        #[account(mut)]
        pub user_token_account: AccountInfo<'info>,
        pub token_program: Program<'info, Token>,
        pub authority: Signer<'info>,
        #[account(mut)]
        pub greeks: AccountInfo<'info>,
        pub oracle: AccountInfo<'info>,
        #[account(mut)]
        pub socialized_loss_account: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info> for Withdraw<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
            ix_data: &[u8],
        ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError>
        {
            let state: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let zeta_group: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let vault: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let margin_account: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let user_token_account: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let token_program: anchor_lang::Program<Token> =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let authority: Signer =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let greeks: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let oracle: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let socialized_loss_account: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            if !vault.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            if !margin_account.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            if !user_token_account.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            if !greeks.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            if !socialized_loss_account.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            Ok(Withdraw {
                state,
                zeta_group,
                vault,
                margin_account,
                user_token_account,
                token_program,
                authority,
                greeks,
                oracle,
                socialized_loss_account,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for Withdraw<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.state.to_account_infos());
            account_infos.extend(self.zeta_group.to_account_infos());
            account_infos.extend(self.vault.to_account_infos());
            account_infos.extend(self.margin_account.to_account_infos());
            account_infos.extend(self.user_token_account.to_account_infos());
            account_infos.extend(self.token_program.to_account_infos());
            account_infos.extend(self.authority.to_account_infos());
            account_infos.extend(self.greeks.to_account_infos());
            account_infos.extend(self.oracle.to_account_infos());
            account_infos.extend(self.socialized_loss_account.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for Withdraw<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.state.to_account_metas(None));
            account_metas.extend(self.zeta_group.to_account_metas(None));
            account_metas.extend(self.vault.to_account_metas(None));
            account_metas.extend(self.margin_account.to_account_metas(None));
            account_metas.extend(self.user_token_account.to_account_metas(None));
            account_metas.extend(self.token_program.to_account_metas(None));
            account_metas.extend(self.authority.to_account_metas(None));
            account_metas.extend(self.greeks.to_account_metas(None));
            account_metas.extend(self.oracle.to_account_metas(None));
            account_metas.extend(self.socialized_loss_account.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for Withdraw<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
            anchor_lang::AccountsExit::exit(&self.vault, program_id)?;
            anchor_lang::AccountsExit::exit(&self.margin_account, program_id)?;
            anchor_lang::AccountsExit::exit(&self.user_token_account, program_id)?;
            anchor_lang::AccountsExit::exit(&self.greeks, program_id)?;
            anchor_lang::AccountsExit::exit(&self.socialized_loss_account, program_id)?;
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
    pub(crate) mod __client_accounts_withdraw {
        use super::*;
        use anchor_lang::prelude::borsh;
        pub struct Withdraw {
            pub state: anchor_lang::solana_program::pubkey::Pubkey,
            pub zeta_group: anchor_lang::solana_program::pubkey::Pubkey,
            pub vault: anchor_lang::solana_program::pubkey::Pubkey,
            pub margin_account: anchor_lang::solana_program::pubkey::Pubkey,
            pub user_token_account: anchor_lang::solana_program::pubkey::Pubkey,
            pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub authority: anchor_lang::solana_program::pubkey::Pubkey,
            pub greeks: anchor_lang::solana_program::pubkey::Pubkey,
            pub oracle: anchor_lang::solana_program::pubkey::Pubkey,
            pub socialized_loss_account: anchor_lang::solana_program::pubkey::Pubkey,
        }
        impl borsh::ser::BorshSerialize for Withdraw
        where
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
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
                borsh::BorshSerialize::serialize(&self.state, writer)?;
                borsh::BorshSerialize::serialize(&self.zeta_group, writer)?;
                borsh::BorshSerialize::serialize(&self.vault, writer)?;
                borsh::BorshSerialize::serialize(&self.margin_account, writer)?;
                borsh::BorshSerialize::serialize(&self.user_token_account, writer)?;
                borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                borsh::BorshSerialize::serialize(&self.authority, writer)?;
                borsh::BorshSerialize::serialize(&self.greeks, writer)?;
                borsh::BorshSerialize::serialize(&self.oracle, writer)?;
                borsh::BorshSerialize::serialize(&self.socialized_loss_account, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for Withdraw {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.state, false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.zeta_group,
                        false,
                    ),
                );
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.vault, false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.margin_account,
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.user_token_account,
                    false,
                ));
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.token_program,
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.authority,
                        true,
                    ),
                );
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.greeks,
                    false,
                ));
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.oracle,
                        false,
                    ),
                );
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.socialized_loss_account,
                    false,
                ));
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
    pub(crate) mod __cpi_client_accounts_withdraw {
        use super::*;
        pub struct Withdraw<'info> {
            pub state: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub zeta_group: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub margin_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub user_token_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub greeks: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub oracle: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub socialized_loss_account:
                anchor_lang::solana_program::account_info::AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for Withdraw<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.state),
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.zeta_group),
                        false,
                    ),
                );
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.vault),
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.margin_account),
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.user_token_account),
                    false,
                ));
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.token_program),
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.authority),
                        true,
                    ),
                );
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.greeks),
                    false,
                ));
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.oracle),
                        false,
                    ),
                );
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.socialized_loss_account),
                    false,
                ));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for Withdraw<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.state));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.zeta_group,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vault));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.margin_account,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.user_token_account,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.token_program,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.authority));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.greeks));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.oracle));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.socialized_loss_account,
                ));
                account_infos
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'info> ::core::clone::Clone for Withdraw<'info> {
        #[inline]
        fn clone(&self) -> Withdraw<'info> {
            match *self {
                Withdraw {
                    state: ref __self_0_0,
                    zeta_group: ref __self_0_1,
                    vault: ref __self_0_2,
                    margin_account: ref __self_0_3,
                    user_token_account: ref __self_0_4,
                    token_program: ref __self_0_5,
                    authority: ref __self_0_6,
                    greeks: ref __self_0_7,
                    oracle: ref __self_0_8,
                    socialized_loss_account: ref __self_0_9,
                } => Withdraw {
                    state: ::core::clone::Clone::clone(&(*__self_0_0)),
                    zeta_group: ::core::clone::Clone::clone(&(*__self_0_1)),
                    vault: ::core::clone::Clone::clone(&(*__self_0_2)),
                    margin_account: ::core::clone::Clone::clone(&(*__self_0_3)),
                    user_token_account: ::core::clone::Clone::clone(&(*__self_0_4)),
                    token_program: ::core::clone::Clone::clone(&(*__self_0_5)),
                    authority: ::core::clone::Clone::clone(&(*__self_0_6)),
                    greeks: ::core::clone::Clone::clone(&(*__self_0_7)),
                    oracle: ::core::clone::Clone::clone(&(*__self_0_8)),
                    socialized_loss_account: ::core::clone::Clone::clone(&(*__self_0_9)),
                },
            }
        }
    }
    pub struct InitializeOpenOrders<'info> {
        pub state: AccountInfo<'info>,
        pub zeta_group: AccountInfo<'info>,
        pub dex_program: AccountInfo<'info>,
        pub system_program: Program<'info, System>,
        #[account(mut)]
        pub open_orders: AccountInfo<'info>,
        #[account(mut)]
        pub margin_account: AccountInfo<'info>,
        #[account(mut)]
        pub authority: Signer<'info>,
        pub market: AccountInfo<'info>,
        pub serum_authority: AccountInfo<'info>,
        #[account(mut)]
        pub open_orders_map: AccountInfo<'info>,
        pub rent: Sysvar<'info, Rent>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info> for InitializeOpenOrders<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
            ix_data: &[u8],
        ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError>
        {
            let state: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let zeta_group: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let dex_program: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let system_program: anchor_lang::Program<System> =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let open_orders: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let margin_account: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let authority: Signer =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let market: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let serum_authority: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let open_orders_map: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let rent: Sysvar<Rent> =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            if !open_orders.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            if !margin_account.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            if !authority.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            if !open_orders_map.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            Ok(InitializeOpenOrders {
                state,
                zeta_group,
                dex_program,
                system_program,
                open_orders,
                margin_account,
                authority,
                market,
                serum_authority,
                open_orders_map,
                rent,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeOpenOrders<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.state.to_account_infos());
            account_infos.extend(self.zeta_group.to_account_infos());
            account_infos.extend(self.dex_program.to_account_infos());
            account_infos.extend(self.system_program.to_account_infos());
            account_infos.extend(self.open_orders.to_account_infos());
            account_infos.extend(self.margin_account.to_account_infos());
            account_infos.extend(self.authority.to_account_infos());
            account_infos.extend(self.market.to_account_infos());
            account_infos.extend(self.serum_authority.to_account_infos());
            account_infos.extend(self.open_orders_map.to_account_infos());
            account_infos.extend(self.rent.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for InitializeOpenOrders<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.state.to_account_metas(None));
            account_metas.extend(self.zeta_group.to_account_metas(None));
            account_metas.extend(self.dex_program.to_account_metas(None));
            account_metas.extend(self.system_program.to_account_metas(None));
            account_metas.extend(self.open_orders.to_account_metas(None));
            account_metas.extend(self.margin_account.to_account_metas(None));
            account_metas.extend(self.authority.to_account_metas(None));
            account_metas.extend(self.market.to_account_metas(None));
            account_metas.extend(self.serum_authority.to_account_metas(None));
            account_metas.extend(self.open_orders_map.to_account_metas(None));
            account_metas.extend(self.rent.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for InitializeOpenOrders<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
            anchor_lang::AccountsExit::exit(&self.open_orders, program_id)?;
            anchor_lang::AccountsExit::exit(&self.margin_account, program_id)?;
            anchor_lang::AccountsExit::exit(&self.authority, program_id)?;
            anchor_lang::AccountsExit::exit(&self.open_orders_map, program_id)?;
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
    pub(crate) mod __client_accounts_initialize_open_orders {
        use super::*;
        use anchor_lang::prelude::borsh;
        pub struct InitializeOpenOrders {
            pub state: anchor_lang::solana_program::pubkey::Pubkey,
            pub zeta_group: anchor_lang::solana_program::pubkey::Pubkey,
            pub dex_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub open_orders: anchor_lang::solana_program::pubkey::Pubkey,
            pub margin_account: anchor_lang::solana_program::pubkey::Pubkey,
            pub authority: anchor_lang::solana_program::pubkey::Pubkey,
            pub market: anchor_lang::solana_program::pubkey::Pubkey,
            pub serum_authority: anchor_lang::solana_program::pubkey::Pubkey,
            pub open_orders_map: anchor_lang::solana_program::pubkey::Pubkey,
            pub rent: anchor_lang::solana_program::pubkey::Pubkey,
        }
        impl borsh::ser::BorshSerialize for InitializeOpenOrders
        where
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
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
                borsh::BorshSerialize::serialize(&self.state, writer)?;
                borsh::BorshSerialize::serialize(&self.zeta_group, writer)?;
                borsh::BorshSerialize::serialize(&self.dex_program, writer)?;
                borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                borsh::BorshSerialize::serialize(&self.open_orders, writer)?;
                borsh::BorshSerialize::serialize(&self.margin_account, writer)?;
                borsh::BorshSerialize::serialize(&self.authority, writer)?;
                borsh::BorshSerialize::serialize(&self.market, writer)?;
                borsh::BorshSerialize::serialize(&self.serum_authority, writer)?;
                borsh::BorshSerialize::serialize(&self.open_orders_map, writer)?;
                borsh::BorshSerialize::serialize(&self.rent, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for InitializeOpenOrders {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.state, false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.zeta_group,
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.dex_program,
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.system_program,
                        false,
                    ),
                );
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.open_orders,
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.margin_account,
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.authority,
                    true,
                ));
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.market,
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.serum_authority,
                        false,
                    ),
                );
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.open_orders_map,
                    false,
                ));
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.rent, false,
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
    pub(crate) mod __cpi_client_accounts_initialize_open_orders {
        use super::*;
        pub struct InitializeOpenOrders<'info> {
            pub state: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub zeta_group: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub dex_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub system_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub open_orders: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub margin_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub market: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub serum_authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub open_orders_map: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for InitializeOpenOrders<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.state),
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.zeta_group),
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.dex_program),
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.system_program),
                        false,
                    ),
                );
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.open_orders),
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.margin_account),
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.authority),
                    true,
                ));
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.market),
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.serum_authority),
                        false,
                    ),
                );
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.open_orders_map),
                    false,
                ));
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.rent),
                        false,
                    ),
                );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeOpenOrders<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.state));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.zeta_group,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.dex_program,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.system_program,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.open_orders,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.margin_account,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.authority));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.market));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.serum_authority,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.open_orders_map,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.rent));
                account_infos
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'info> ::core::clone::Clone for InitializeOpenOrders<'info> {
        #[inline]
        fn clone(&self) -> InitializeOpenOrders<'info> {
            match *self {
                InitializeOpenOrders {
                    state: ref __self_0_0,
                    zeta_group: ref __self_0_1,
                    dex_program: ref __self_0_2,
                    system_program: ref __self_0_3,
                    open_orders: ref __self_0_4,
                    margin_account: ref __self_0_5,
                    authority: ref __self_0_6,
                    market: ref __self_0_7,
                    serum_authority: ref __self_0_8,
                    open_orders_map: ref __self_0_9,
                    rent: ref __self_0_10,
                } => InitializeOpenOrders {
                    state: ::core::clone::Clone::clone(&(*__self_0_0)),
                    zeta_group: ::core::clone::Clone::clone(&(*__self_0_1)),
                    dex_program: ::core::clone::Clone::clone(&(*__self_0_2)),
                    system_program: ::core::clone::Clone::clone(&(*__self_0_3)),
                    open_orders: ::core::clone::Clone::clone(&(*__self_0_4)),
                    margin_account: ::core::clone::Clone::clone(&(*__self_0_5)),
                    authority: ::core::clone::Clone::clone(&(*__self_0_6)),
                    market: ::core::clone::Clone::clone(&(*__self_0_7)),
                    serum_authority: ::core::clone::Clone::clone(&(*__self_0_8)),
                    open_orders_map: ::core::clone::Clone::clone(&(*__self_0_9)),
                    rent: ::core::clone::Clone::clone(&(*__self_0_10)),
                },
            }
        }
    }
    pub struct MarketAccounts<'info> {
        #[account(mut)]
        pub market: AccountInfo<'info>,
        #[account(mut)]
        pub request_queue: AccountInfo<'info>,
        #[account(mut)]
        pub event_queue: AccountInfo<'info>,
        #[account(mut)]
        pub bids: AccountInfo<'info>,
        #[account(mut)]
        pub asks: AccountInfo<'info>,
        #[account(mut)]
        pub order_payer_token_account: AccountInfo<'info>,
        #[account(mut)]
        pub coin_vault: AccountInfo<'info>,
        #[account(mut)]
        pub pc_vault: AccountInfo<'info>,
        #[account(mut)]
        pub coin_wallet: AccountInfo<'info>,
        #[account(mut)]
        pub pc_wallet: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info> for MarketAccounts<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
            ix_data: &[u8],
        ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError>
        {
            let market: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let request_queue: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let event_queue: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let bids: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let asks: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let order_payer_token_account: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let coin_vault: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let pc_vault: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let coin_wallet: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let pc_wallet: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            if !market.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            if !request_queue.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            if !event_queue.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            if !bids.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            if !asks.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            if !order_payer_token_account.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            if !coin_vault.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            if !pc_vault.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            if !coin_wallet.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            if !pc_wallet.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            Ok(MarketAccounts {
                market,
                request_queue,
                event_queue,
                bids,
                asks,
                order_payer_token_account,
                coin_vault,
                pc_vault,
                coin_wallet,
                pc_wallet,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for MarketAccounts<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.market.to_account_infos());
            account_infos.extend(self.request_queue.to_account_infos());
            account_infos.extend(self.event_queue.to_account_infos());
            account_infos.extend(self.bids.to_account_infos());
            account_infos.extend(self.asks.to_account_infos());
            account_infos.extend(self.order_payer_token_account.to_account_infos());
            account_infos.extend(self.coin_vault.to_account_infos());
            account_infos.extend(self.pc_vault.to_account_infos());
            account_infos.extend(self.coin_wallet.to_account_infos());
            account_infos.extend(self.pc_wallet.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for MarketAccounts<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.market.to_account_metas(None));
            account_metas.extend(self.request_queue.to_account_metas(None));
            account_metas.extend(self.event_queue.to_account_metas(None));
            account_metas.extend(self.bids.to_account_metas(None));
            account_metas.extend(self.asks.to_account_metas(None));
            account_metas.extend(self.order_payer_token_account.to_account_metas(None));
            account_metas.extend(self.coin_vault.to_account_metas(None));
            account_metas.extend(self.pc_vault.to_account_metas(None));
            account_metas.extend(self.coin_wallet.to_account_metas(None));
            account_metas.extend(self.pc_wallet.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for MarketAccounts<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
            anchor_lang::AccountsExit::exit(&self.market, program_id)?;
            anchor_lang::AccountsExit::exit(&self.request_queue, program_id)?;
            anchor_lang::AccountsExit::exit(&self.event_queue, program_id)?;
            anchor_lang::AccountsExit::exit(&self.bids, program_id)?;
            anchor_lang::AccountsExit::exit(&self.asks, program_id)?;
            anchor_lang::AccountsExit::exit(&self.order_payer_token_account, program_id)?;
            anchor_lang::AccountsExit::exit(&self.coin_vault, program_id)?;
            anchor_lang::AccountsExit::exit(&self.pc_vault, program_id)?;
            anchor_lang::AccountsExit::exit(&self.coin_wallet, program_id)?;
            anchor_lang::AccountsExit::exit(&self.pc_wallet, program_id)?;
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
    pub(crate) mod __client_accounts_market_accounts {
        use super::*;
        use anchor_lang::prelude::borsh;
        pub struct MarketAccounts {
            pub market: anchor_lang::solana_program::pubkey::Pubkey,
            pub request_queue: anchor_lang::solana_program::pubkey::Pubkey,
            pub event_queue: anchor_lang::solana_program::pubkey::Pubkey,
            pub bids: anchor_lang::solana_program::pubkey::Pubkey,
            pub asks: anchor_lang::solana_program::pubkey::Pubkey,
            pub order_payer_token_account: anchor_lang::solana_program::pubkey::Pubkey,
            pub coin_vault: anchor_lang::solana_program::pubkey::Pubkey,
            pub pc_vault: anchor_lang::solana_program::pubkey::Pubkey,
            pub coin_wallet: anchor_lang::solana_program::pubkey::Pubkey,
            pub pc_wallet: anchor_lang::solana_program::pubkey::Pubkey,
        }
        impl borsh::ser::BorshSerialize for MarketAccounts
        where
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
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
                borsh::BorshSerialize::serialize(&self.market, writer)?;
                borsh::BorshSerialize::serialize(&self.request_queue, writer)?;
                borsh::BorshSerialize::serialize(&self.event_queue, writer)?;
                borsh::BorshSerialize::serialize(&self.bids, writer)?;
                borsh::BorshSerialize::serialize(&self.asks, writer)?;
                borsh::BorshSerialize::serialize(&self.order_payer_token_account, writer)?;
                borsh::BorshSerialize::serialize(&self.coin_vault, writer)?;
                borsh::BorshSerialize::serialize(&self.pc_vault, writer)?;
                borsh::BorshSerialize::serialize(&self.coin_wallet, writer)?;
                borsh::BorshSerialize::serialize(&self.pc_wallet, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for MarketAccounts {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.market,
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.request_queue,
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.event_queue,
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.bids, false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.asks, false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.order_payer_token_account,
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.coin_vault,
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.pc_vault,
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.coin_wallet,
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.pc_wallet,
                    false,
                ));
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
    pub(crate) mod __cpi_client_accounts_market_accounts {
        use super::*;
        pub struct MarketAccounts<'info> {
            pub market: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub request_queue: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub event_queue: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub bids: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub asks: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub order_payer_token_account:
                anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub coin_vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub pc_vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub coin_wallet: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub pc_wallet: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for MarketAccounts<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.market),
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.request_queue),
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.event_queue),
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.bids),
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.asks),
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.order_payer_token_account),
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.coin_vault),
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.pc_vault),
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.coin_wallet),
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.pc_wallet),
                    false,
                ));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for MarketAccounts<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.market));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.request_queue,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.event_queue,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.bids));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.asks));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.order_payer_token_account,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.coin_vault,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.pc_vault));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.coin_wallet,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.pc_wallet));
                account_infos
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'info> ::core::clone::Clone for MarketAccounts<'info> {
        #[inline]
        fn clone(&self) -> MarketAccounts<'info> {
            match *self {
                MarketAccounts {
                    market: ref __self_0_0,
                    request_queue: ref __self_0_1,
                    event_queue: ref __self_0_2,
                    bids: ref __self_0_3,
                    asks: ref __self_0_4,
                    order_payer_token_account: ref __self_0_5,
                    coin_vault: ref __self_0_6,
                    pc_vault: ref __self_0_7,
                    coin_wallet: ref __self_0_8,
                    pc_wallet: ref __self_0_9,
                } => MarketAccounts {
                    market: ::core::clone::Clone::clone(&(*__self_0_0)),
                    request_queue: ::core::clone::Clone::clone(&(*__self_0_1)),
                    event_queue: ::core::clone::Clone::clone(&(*__self_0_2)),
                    bids: ::core::clone::Clone::clone(&(*__self_0_3)),
                    asks: ::core::clone::Clone::clone(&(*__self_0_4)),
                    order_payer_token_account: ::core::clone::Clone::clone(&(*__self_0_5)),
                    coin_vault: ::core::clone::Clone::clone(&(*__self_0_6)),
                    pc_vault: ::core::clone::Clone::clone(&(*__self_0_7)),
                    coin_wallet: ::core::clone::Clone::clone(&(*__self_0_8)),
                    pc_wallet: ::core::clone::Clone::clone(&(*__self_0_9)),
                },
            }
        }
    }
    pub struct PlaceOrder<'info> {
        pub state: AccountInfo<'info>,
        pub zeta_group: AccountInfo<'info>,
        #[account(mut)]
        pub margin_account: AccountInfo<'info>,
        pub authority: Signer<'info>,
        pub dex_program: AccountInfo<'info>,
        pub token_program: Program<'info, Token>,
        pub serum_authority: AccountInfo<'info>,
        #[account(mut)]
        pub greeks: AccountInfo<'info>,
        #[account(mut)]
        pub open_orders: AccountInfo<'info>,
        pub rent: Sysvar<'info, Rent>,
        pub market_accounts: MarketAccounts<'info>,
        pub oracle: AccountInfo<'info>,
        #[account(mut)]
        pub market_node: AccountInfo<'info>,
        #[account(mut)]
        pub market_mint: AccountInfo<'info>,
        pub mint_authority: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info> for PlaceOrder<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
            ix_data: &[u8],
        ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError>
        {
            let state: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let zeta_group: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let margin_account: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let authority: Signer =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let dex_program: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let token_program: anchor_lang::Program<Token> =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let serum_authority: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let greeks: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let open_orders: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let rent: Sysvar<Rent> =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let market_accounts: MarketAccounts<'info> =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let oracle: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let market_node: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let market_mint: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let mint_authority: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            if !margin_account.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            if !greeks.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            if !open_orders.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            if !market_node.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            if !market_mint.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            Ok(PlaceOrder {
                state,
                zeta_group,
                margin_account,
                authority,
                dex_program,
                token_program,
                serum_authority,
                greeks,
                open_orders,
                rent,
                market_accounts,
                oracle,
                market_node,
                market_mint,
                mint_authority,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for PlaceOrder<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.state.to_account_infos());
            account_infos.extend(self.zeta_group.to_account_infos());
            account_infos.extend(self.margin_account.to_account_infos());
            account_infos.extend(self.authority.to_account_infos());
            account_infos.extend(self.dex_program.to_account_infos());
            account_infos.extend(self.token_program.to_account_infos());
            account_infos.extend(self.serum_authority.to_account_infos());
            account_infos.extend(self.greeks.to_account_infos());
            account_infos.extend(self.open_orders.to_account_infos());
            account_infos.extend(self.rent.to_account_infos());
            account_infos.extend(self.market_accounts.to_account_infos());
            account_infos.extend(self.oracle.to_account_infos());
            account_infos.extend(self.market_node.to_account_infos());
            account_infos.extend(self.market_mint.to_account_infos());
            account_infos.extend(self.mint_authority.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for PlaceOrder<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.state.to_account_metas(None));
            account_metas.extend(self.zeta_group.to_account_metas(None));
            account_metas.extend(self.margin_account.to_account_metas(None));
            account_metas.extend(self.authority.to_account_metas(None));
            account_metas.extend(self.dex_program.to_account_metas(None));
            account_metas.extend(self.token_program.to_account_metas(None));
            account_metas.extend(self.serum_authority.to_account_metas(None));
            account_metas.extend(self.greeks.to_account_metas(None));
            account_metas.extend(self.open_orders.to_account_metas(None));
            account_metas.extend(self.rent.to_account_metas(None));
            account_metas.extend(self.market_accounts.to_account_metas(None));
            account_metas.extend(self.oracle.to_account_metas(None));
            account_metas.extend(self.market_node.to_account_metas(None));
            account_metas.extend(self.market_mint.to_account_metas(None));
            account_metas.extend(self.mint_authority.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for PlaceOrder<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
            anchor_lang::AccountsExit::exit(&self.margin_account, program_id)?;
            anchor_lang::AccountsExit::exit(&self.greeks, program_id)?;
            anchor_lang::AccountsExit::exit(&self.open_orders, program_id)?;
            anchor_lang::AccountsExit::exit(&self.market_accounts, program_id)?;
            anchor_lang::AccountsExit::exit(&self.market_node, program_id)?;
            anchor_lang::AccountsExit::exit(&self.market_mint, program_id)?;
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
    pub(crate) mod __client_accounts_place_order {
        use super::*;
        use anchor_lang::prelude::borsh;
        pub use __client_accounts_market_accounts::MarketAccounts;
        pub struct PlaceOrder {
            pub state: anchor_lang::solana_program::pubkey::Pubkey,
            pub zeta_group: anchor_lang::solana_program::pubkey::Pubkey,
            pub margin_account: anchor_lang::solana_program::pubkey::Pubkey,
            pub authority: anchor_lang::solana_program::pubkey::Pubkey,
            pub dex_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub serum_authority: anchor_lang::solana_program::pubkey::Pubkey,
            pub greeks: anchor_lang::solana_program::pubkey::Pubkey,
            pub open_orders: anchor_lang::solana_program::pubkey::Pubkey,
            pub rent: anchor_lang::solana_program::pubkey::Pubkey,
            pub market_accounts: __client_accounts_market_accounts::MarketAccounts,
            pub oracle: anchor_lang::solana_program::pubkey::Pubkey,
            pub market_node: anchor_lang::solana_program::pubkey::Pubkey,
            pub market_mint: anchor_lang::solana_program::pubkey::Pubkey,
            pub mint_authority: anchor_lang::solana_program::pubkey::Pubkey,
        }
        impl borsh::ser::BorshSerialize for PlaceOrder
        where
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            __client_accounts_market_accounts::MarketAccounts: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.state, writer)?;
                borsh::BorshSerialize::serialize(&self.zeta_group, writer)?;
                borsh::BorshSerialize::serialize(&self.margin_account, writer)?;
                borsh::BorshSerialize::serialize(&self.authority, writer)?;
                borsh::BorshSerialize::serialize(&self.dex_program, writer)?;
                borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                borsh::BorshSerialize::serialize(&self.serum_authority, writer)?;
                borsh::BorshSerialize::serialize(&self.greeks, writer)?;
                borsh::BorshSerialize::serialize(&self.open_orders, writer)?;
                borsh::BorshSerialize::serialize(&self.rent, writer)?;
                borsh::BorshSerialize::serialize(&self.market_accounts, writer)?;
                borsh::BorshSerialize::serialize(&self.oracle, writer)?;
                borsh::BorshSerialize::serialize(&self.market_node, writer)?;
                borsh::BorshSerialize::serialize(&self.market_mint, writer)?;
                borsh::BorshSerialize::serialize(&self.mint_authority, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for PlaceOrder {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.state, false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.zeta_group,
                        false,
                    ),
                );
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.margin_account,
                    false,
                ));
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.authority,
                        true,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.dex_program,
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.token_program,
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.serum_authority,
                        false,
                    ),
                );
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.greeks,
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.open_orders,
                    false,
                ));
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.rent, false,
                    ),
                );
                account_metas.extend(self.market_accounts.to_account_metas(None));
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.oracle,
                        false,
                    ),
                );
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.market_node,
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.market_mint,
                    false,
                ));
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.mint_authority,
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
    pub(crate) mod __cpi_client_accounts_place_order {
        use super::*;
        pub use __cpi_client_accounts_market_accounts::MarketAccounts;
        pub struct PlaceOrder<'info> {
            pub state: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub zeta_group: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub margin_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub dex_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub serum_authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub greeks: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub open_orders: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub market_accounts: __cpi_client_accounts_market_accounts::MarketAccounts<'info>,
            pub oracle: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub market_node: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub market_mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub mint_authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for PlaceOrder<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.state),
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.zeta_group),
                        false,
                    ),
                );
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.margin_account),
                    false,
                ));
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.authority),
                        true,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.dex_program),
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.token_program),
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.serum_authority),
                        false,
                    ),
                );
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.greeks),
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.open_orders),
                    false,
                ));
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.rent),
                        false,
                    ),
                );
                account_metas.extend(self.market_accounts.to_account_metas(None));
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.oracle),
                        false,
                    ),
                );
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.market_node),
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.market_mint),
                    false,
                ));
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.mint_authority),
                        false,
                    ),
                );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for PlaceOrder<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.state));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.zeta_group,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.margin_account,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.authority));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.dex_program,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.token_program,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.serum_authority,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.greeks));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.open_orders,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.rent));
                account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(
                    &self.market_accounts,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.oracle));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.market_node,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.market_mint,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.mint_authority,
                ));
                account_infos
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'info> ::core::clone::Clone for PlaceOrder<'info> {
        #[inline]
        fn clone(&self) -> PlaceOrder<'info> {
            match *self {
                PlaceOrder {
                    state: ref __self_0_0,
                    zeta_group: ref __self_0_1,
                    margin_account: ref __self_0_2,
                    authority: ref __self_0_3,
                    dex_program: ref __self_0_4,
                    token_program: ref __self_0_5,
                    serum_authority: ref __self_0_6,
                    greeks: ref __self_0_7,
                    open_orders: ref __self_0_8,
                    rent: ref __self_0_9,
                    market_accounts: ref __self_0_10,
                    oracle: ref __self_0_11,
                    market_node: ref __self_0_12,
                    market_mint: ref __self_0_13,
                    mint_authority: ref __self_0_14,
                } => PlaceOrder {
                    state: ::core::clone::Clone::clone(&(*__self_0_0)),
                    zeta_group: ::core::clone::Clone::clone(&(*__self_0_1)),
                    margin_account: ::core::clone::Clone::clone(&(*__self_0_2)),
                    authority: ::core::clone::Clone::clone(&(*__self_0_3)),
                    dex_program: ::core::clone::Clone::clone(&(*__self_0_4)),
                    token_program: ::core::clone::Clone::clone(&(*__self_0_5)),
                    serum_authority: ::core::clone::Clone::clone(&(*__self_0_6)),
                    greeks: ::core::clone::Clone::clone(&(*__self_0_7)),
                    open_orders: ::core::clone::Clone::clone(&(*__self_0_8)),
                    rent: ::core::clone::Clone::clone(&(*__self_0_9)),
                    market_accounts: ::core::clone::Clone::clone(&(*__self_0_10)),
                    oracle: ::core::clone::Clone::clone(&(*__self_0_11)),
                    market_node: ::core::clone::Clone::clone(&(*__self_0_12)),
                    market_mint: ::core::clone::Clone::clone(&(*__self_0_13)),
                    mint_authority: ::core::clone::Clone::clone(&(*__self_0_14)),
                },
            }
        }
    }
    pub struct CancelAccounts<'info> {
        pub zeta_group: AccountInfo<'info>,
        pub state: AccountInfo<'info>,
        #[account(mut)]
        pub margin_account: AccountInfo<'info>,
        pub dex_program: AccountInfo<'info>,
        pub serum_authority: AccountInfo<'info>,
        #[account(mut)]
        pub open_orders: AccountInfo<'info>,
        #[account(mut)]
        pub market: AccountInfo<'info>,
        #[account(mut)]
        pub bids: AccountInfo<'info>,
        #[account(mut)]
        pub asks: AccountInfo<'info>,
        #[account(mut)]
        pub event_queue: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info> for CancelAccounts<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
            ix_data: &[u8],
        ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError>
        {
            let zeta_group: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let state: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let margin_account: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let dex_program: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let serum_authority: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let open_orders: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let market: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let bids: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let asks: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let event_queue: AccountInfo =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            if !margin_account.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            if !open_orders.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            if !market.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            if !bids.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            if !asks.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            if !event_queue.to_account_info().is_writable {
                return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
            }
            Ok(CancelAccounts {
                zeta_group,
                state,
                margin_account,
                dex_program,
                serum_authority,
                open_orders,
                market,
                bids,
                asks,
                event_queue,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for CancelAccounts<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.zeta_group.to_account_infos());
            account_infos.extend(self.state.to_account_infos());
            account_infos.extend(self.margin_account.to_account_infos());
            account_infos.extend(self.dex_program.to_account_infos());
            account_infos.extend(self.serum_authority.to_account_infos());
            account_infos.extend(self.open_orders.to_account_infos());
            account_infos.extend(self.market.to_account_infos());
            account_infos.extend(self.bids.to_account_infos());
            account_infos.extend(self.asks.to_account_infos());
            account_infos.extend(self.event_queue.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for CancelAccounts<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.zeta_group.to_account_metas(None));
            account_metas.extend(self.state.to_account_metas(None));
            account_metas.extend(self.margin_account.to_account_metas(None));
            account_metas.extend(self.dex_program.to_account_metas(None));
            account_metas.extend(self.serum_authority.to_account_metas(None));
            account_metas.extend(self.open_orders.to_account_metas(None));
            account_metas.extend(self.market.to_account_metas(None));
            account_metas.extend(self.bids.to_account_metas(None));
            account_metas.extend(self.asks.to_account_metas(None));
            account_metas.extend(self.event_queue.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for CancelAccounts<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
            anchor_lang::AccountsExit::exit(&self.margin_account, program_id)?;
            anchor_lang::AccountsExit::exit(&self.open_orders, program_id)?;
            anchor_lang::AccountsExit::exit(&self.market, program_id)?;
            anchor_lang::AccountsExit::exit(&self.bids, program_id)?;
            anchor_lang::AccountsExit::exit(&self.asks, program_id)?;
            anchor_lang::AccountsExit::exit(&self.event_queue, program_id)?;
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
    pub(crate) mod __client_accounts_cancel_accounts {
        use super::*;
        use anchor_lang::prelude::borsh;
        pub struct CancelAccounts {
            pub zeta_group: anchor_lang::solana_program::pubkey::Pubkey,
            pub state: anchor_lang::solana_program::pubkey::Pubkey,
            pub margin_account: anchor_lang::solana_program::pubkey::Pubkey,
            pub dex_program: anchor_lang::solana_program::pubkey::Pubkey,
            pub serum_authority: anchor_lang::solana_program::pubkey::Pubkey,
            pub open_orders: anchor_lang::solana_program::pubkey::Pubkey,
            pub market: anchor_lang::solana_program::pubkey::Pubkey,
            pub bids: anchor_lang::solana_program::pubkey::Pubkey,
            pub asks: anchor_lang::solana_program::pubkey::Pubkey,
            pub event_queue: anchor_lang::solana_program::pubkey::Pubkey,
        }
        impl borsh::ser::BorshSerialize for CancelAccounts
        where
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
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
                borsh::BorshSerialize::serialize(&self.zeta_group, writer)?;
                borsh::BorshSerialize::serialize(&self.state, writer)?;
                borsh::BorshSerialize::serialize(&self.margin_account, writer)?;
                borsh::BorshSerialize::serialize(&self.dex_program, writer)?;
                borsh::BorshSerialize::serialize(&self.serum_authority, writer)?;
                borsh::BorshSerialize::serialize(&self.open_orders, writer)?;
                borsh::BorshSerialize::serialize(&self.market, writer)?;
                borsh::BorshSerialize::serialize(&self.bids, writer)?;
                borsh::BorshSerialize::serialize(&self.asks, writer)?;
                borsh::BorshSerialize::serialize(&self.event_queue, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for CancelAccounts {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.zeta_group,
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.state, false,
                    ),
                );
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.margin_account,
                    false,
                ));
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.dex_program,
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.serum_authority,
                        false,
                    ),
                );
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.open_orders,
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.market,
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.bids, false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.asks, false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    self.event_queue,
                    false,
                ));
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
    pub(crate) mod __cpi_client_accounts_cancel_accounts {
        use super::*;
        pub struct CancelAccounts<'info> {
            pub zeta_group: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub state: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub margin_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub dex_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub serum_authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub open_orders: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub market: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub bids: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub asks: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub event_queue: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for CancelAccounts<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.zeta_group),
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.state),
                        false,
                    ),
                );
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.margin_account),
                    false,
                ));
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.dex_program),
                        false,
                    ),
                );
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.serum_authority),
                        false,
                    ),
                );
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.open_orders),
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.market),
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.bids),
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.asks),
                    false,
                ));
                account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                    anchor_lang::Key::key(&self.event_queue),
                    false,
                ));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for CancelAccounts<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.zeta_group,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.state));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.margin_account,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.dex_program,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.serum_authority,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.open_orders,
                ));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.market));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.bids));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.asks));
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                    &self.event_queue,
                ));
                account_infos
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'info> ::core::clone::Clone for CancelAccounts<'info> {
        #[inline]
        fn clone(&self) -> CancelAccounts<'info> {
            match *self {
                CancelAccounts {
                    zeta_group: ref __self_0_0,
                    state: ref __self_0_1,
                    margin_account: ref __self_0_2,
                    dex_program: ref __self_0_3,
                    serum_authority: ref __self_0_4,
                    open_orders: ref __self_0_5,
                    market: ref __self_0_6,
                    bids: ref __self_0_7,
                    asks: ref __self_0_8,
                    event_queue: ref __self_0_9,
                } => CancelAccounts {
                    zeta_group: ::core::clone::Clone::clone(&(*__self_0_0)),
                    state: ::core::clone::Clone::clone(&(*__self_0_1)),
                    margin_account: ::core::clone::Clone::clone(&(*__self_0_2)),
                    dex_program: ::core::clone::Clone::clone(&(*__self_0_3)),
                    serum_authority: ::core::clone::Clone::clone(&(*__self_0_4)),
                    open_orders: ::core::clone::Clone::clone(&(*__self_0_5)),
                    market: ::core::clone::Clone::clone(&(*__self_0_6)),
                    bids: ::core::clone::Clone::clone(&(*__self_0_7)),
                    asks: ::core::clone::Clone::clone(&(*__self_0_8)),
                    event_queue: ::core::clone::Clone::clone(&(*__self_0_9)),
                },
            }
        }
    }
    pub struct CancelOrder<'info> {
        pub authority: Signer<'info>,
        pub cancel_accounts: CancelAccounts<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info> for CancelOrder<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
            ix_data: &[u8],
        ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError>
        {
            let authority: Signer =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            let cancel_accounts: CancelAccounts<'info> =
                anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
            Ok(CancelOrder {
                authority,
                cancel_accounts,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for CancelOrder<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.authority.to_account_infos());
            account_infos.extend(self.cancel_accounts.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for CancelOrder<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.authority.to_account_metas(None));
            account_metas.extend(self.cancel_accounts.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for CancelOrder<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
            anchor_lang::AccountsExit::exit(&self.cancel_accounts, program_id)?;
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
    pub(crate) mod __client_accounts_cancel_order {
        use super::*;
        use anchor_lang::prelude::borsh;
        pub use __client_accounts_cancel_accounts::CancelAccounts;
        pub struct CancelOrder {
            pub authority: anchor_lang::solana_program::pubkey::Pubkey,
            pub cancel_accounts: __client_accounts_cancel_accounts::CancelAccounts,
        }
        impl borsh::ser::BorshSerialize for CancelOrder
        where
            anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            __client_accounts_cancel_accounts::CancelAccounts: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.authority, writer)?;
                borsh::BorshSerialize::serialize(&self.cancel_accounts, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for CancelOrder {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.authority,
                        true,
                    ),
                );
                account_metas.extend(self.cancel_accounts.to_account_metas(None));
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
    pub(crate) mod __cpi_client_accounts_cancel_order {
        use super::*;
        pub use __cpi_client_accounts_cancel_accounts::CancelAccounts;
        pub struct CancelOrder<'info> {
            pub authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub cancel_accounts: __cpi_client_accounts_cancel_accounts::CancelAccounts<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for CancelOrder<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.authority),
                        true,
                    ),
                );
                account_metas.extend(self.cancel_accounts.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for CancelOrder<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.authority));
                account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(
                    &self.cancel_accounts,
                ));
                account_infos
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'info> ::core::clone::Clone for CancelOrder<'info> {
        #[inline]
        fn clone(&self) -> CancelOrder<'info> {
            match *self {
                CancelOrder {
                    authority: ref __self_0_0,
                    cancel_accounts: ref __self_0_1,
                } => CancelOrder {
                    authority: ::core::clone::Clone::clone(&(*__self_0_0)),
                    cancel_accounts: ::core::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
}
pub mod zeta_utils {
    use crate::*;
    use std::cell::RefMut;
    use std::convert::{TryFrom, TryInto};
    use std::ops::DerefMut;
    pub fn deserialize_account_info_zerocopy<'a, T: bytemuck::Pod>(
        account_info: &'a AccountInfo,
    ) -> Result<RefMut<'a, T>> {
        let data = account_info.try_borrow_mut_data()?;
        Ok(RefMut::map(data, |data| {
            bytemuck::from_bytes_mut(&mut data.deref_mut()[8..])
        }))
    }
    #[inline(never)]
    pub fn deserialize_account_info<
        'a,
        T: AccountSerialize + AccountDeserialize + Owner + Clone,
    >(
        account_info: &AccountInfo<'a>,
    ) -> Result<T> {
        let mut data: &[u8] = &account_info.try_borrow_data()?;
        Ok(T::try_deserialize_unchecked(&mut data)?)
    }
    pub fn get_otm_amount(spot: u64, strike: u64, product: Kind) -> Result<u64> {
        match product {
            Kind::Call => Ok((strike as i128)
                .checked_sub(spot as i128)
                .unwrap()
                .max(0)
                .try_into()
                .unwrap()),
            Kind::Put => Ok((spot as i128)
                .checked_sub(strike as i128)
                .unwrap()
                .max(0)
                .try_into()
                .unwrap()),
            _ => {
                return {
                    ::solana_program::log::sol_log(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Error thrown at ", ":"],
                            &match (&"programs/zeta-cpi/src/zeta_utils.rs", &45u32) {
                                args => [
                                    ::core::fmt::ArgumentV1::new_display(args.0),
                                    ::core::fmt::ArgumentV1::new_display(args.1),
                                ],
                            },
                        ));
                        res
                    });
                    Err(ErrorCode::UnsupportedKind.into())
                }
            }
        }
    }
    /// Initial margin for single product
    pub fn get_initial_margin_per_lot(
        spot: u64,
        strike: u64,
        mark: u64,
        product: Kind,
        side: Side,
        margin_parameters: &MarginParameters,
    ) -> Result<u64> {
        let initial_margin: u128 = match product {
            Kind::Future => (spot as u128)
                .checked_mul(margin_parameters.future_margin_initial.into())
                .unwrap()
                .checked_div(NATIVE_PRECISION_DENOMINATOR)
                .unwrap(),
            Kind::Call | Kind::Put => match side {
                Side::Bid => (spot as u128)
                    .checked_mul(margin_parameters.option_spot_percentage_long_initial.into())
                    .unwrap()
                    .checked_div(NATIVE_PRECISION_DENOMINATOR)
                    .unwrap()
                    .min(
                        (mark as u128)
                            .checked_mul(
                                margin_parameters.option_mark_percentage_long_initial.into(),
                            )
                            .unwrap()
                            .checked_div(NATIVE_PRECISION_DENOMINATOR)
                            .unwrap(),
                    ),
                Side::Ask => {
                    let otm_amount: u128 = get_otm_amount(spot, strike, product)?.into();
                    let otm_pct = otm_amount
                        .checked_mul(NATIVE_PRECISION_DENOMINATOR)
                        .unwrap()
                        .checked_div(spot.into())
                        .unwrap();
                    let dynamic_margin_pct =
                        (margin_parameters.option_dynamic_percentage_short_initial as u128)
                            .checked_sub(otm_pct)
                            .unwrap_or(0);
                    let margin_pct = dynamic_margin_pct.max(
                        margin_parameters
                            .option_spot_percentage_short_initial
                            .into(),
                    );
                    margin_pct
                        .checked_mul(spot.into())
                        .unwrap()
                        .checked_div(NATIVE_PRECISION_DENOMINATOR)
                        .unwrap()
                }
                Side::Uninitialized => {
                    ::core::panicking::panic("internal error: entered unreachable code")
                }
            },
            _ => {
                return {
                    ::solana_program::log::sol_log(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Error thrown at ", ":"],
                            &match (&"programs/zeta-cpi/src/zeta_utils.rs", &103u32) {
                                args => [
                                    ::core::fmt::ArgumentV1::new_display(args.0),
                                    ::core::fmt::ArgumentV1::new_display(args.1),
                                ],
                            },
                        ));
                        res
                    });
                    Err(ErrorCode::UnsupportedKind.into())
                }
            }
        };
        if product == Kind::Put && side == Side::Ask {
            let sell_put_cap_margin = (strike as u128)
                .checked_mul(margin_parameters.option_short_put_cap_percentage as u128)
                .unwrap()
                .checked_div(NATIVE_PRECISION_DENOMINATOR)
                .unwrap();
            return Ok(u64::try_from(initial_margin.min(sell_put_cap_margin)).unwrap());
        }
        Ok(u64::try_from(initial_margin).unwrap())
    }
    /// Maintenance margin for single product
    pub fn get_maintenance_margin_per_lot(
        spot: u64,
        strike: u64,
        mark: u64,
        product: Kind,
        long: bool,
        margin_parameters: &MarginParameters,
    ) -> Result<u64> {
        let maintenance_margin: u128 = match product {
            Kind::Future => (spot as u128)
                .checked_mul(margin_parameters.future_margin_maintenance.into())
                .unwrap()
                .checked_div(NATIVE_PRECISION_DENOMINATOR)
                .unwrap(),
            Kind::Call | Kind::Put => {
                if long {
                    (spot as u128)
                        .checked_mul(
                            margin_parameters
                                .option_spot_percentage_long_maintenance
                                .into(),
                        )
                        .unwrap()
                        .checked_div(NATIVE_PRECISION_DENOMINATOR)
                        .unwrap()
                        .min(
                            (mark as u128)
                                .checked_mul(
                                    margin_parameters
                                        .option_mark_percentage_long_maintenance
                                        .into(),
                                )
                                .unwrap()
                                .checked_div(NATIVE_PRECISION_DENOMINATOR)
                                .unwrap(),
                        )
                } else {
                    let otm_amount: u128 = get_otm_amount(spot, strike, product)?.into();
                    let otm_pct = otm_amount
                        .checked_mul(NATIVE_PRECISION_DENOMINATOR)
                        .unwrap()
                        .checked_div(spot.into())
                        .unwrap();
                    let dynamic_margin_pct: u128 =
                        (margin_parameters.option_dynamic_percentage_short_maintenance as u128)
                            .checked_sub(otm_pct)
                            .unwrap_or(0);
                    let margin_pct = dynamic_margin_pct.max(
                        margin_parameters
                            .option_spot_percentage_short_maintenance
                            .into(),
                    );
                    margin_pct
                        .checked_mul(spot.into())
                        .unwrap()
                        .checked_div(NATIVE_PRECISION_DENOMINATOR)
                        .unwrap()
                }
            }
            _ => {
                return {
                    ::solana_program::log::sol_log(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Error thrown at ", ":"],
                            &match (&"programs/zeta-cpi/src/zeta_utils.rs", &181u32) {
                                args => [
                                    ::core::fmt::ArgumentV1::new_display(args.0),
                                    ::core::fmt::ArgumentV1::new_display(args.1),
                                ],
                            },
                        ));
                        res
                    });
                    Err(ErrorCode::UnsupportedKind.into())
                }
            }
        };
        if product == Kind::Put && !long {
            let sell_put_cap_margin = (strike as u128)
                .checked_mul(margin_parameters.option_short_put_cap_percentage as u128)
                .unwrap()
                .checked_div(NATIVE_PRECISION_DENOMINATOR)
                .unwrap();
            return Ok(u64::try_from(maintenance_margin.min(sell_put_cap_margin)).unwrap());
        }
        Ok(u64::try_from(maintenance_margin).unwrap())
    }
    /// Returns the native oracle price (6.dp)
    ///
    /// # Arguments
    ///
    /// * `oracle` - Oracle account.
    pub fn get_native_oracle_price(oracle: &AccountInfo) -> u64 {
        let oracle_price = pyth_client::Price::load(&oracle).unwrap();
        (oracle_price.agg.price as u128)
            .checked_mul(10u128.pow(PLATFORM_PRECISION.into()))
            .unwrap()
            .checked_div(10u128.pow((-oracle_price.expo).try_into().unwrap()))
            .unwrap()
            .try_into()
            .unwrap()
    }
    pub fn get_oracle_price(oracle: &AccountInfo, precision: u32) -> i128 {
        let oracle_price = pyth_client::Price::load(&oracle).unwrap();
        (oracle_price.agg.price as u128)
            .checked_mul(10u128.pow(precision))
            .unwrap()
            .checked_div(10u128.pow((-oracle_price.expo).try_into().unwrap()))
            .unwrap()
            .try_into()
            .unwrap()
    }
    /// Returns the market index given an expiry index and index into the slice.
    ///
    /// # Arguments
    ///
    /// * `expiry_index` - Expiry series index.
    /// * `product_index` - Index into the products slice. [0..NUM_PRODUCTS_PER_SERIES).
    pub fn get_products_slice_market_index(expiry_index: usize, product_index: usize) -> usize {
        expiry_index
            .checked_mul(NUM_PRODUCTS_PER_SERIES)
            .unwrap()
            .checked_add(product_index)
            .unwrap()
    }
}
use crate::context::*;
use crate::zeta_account::*;
use crate::zeta_constants::*;
use crate::zeta_utils::*;
/// The static program ID
pub static ID: anchor_lang::solana_program::pubkey::Pubkey =
    anchor_lang::solana_program::pubkey::Pubkey::new_from_array([
        91u8, 171u8, 103u8, 86u8, 186u8, 217u8, 100u8, 0u8, 129u8, 247u8, 155u8, 212u8, 13u8,
        196u8, 186u8, 113u8, 5u8, 155u8, 203u8, 171u8, 213u8, 60u8, 194u8, 116u8, 184u8, 192u8,
        57u8, 193u8, 222u8, 118u8, 125u8, 233u8,
    ]);
/// Confirms that a given pubkey is equivalent to the program ID
pub fn check_id(id: &anchor_lang::solana_program::pubkey::Pubkey) -> bool {
    id == &ID
}
/// Returns the program ID
pub fn id() -> anchor_lang::solana_program::pubkey::Pubkey {
    ID
}
use zeta_cpi::*;
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
    pub struct ZetaCpi;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for ZetaCpi {
        #[inline]
        fn clone(&self) -> ZetaCpi {
            match *self {
                ZetaCpi => ZetaCpi,
            }
        }
    }
    impl anchor_lang::AccountDeserialize for ZetaCpi {
        fn try_deserialize(
            buf: &mut &[u8],
        ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError>
        {
            ZetaCpi::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(
            _buf: &mut &[u8],
        ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError>
        {
            Ok(ZetaCpi)
        }
    }
    impl anchor_lang::Id for ZetaCpi {
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
        [67, 235, 66, 102, 167, 171, 120, 197] => {
            __private::__global::initialize_margin_account(program_id, accounts, ix_data)
        }
        [242, 35, 198, 137, 82, 225, 242, 182] => {
            __private::__global::deposit(program_id, accounts, ix_data)
        }
        [183, 18, 70, 156, 148, 109, 161, 34] => {
            __private::__global::withdraw(program_id, accounts, ix_data)
        }
        [55, 234, 16, 82, 100, 42, 126, 192] => {
            __private::__global::initialize_open_orders(program_id, accounts, ix_data)
        }
        [51, 194, 155, 175, 109, 130, 96, 106] => {
            __private::__global::place_order(program_id, accounts, ix_data)
        }
        [95, 129, 237, 240, 8, 49, 223, 132] => {
            __private::__global::cancel_order(program_id, accounts, ix_data)
        }
        [245, 2, 171, 103, 213, 126, 76, 207] => {
            __private::__global::read_program_data(program_id, accounts, ix_data)
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
        pub fn initialize_margin_account(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::InitializeMarginAccount::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::InitializeMarginAccount = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = InitializeMarginAccountCaller::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
            )?;
            zeta_cpi::initialize_margin_account(Context::new(
                program_id,
                &mut accounts,
                remaining_accounts,
            ))?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn deposit(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::Deposit::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::Deposit { amount } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                DepositCaller::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            zeta_cpi::deposit(
                Context::new(program_id, &mut accounts, remaining_accounts),
                amount,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn withdraw(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::Withdraw::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::Withdraw { amount } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                WithdrawCaller::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            zeta_cpi::withdraw(
                Context::new(program_id, &mut accounts, remaining_accounts),
                amount,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn initialize_open_orders(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::InitializeOpenOrders::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::InitializeOpenOrders = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = InitializeOpenOrdersCaller::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
            )?;
            zeta_cpi::initialize_open_orders(Context::new(
                program_id,
                &mut accounts,
                remaining_accounts,
            ))?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn place_order(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::PlaceOrder::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::PlaceOrder {
                price,
                size,
                side,
                client_order_id,
            } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                PlaceOrderCaller::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            zeta_cpi::place_order(
                Context::new(program_id, &mut accounts, remaining_accounts),
                price,
                size,
                side,
                client_order_id,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn cancel_order(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::CancelOrder::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::CancelOrder { side, order_id } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                CancelOrderCaller::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            zeta_cpi::cancel_order(
                Context::new(program_id, &mut accounts, remaining_accounts),
                side,
                order_id,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn read_program_data(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            let ix = instruction::ReadProgramData::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::ReadProgramData = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                ReadProgramData::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            zeta_cpi::read_program_data(Context::new(
                program_id,
                &mut accounts,
                remaining_accounts,
            ))?;
            accounts.exit(program_id)
        }
    }
}
pub mod zeta_cpi {
    use super::*;
    pub fn initialize_margin_account(ctx: Context<InitializeMarginAccountCaller>) -> ProgramResult {
        zeta_client::initialize_margin_account(
            ctx.accounts.zeta_program.clone(),
            ctx.accounts.initialize_margin_cpi_accounts.clone(),
        )
    }
    pub fn deposit(ctx: Context<DepositCaller>, amount: u64) -> ProgramResult {
        zeta_client::deposit(
            ctx.accounts.zeta_program.clone(),
            ctx.accounts.deposit_cpi_accounts.clone(),
            amount,
        )
    }
    pub fn withdraw(ctx: Context<WithdrawCaller>, amount: u64) -> ProgramResult {
        zeta_client::withdraw(
            ctx.accounts.zeta_program.clone(),
            ctx.accounts.withdraw_cpi_accounts.clone(),
            amount,
        )
    }
    pub fn initialize_open_orders(ctx: Context<InitializeOpenOrdersCaller>) -> ProgramResult {
        zeta_client::initialize_open_orders(
            ctx.accounts.zeta_program.clone(),
            ctx.accounts.initialize_open_orders_cpi_accounts.clone(),
        )
    }
    pub fn place_order(
        ctx: Context<PlaceOrderCaller>,
        price: u64,
        size: u64,
        side: Side,
        client_order_id: Option<u64>,
    ) -> ProgramResult {
        zeta_client::place_order(
            ctx.accounts.zeta_program.clone(),
            ctx.accounts.place_order_cpi_accounts.clone(),
            price,
            size,
            side,
            client_order_id,
        )
    }
    pub fn cancel_order(
        ctx: Context<CancelOrderCaller>,
        side: Side,
        order_id: u128,
    ) -> ProgramResult {
        zeta_client::cancel_order(
            ctx.accounts.zeta_program.clone(),
            ctx.accounts.cancel_order_cpi_accounts.clone(),
            side,
            order_id,
        )
    }
    pub fn read_program_data(ctx: Context<ReadProgramData>) -> ProgramResult {
        let zeta_group =
            deserialize_account_info_zerocopy::<ZetaGroup>(&ctx.accounts.zeta_group).unwrap();
        let expiry_index = zeta_group.front_expiry_index as usize;
        let expiry_series = zeta_group.expiry_series[expiry_index];
        {
            ::solana_program::log::sol_log(&{
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["Active timestamp "],
                    &[::core::fmt::ArgumentV1::new_display(
                        &expiry_series.active_ts,
                    )],
                ));
                res
            });
            ::solana_program::log::sol_log(&{
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["Expiry timestamp "],
                    &[::core::fmt::ArgumentV1::new_display(
                        &expiry_series.expiry_ts,
                    )],
                ));
                res
            });
            let status = zeta_group.expiry_series[expiry_index].status()?;
            ::solana_program::log::sol_log(&{
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["Is market live?: "],
                    &[::core::fmt::ArgumentV1::new_debug(
                        &(status == ExpirySeriesStatus::Live),
                    )],
                ));
                res
            });
            ::solana_program::log::sol_log(&{
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["Is market expired?: "],
                    &[::core::fmt::ArgumentV1::new_debug(
                        &(status == ExpirySeriesStatus::Expired),
                    )],
                ));
                res
            });
        }
        let products_slice = zeta_group.get_products_slice(expiry_index);
        for i in 0..products_slice.len() {
            let product = &products_slice[i];
            let market_index = get_products_slice_market_index(expiry_index, i);
            let strike = product.strike.get_strike()?;
            let market = product.market;
            let kind = product.kind;
            ::solana_program::log::sol_log(&{
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["Market index = ", ", Strike = ", ", Kind = "],
                    &[
                        ::core::fmt::ArgumentV1::new_display(&market_index),
                        ::core::fmt::ArgumentV1::new_display(&strike),
                        ::core::fmt::ArgumentV1::new_debug(&kind),
                    ],
                ));
                res
            });
        }
        let oracle_price = get_native_oracle_price(&ctx.accounts.oracle);
        ::solana_program::log::sol_log(&{
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["Oracle price "],
                &[::core::fmt::ArgumentV1::new_debug(&oracle_price)],
            ));
            res
        });
        let product_index = 0;
        let market_index = get_products_slice_market_index(expiry_index, product_index);
        let greeks = deserialize_account_info_zerocopy::<Greeks>(&ctx.accounts.greeks).unwrap();
        let market_mark_prices = greeks.get_mark_prices_slice(expiry_index)[product_index];
        let market_product_greeks = greeks.get_product_greeks_slice(expiry_index)[product_index];
        ::solana_program::log::sol_log(&{
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &[
                    "Market index = ",
                    ", Mark price = ",
                    ", Delta = ",
                    ", Vega = ",
                    ", IV = ",
                ],
                &[
                    ::core::fmt::ArgumentV1::new_display(&market_index),
                    ::core::fmt::ArgumentV1::new_display(&market_mark_prices),
                    ::core::fmt::ArgumentV1::new_display(&market_product_greeks.delta),
                    ::core::fmt::ArgumentV1::new_debug(&Decimal::from(market_product_greeks.vega)),
                    ::core::fmt::ArgumentV1::new_debug(&Decimal::from(
                        market_product_greeks.volatility,
                    )),
                ],
            ));
            res
        });
        let margin_account =
            deserialize_account_info_zerocopy::<MarginAccount>(&ctx.accounts.margin_account)
                .unwrap();
        ::solana_program::log::sol_log(&{
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["Margin account balance: "],
                &[::core::fmt::ArgumentV1::new_debug(&margin_account.balance)],
            ));
            res
        });
        let position = margin_account.positions[market_index].position;
        let cost_of_trades = margin_account.positions[market_index].cost_of_trades;
        ::solana_program::log::sol_log(&{
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &[
                    "Margin account position for market index ",
                    ": Position=",
                    ", Cost of trades=",
                ],
                &[
                    ::core::fmt::ArgumentV1::new_display(&market_index),
                    ::core::fmt::ArgumentV1::new_display(&position),
                    ::core::fmt::ArgumentV1::new_display(&cost_of_trades),
                ],
            ));
            res
        });
        let initial_margin_requirement =
            margin_account.get_initial_margin(&greeks, &zeta_group, oracle_price);
        let maintenance_margin_requirement =
            margin_account.get_maintenance_margin(&greeks, &zeta_group, oracle_price);
        let total_margin_requirement = initial_margin_requirement
            .checked_add(maintenance_margin_requirement)
            .unwrap();
        ::solana_program::log::sol_log(&{
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["Margin account: Initial: ", ", Maintenance: ", ", Total: "],
                &[
                    ::core::fmt::ArgumentV1::new_display(&initial_margin_requirement),
                    ::core::fmt::ArgumentV1::new_display(&maintenance_margin_requirement),
                    ::core::fmt::ArgumentV1::new_display(&total_margin_requirement),
                ],
            ));
            res
        });
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
    pub struct InitializeMarginAccount;
    impl borsh::ser::BorshSerialize for InitializeMarginAccount {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for InitializeMarginAccount {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for InitializeMarginAccount {
        fn data(&self) -> Vec<u8> {
            let mut d = [67, 235, 66, 102, 167, 171, 120, 197].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct Deposit {
        pub amount: u64,
    }
    impl borsh::ser::BorshSerialize for Deposit
    where
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.amount, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Deposit
    where
        u64: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                amount: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for Deposit {
        fn data(&self) -> Vec<u8> {
            let mut d = [242, 35, 198, 137, 82, 225, 242, 182].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct Withdraw {
        pub amount: u64,
    }
    impl borsh::ser::BorshSerialize for Withdraw
    where
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.amount, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Withdraw
    where
        u64: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                amount: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for Withdraw {
        fn data(&self) -> Vec<u8> {
            let mut d = [183, 18, 70, 156, 148, 109, 161, 34].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct InitializeOpenOrders;
    impl borsh::ser::BorshSerialize for InitializeOpenOrders {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for InitializeOpenOrders {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for InitializeOpenOrders {
        fn data(&self) -> Vec<u8> {
            let mut d = [55, 234, 16, 82, 100, 42, 126, 192].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct PlaceOrder {
        pub price: u64,
        pub size: u64,
        pub side: Side,
        pub client_order_id: Option<u64>,
    }
    impl borsh::ser::BorshSerialize for PlaceOrder
    where
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        Side: borsh::ser::BorshSerialize,
        Option<u64>: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.price, writer)?;
            borsh::BorshSerialize::serialize(&self.size, writer)?;
            borsh::BorshSerialize::serialize(&self.side, writer)?;
            borsh::BorshSerialize::serialize(&self.client_order_id, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for PlaceOrder
    where
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        Side: borsh::BorshDeserialize,
        Option<u64>: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                price: borsh::BorshDeserialize::deserialize(buf)?,
                size: borsh::BorshDeserialize::deserialize(buf)?,
                side: borsh::BorshDeserialize::deserialize(buf)?,
                client_order_id: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for PlaceOrder {
        fn data(&self) -> Vec<u8> {
            let mut d = [51, 194, 155, 175, 109, 130, 96, 106].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct CancelOrder {
        pub side: Side,
        pub order_id: u128,
    }
    impl borsh::ser::BorshSerialize for CancelOrder
    where
        Side: borsh::ser::BorshSerialize,
        u128: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.side, writer)?;
            borsh::BorshSerialize::serialize(&self.order_id, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for CancelOrder
    where
        Side: borsh::BorshDeserialize,
        u128: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                side: borsh::BorshDeserialize::deserialize(buf)?,
                order_id: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for CancelOrder {
        fn data(&self) -> Vec<u8> {
            let mut d = [95, 129, 237, 240, 8, 49, 223, 132].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct ReadProgramData;
    impl borsh::ser::BorshSerialize for ReadProgramData {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for ReadProgramData {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for ReadProgramData {
        fn data(&self) -> Vec<u8> {
            let mut d = [245, 2, 171, 103, 213, 126, 76, 207].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
}
/// An Anchor generated module, providing a set of structs
/// mirroring the structs deriving `Accounts`, where each field is
/// a `Pubkey`. This is useful for specifying accounts for a client.
pub mod accounts {
    pub use crate::__client_accounts_place_order_caller::*;
    pub use crate::__client_accounts_withdraw_caller::*;
    pub use crate::__client_accounts_deposit_caller::*;
    pub use crate::__client_accounts_initialize_open_orders_caller::*;
    pub use crate::__client_accounts_initialize_margin_account_caller::*;
    pub use crate::__client_accounts_cancel_order_caller::*;
    pub use crate::__client_accounts_read_program_data::*;
}
/// Anchor generated Result to be used as the return type for the
/// program.
pub type Result<T> = std::result::Result<T, Error>;
/// Anchor generated error allowing one to easily return a
/// `ProgramError` or a custom, user defined error code by utilizing
/// its `From` implementation.
#[doc(hidden)]
pub enum Error {
    #[error(transparent)]
    ProgramError(#[from] anchor_lang::solana_program::program_error::ProgramError),
    #[error(transparent)]
    ErrorCode(#[from] ErrorCode),
}
#[allow(unused_qualifications)]
impl std::error::Error for Error {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        use thiserror::private::AsDynError;
        #[allow(deprecated)]
        match self {
            Error::ProgramError { 0: transparent } => {
                std::error::Error::source(transparent.as_dyn_error())
            }
            Error::ErrorCode { 0: transparent } => {
                std::error::Error::source(transparent.as_dyn_error())
            }
        }
    }
}
#[allow(unused_qualifications)]
impl std::fmt::Display for Error {
    fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
        match self {
            Error::ProgramError(_0) => std::fmt::Display::fmt(_0, __formatter),
            Error::ErrorCode(_0) => std::fmt::Display::fmt(_0, __formatter),
        }
    }
}
#[allow(unused_qualifications)]
impl std::convert::From<anchor_lang::solana_program::program_error::ProgramError> for Error {
    #[allow(deprecated)]
    fn from(source: anchor_lang::solana_program::program_error::ProgramError) -> Self {
        Error::ProgramError { 0: source }
    }
}
#[allow(unused_qualifications)]
impl std::convert::From<ErrorCode> for Error {
    #[allow(deprecated)]
    fn from(source: ErrorCode) -> Self {
        Error::ErrorCode { 0: source }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Error {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&Error::ProgramError(ref __self_0),) => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_tuple(f, "ProgramError");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&Error::ErrorCode(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "ErrorCode");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
        }
    }
}
#[repr(u32)]
pub enum ErrorCode {
    AccountNotMutable,
    UnsupportedKind,
    ProductStrikeUninitialized,
    InvalidProductMarketKey,
    MarketNotLive,
    ProductDirty,
    InvalidOptionKind,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for ErrorCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&ErrorCode::AccountNotMutable,) => {
                ::core::fmt::Formatter::write_str(f, "AccountNotMutable")
            }
            (&ErrorCode::UnsupportedKind,) => {
                ::core::fmt::Formatter::write_str(f, "UnsupportedKind")
            }
            (&ErrorCode::ProductStrikeUninitialized,) => {
                ::core::fmt::Formatter::write_str(f, "ProductStrikeUninitialized")
            }
            (&ErrorCode::InvalidProductMarketKey,) => {
                ::core::fmt::Formatter::write_str(f, "InvalidProductMarketKey")
            }
            (&ErrorCode::MarketNotLive,) => ::core::fmt::Formatter::write_str(f, "MarketNotLive"),
            (&ErrorCode::ProductDirty,) => ::core::fmt::Formatter::write_str(f, "ProductDirty"),
            (&ErrorCode::InvalidOptionKind,) => {
                ::core::fmt::Formatter::write_str(f, "InvalidOptionKind")
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for ErrorCode {
    #[inline]
    fn clone(&self) -> ErrorCode {
        {
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for ErrorCode {}
impl std::fmt::Display for ErrorCode {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            ErrorCode::AccountNotMutable => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Account not mutable"],
                &[],
            )),
            ErrorCode::UnsupportedKind => {
                fmt.write_fmt(::core::fmt::Arguments::new_v1(&["Unsupported kind"], &[]))
            }
            ErrorCode::ProductStrikeUninitialized => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Product strike uninitialized"],
                &[],
            )),
            ErrorCode::InvalidProductMarketKey => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Invalid product market key"],
                &[],
            )),
            ErrorCode::MarketNotLive => {
                fmt.write_fmt(::core::fmt::Arguments::new_v1(&["Market not live"], &[]))
            }
            ErrorCode::ProductDirty => {
                fmt.write_fmt(::core::fmt::Arguments::new_v1(&["Product dirty"], &[]))
            }
            ErrorCode::InvalidOptionKind => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Invalid option kind, must be Call or Put"],
                &[],
            )),
        }
    }
}
impl std::error::Error for ErrorCode {}
impl std::convert::From<Error> for anchor_lang::solana_program::program_error::ProgramError {
    fn from(e: Error) -> anchor_lang::solana_program::program_error::ProgramError {
        match e {
            Error::ProgramError(e) => e,
            Error::ErrorCode(c) => {
                anchor_lang::solana_program::program_error::ProgramError::Custom(
                    c as u32 + anchor_lang::__private::ERROR_CODE_OFFSET,
                )
            }
        }
    }
}
impl std::convert::From<ErrorCode> for anchor_lang::solana_program::program_error::ProgramError {
    fn from(e: ErrorCode) -> anchor_lang::solana_program::program_error::ProgramError {
        let err: Error = e.into();
        err.into()
    }
}
