#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc721 {
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::collections::{
        hashmap::Entry,
        HashMap as StorageHashMap,
    };
    use scale::{
        Decode,
        Encode,
    };
    use core::convert::TryInto;

    pub type TokenId = u32;

    #[ink(storage)]
    pub struct Erc721 {
        /// Mapping from token to owner.
        token_owner: StorageHashMap<TokenId, AccountId>,
        /// Mapping from token to approvals users.
        token_approvals: StorageHashMap<TokenId, AccountId>,
        /// Mapping from owner to number of owned token.
        owned_tokens_count: StorageHashMap<AccountId, u32>,
        /// Mapping from owner to operator approvals.
        operator_approvals: StorageHashMap<(AccountId, AccountId), bool>,
        //Mapping stats to owner
        victories: StorageHashMap<TokenId, u32>,
        //Mapping losses to owner
        losses: StorageHashMap<TokenId, u32>,
        ///Stores the time (block number) added with one day (7200 blocks) to delay the use of certain public functions
        ready_time: StorageHashMap<AccountId, BlockNumber>,
        ///The heirarchy of angels from penultimate status to highest
        archangel: StorageHashMap<TokenId, bool>,
        principality: StorageHashMap<TokenId, bool>,
        power: StorageHashMap<TokenId, bool>,
        virtue: StorageHashMap<TokenId, bool>,
        dominion: StorageHashMap<TokenId, bool>,
        throne: StorageHashMap<TokenId, bool>,
        cherubim: StorageHashMap<TokenId, bool>,
        seraphim: StorageHashMap<TokenId, bool>,
        ///False if one account attacks the other
        alliances: StorageHashMap<(TokenId, TokenId), bool>,
    }

    #[derive(Encode, Decode, Debug, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotOwner,
        NotApproved,
        TokenExists,
        TokenNotFound,
        CannotInsert,
        CannotRemove,
        CannotFetchValue,
        NotAllowed,
    }

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        id: TokenId,
    }

    /// Event emitted when a token approve occurs.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        id: TokenId,
    }

    /// Event emitted when an operator is enabled or disabled for an owner.
    /// The operator can manage all NFTs of the owner.
    #[ink(event)]
    pub struct ApprovalForAll {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        operator: AccountId,
        approved: bool,
    }

    #[ink(event)]
    pub struct Ascension {
        #[ink(topic)]
        token: TokenId,
        #[ink(topic)]
        victories: u64,
    }

    #[ink(event)]
    pub struct Attack {
        #[ink(topic)]
        attacker: AccountId,
        #[ink(topic)]
        victim: AccountId,
        #[ink(topic)]
        block: BlockNumber,
    }

    #[ink(event)]
    pub struct Alliance {
        #[ink(topic)]
        angel: TokenId,
        #[ink(topic)]
        ally: TokenId,
    }

    ///Public functions
    impl Erc721 {
        /// Creates a new ERC721 token contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                token_owner: Default::default(),
                token_approvals: Default::default(),
                owned_tokens_count: Default::default(),
                operator_approvals: Default::default(),
                victories: Default::default(),
                losses: Default::default(),
                ready_time: Default::default(),
                archangel: Default::default(),
                principality: Default::default(),
                power: Default::default(),
                virtue: Default::default(),
                dominion: Default::default(),
                throne: Default::default(),
                cherubim: Default::default(),
                seraphim: Default::default(),
                alliances: Default::default(),
            }
        }

        /// Returns the balance of the owner.
        /// This represents the amount of unique tokens the owner has.
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> u32 {
            self.balance_of_or_zero(&owner)
        }

        //Returns the victories for an token
        #[ink(message)]
        pub fn victories_count(&self, owner: TokenId) -> u64 {
            self.victories_of_or_zero(&owner)
        }
        ///Returns the losses for a token
        #[ink(message)]
        pub fn losses_count(&self, owner: TokenId) -> u64 {
            self.losses_of_or_zero(&owner)
        }
        ///Getters of credibility status
        #[ink(message)]
        pub fn is_archangel(&self, token: TokenId) -> bool {
            self.archangel(token)
        }
        #[ink(message)]
        pub fn is_principality(&self, token: TokenId) -> bool {
            self.principality(token)
        }
        #[ink(message)]
        pub fn is_power(&self, token: TokenId) -> bool {
            self.power(token)
        }
        #[ink(message)]
        pub fn is_virtue(&self, token: TokenId) -> bool {
            self.virtue(token)
        }
        #[ink(message)]
        pub fn is_dominion(&self, token: TokenId) -> bool {
            self.dominion(token)
        }
        #[ink(message)]
        pub fn is_throne(&self, token:TokenId) -> bool {
            self.throne(token)
        }
        #[ink(message)]
        pub fn is_cherubim(&self, token: TokenId) -> bool {
            self.cherubim(token)
        }
        #[ink(message)]
        pub fn is_seraphim(&self, token: TokenId) -> bool {
            self.seraphim(token)
        }
        pub fn is_ready(&self, account: AccountId) -> BlockNumber {
            self.ready(account)
        }

        /// Returns the owner of the token.
        #[ink(message)]
        pub fn owner_of(&self, id: TokenId) -> Option<AccountId> {
            self.token_owner.get(&id).cloned()
        }
        
        ///Prototype for the actions a player can execute against another player
        #[ink(message, payable)]
        pub fn attack(&mut self, from: TokenId, to: TokenId) -> Result<(), Error> {
            let caller = self.env().caller();
            if self.is_account_allowed(caller) == false {
                return Err(Error::NotAllowed)
            };
            self.add_loss(to);
            self.add_victory(from);
            self.time_constrain(caller, 7200);     
            Ok(())
        }

        ///Token must contain more than 4 victories
        #[ink(message, payable)]
        pub fn ascend(&mut self, id: TokenId) -> Result<(), Error> {
            let caller = self.env().caller();
            if self.is_account_allowed(caller) == false {
                return Err(Error::NotAllowed)
            };
            let vict_count = self.victories_count(id);
            match vict_count {
                4 => self.archangel.insert(id, true),
                8 => self.principality.insert(id, true),
                16 => self.power.insert(id, true),
                32 => self.virtue.insert(id, true),
                64 => self.dominion.insert(id, true),
                128 => self.throne.insert(id, true),
                256 => self.cherubim.insert(id, true),
                512 => self.seraphim.insert(id, true),
                _ => return Err(Error::NotAllowed),
            };
            self.env().emit_event(Ascension {
                token: id,
                victories: vict_count,
            });
            self.time_constrain(caller, 7200);
            Ok(())
        }

        #[ink(message, payable)]
        pub fn erase_loss(&mut self, id: TokenId) -> Result<(), Error> {
            let caller = self.env().caller();
            if self.is_account_allowed(caller) == false {
                return Err(Error::NotAllowed)
            };
            assert!(self.victories_count(id) > 63, "Must have at least 64 victories");
            let Self {
                losses,
                ..
            } = self;
            decrease_counter_of_tokenid(losses, &id)?;
            self.time_constrain(caller, 7200);
            Ok(())
        }

        ///Seraphims can remove the status of archangels to simple angels
        #[ink(message, payable)]
        pub fn relegate_archangel(&mut self, id: TokenId, to: TokenId) -> Result<(), Error> {
            let caller = self.env().caller();
            if self.is_account_allowed(caller) == false {
                return Err(Error::NotAllowed)
            };
            assert!(self.is_seraphim(id) == true, "Only Seraphims are allowed");
            if self.is_archangel(to) == false{
                return Err(Error::NotAllowed)
            };

            self.archangel.entry(to).or_insert(false);
            self.time_constrain(caller, 7200);
            Ok(())
        }

        #[ink(message)]
        pub fn delay_angel(&mut self, from: TokenId, opponent: AccountId) -> Result<(), Error> {
            assert!(self.is_archangel(from) == true, "Only Archangels are allowed");
            let caller = self.env().caller();
            if self.is_account_allowed(caller) == false {
                return Err(Error::NotAllowed)
            };
            self.time_constrain(opponent, 7200);
            self.time_constrain(caller, 3600);
            Ok(())
        }

        #[ink(message, payable)]
        pub fn form_alliance(&mut self, angel: TokenId, ally: TokenId) -> Result<(), Error> {
            let caller = self.env().caller();
            if self.is_account_allowed(caller) == false {
                return Err(Error::NotAllowed)
            };
            self.ally(angel, ally, true)?;
            self.time_constrain(caller, 7200);
            Ok(())
        }

        #[ink(message, payable)]
        pub fn dissolve_alliance(&mut self, attacker: TokenId, victim: TokenId, ally: TokenId) -> Result<(), Error> {
            let caller = self.env().caller();
            if self.is_account_allowed(caller) == false {
                return Err(Error::NotAllowed)
            };
            assert!(self.is_cherubim(attacker) == true, "Only Cherubs allowed");
            self.ally(victim, ally, false)?;
            self.time_constrain(caller, 7200);
            Ok(())
        }
        
        #[ink(message, payable)]
        pub fn gangel_bangel(
            &mut self,
            attacker: TokenId, 
            attacker_ally: TokenId, 
            victim: TokenId) -> Result<(), Error> {
                let caller = self.env().caller();
                if self.is_account_allowed(caller) == false {
                    return Err(Error::NotAllowed)
                };
                assert!(self.is_allied(attacker, attacker_ally) == true, "Only alliances allowed");
                assert!(self.is_power(victim) != true, "Powers are immune");
            self.add_loss(victim);
            let Self {
                victories,
                ..
            } = self;
            decrease_counter_of_tokenid(victories, &victim)?;
            self.time_constrain(caller, 14400);
            Ok(())
        }

        #[ink(message)]
        pub fn is_allied(&self, angel: TokenId, _angel: TokenId) -> bool {
            self.allied(angel, _angel)
        }

        /// Returns the approved account ID for this token if any.
        #[ink(message)]
        pub fn get_approved(&self, id: TokenId) -> Option<AccountId> {
            self.token_approvals.get(&id).cloned()
        }

        /// Returns `true` if the operator is approved by the owner.
        #[ink(message)]
        pub fn is_approved_for_all(&self, owner: AccountId, operator: AccountId) -> bool {
            self.approved_for_all(owner, operator)
        }

        /// Approves or disapproves the operator for all tokens of the caller.
        #[ink(message)]
        pub fn set_approval_for_all(
            &mut self,
            to: AccountId,
            approved: bool,
        ) -> Result<(), Error> {
            self.approve_for_all(to, approved)?;
            Ok(())
        }

        /// Approves the account to transfer the specified token on behalf of the caller.
        #[ink(message)]
        pub fn approve(&mut self, to: AccountId, id: TokenId) -> Result<(), Error> {
            self.approve_for(&to, id)?;
            Ok(())
        }

        /// Transfers the token from the caller to the given destination.
        #[ink(message)]
        pub fn transfer(
            &mut self,
            destination: AccountId,
            id: TokenId,
        ) -> Result<(), Error> {
            let caller = self.env().caller();
            self.transfer_token_from(&caller, &destination, id)?;
            Ok(())
        }

        /// Transfer approved or owned token.
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            id: TokenId,
        ) -> Result<(), Error> {
            self.transfer_token_from(&from, &to, id)?;
            Ok(())
        }

        /// Creates a new token.
        #[ink(message, payable)]
        pub fn mint(&mut self, id: TokenId) -> Result<(), Error> {
            let caller = self.env().caller();
            assert!(self.balance_of(caller) == 0, "Must own no tokens");
            self.add_token_to(&caller, id)?;
            self.env().emit_event(Transfer {
                from: Some(AccountId::from([0x0; 32])),
                to: Some(caller),
                id,
            });
            Ok(())
        }

        /// Deletes an existing token. Only the owner can burn the token.
        #[ink(message)]
        pub fn burn(&mut self, id: TokenId) -> Result<(), Error> {
            let caller = self.env().caller();
            let Self {
                token_owner,
                owned_tokens_count,
                ..
            } = self;
            let occupied = match token_owner.entry(id) {
                Entry::Vacant(_) => return Err(Error::TokenNotFound),
                Entry::Occupied(occupied) => occupied,
            };
            //If the occupying token does not match the reference to the caller, error returns
            if occupied.get() != &caller {
                return Err(Error::NotOwner)
            };
            decrease_counter_of(owned_tokens_count, &caller)?;
            occupied.remove_entry();
            self.env().emit_event(Transfer {
                from: Some(caller),
                to: Some(AccountId::from([0x0; 32])),
                id,
            });
            Ok(())
        }
        

        //Private functions
        /// Transfers token `id` `from` the sender to the `to` AccountId.
        fn transfer_token_from(
            &mut self,
            from: &AccountId,
            to: &AccountId,
            id: TokenId,
        ) -> Result<(), Error> {
            let caller = self.env().caller();
            if !self.exists(id) {
                return Err(Error::TokenNotFound)
            };
            if !self.approved_or_owner(Some(caller), id) {
                return Err(Error::NotApproved)
            };
            self.clear_approval(id)?;
            self.remove_token_from(from, id)?;
            self.add_token_to(to, id)?;
            self.env().emit_event(Transfer {
                from: Some(*from),
                to: Some(*to),
                id,
            });
            Ok(())
        }


        /// Removes token `id` from the owner.
        fn remove_token_from(
            &mut self,
            from: &AccountId,
            id: TokenId,
        ) -> Result<(), Error> {
            let Self {
                token_owner,
                owned_tokens_count,
                ..
            } = self;
            let occupied = match token_owner.entry(id) {
                Entry::Vacant(_) => return Err(Error::TokenNotFound),
                Entry::Occupied(occupied) => occupied,
            };
            decrease_counter_of(owned_tokens_count, from)?;
            occupied.remove_entry();
            Ok(())
        }

        /// Adds the token `id` to the `to` AccountID.
        fn add_token_to(&mut self, to: &AccountId, id: TokenId) -> Result<(), Error> {
            let Self {
                token_owner,
                owned_tokens_count,
                ..
            } = self;
            let vacant_token_owner = match token_owner.entry(id) {
                Entry::Vacant(vacant) => vacant,
                Entry::Occupied(_) => return Err(Error::TokenExists),
            };
            if *to == AccountId::from([0x0; 32]) {
                return Err(Error::NotAllowed)
            };
            let entry = owned_tokens_count.entry(*to);
            increase_counter_of(entry);
            vacant_token_owner.insert(*to);
            Ok(())
        }

        /// Approves or disapproves the operator to transfer all tokens of the caller.
        fn approve_for_all(
            &mut self,
            to: AccountId,
            approved: bool,
        ) -> Result<(), Error> {
            let caller = self.env().caller();
            if to == caller {
                return Err(Error::NotAllowed)
            }
            self.env().emit_event(ApprovalForAll {
                owner: caller,
                operator: to,
                approved,
            });
            if self.approved_for_all(caller, to) {
                let status = self
                    .operator_approvals
                    .get_mut(&(caller, to))
                    .ok_or(Error::CannotFetchValue)?;
                *status = approved;
                Ok(())
            } else {
                match self.operator_approvals.insert((caller, to), approved) {
                    Some(_) => Err(Error::CannotInsert),
                    None => Ok(()),
                }
            }
        }

        /// Approve the passed AccountId to transfer the specified token on behalf of the message's sender.
        fn approve_for(&mut self, to: &AccountId, id: TokenId) -> Result<(), Error> {
            let caller = self.env().caller();
            let owner = self.owner_of(id);
            if !(owner == Some(caller)
                || self.approved_for_all(owner.expect("Error with AccountId"), caller))
            {
                return Err(Error::NotAllowed)
            };
            if *to == AccountId::from([0x0; 32]) {
                return Err(Error::NotAllowed)
            };

            if self.token_approvals.insert(id, *to).is_some() {
                return Err(Error::CannotInsert)
            };
            self.env().emit_event(Approval {
                from: caller,
                to: *to,
                id,
            });
            Ok(())
        }

        /// Removes existing approval from token `id`.
        fn clear_approval(&mut self, id: TokenId) -> Result<(), Error> {
            if !self.token_approvals.contains_key(&id) {
                return Ok(())
            };
            match self.token_approvals.take(&id) {
                Some(_res) => Ok(()),
                None => Err(Error::CannotRemove),
            }
        }

        ///Allows Dominions to form alliances by adding a tuple of
        ///angels -> bool
        fn ally(&mut self, angel: TokenId, an_ally: TokenId, approval: bool) -> Result<(), Error> {
            let caller = self.env().caller();
            assert!(self.is_account_allowed(caller) == true, "Must wait a few blocks more");
            if self.is_dominion(angel) == false || self.is_dominion(an_ally) {
                return Err(Error::NotAllowed)
            };
            self.env().emit_event(Alliance {
                angel,
                ally: an_ally,
            });
            match self.alliances.insert((angel, an_ally), approval) {
                Some(_) => Err(Error::CannotInsert),
                None => Ok(()),
            }   
        }

        fn archangel(&self, token: TokenId) -> bool {
            *self.archangel.get(&token).unwrap_or(&false)
        }
        fn principality(&self, token: TokenId) -> bool {
            *self.principality.get(&token).unwrap_or(&false)
        }
        fn power(&self, token: TokenId) -> bool {
            *self.power.get(&token).unwrap_or(&false)
        }
        fn virtue(&self, token: TokenId) -> bool {
            *self.virtue.get(&token).unwrap_or(&false)
        }
        fn dominion(&self, token: TokenId) -> bool {
            *self.dominion.get(&token).unwrap_or(&false)
        }
        fn throne(&self, token: TokenId) -> bool {
            *self.throne.get(&token).unwrap_or(&false)
        }
        fn cherubim(&self, token: TokenId) -> bool {
            *self.cherubim.get(&token).unwrap_or(&false)
        }
        fn seraphim(&self, token: TokenId) -> bool {
            *self.seraphim.get(&token).unwrap_or(&false)
        }
        fn ready(&self, account: AccountId) -> BlockNumber {
            *self.ready_time.get(&account).unwrap_or(&0)
        }

        // Returns the total number of tokens from an account.
        fn balance_of_or_zero(&self, of: &AccountId) -> u32 {
            *self.owned_tokens_count.get(of).unwrap_or(&0)
        }

        //Returns the victories from an account
        fn victories_of_or_zero(&self, of: &TokenId) -> u64 {
            (*self.victories.get(of).unwrap_or(&0)).into()
        }

        ///Returns the losses from an account
        fn losses_of_or_zero(&self, of: &TokenId) -> u64 {
            (*self.losses.get(of).unwrap_or(&0)).into()
        }

        fn allied(&self, angel: TokenId, _angel: TokenId) -> bool {
            *self.alliances.get(&(angel, _angel)).unwrap_or(&false)
        }

        /// Gets an operator on other Account's behalf.
        fn approved_for_all(&self, owner: AccountId, operator: AccountId) -> bool {
            *self
                .operator_approvals
                .get(&(owner, operator))
                .unwrap_or(&false)
        }

        /// Returns true if the AccountId `from` is the owner of token `id`
        /// or it has been approved on behalf of the token `id` owner.
        fn approved_or_owner(&self, from: Option<AccountId>, id: TokenId) -> bool {
            let owner = self.owner_of(id);
            from != Some(AccountId::from([0x0; 32]))
                && (from == owner
                    || from == self.token_approvals.get(&id).cloned()
                    || self.approved_for_all(
                        owner.expect("Error with AccountId"),
                        from.expect("Error with AccountId"),
                    ))
        }

        /// Returns true if token `id` exists or false if it does not.
        fn exists(&self, id: TokenId) -> bool {
            self.token_owner.get(&id).is_some() && self.token_owner.contains_key(&id)
        }
        
        ///Will be inherited inside another function that executes this stat change
        ///along with add_victory
        fn add_loss(&mut self, id: TokenId) -> bool {
            let losses_count = self.losses_count(id);
            self.losses.insert(id, (losses_count + 1).try_into().unwrap());
            true
        }
        fn add_victory(&mut self, id: TokenId) -> bool {
            let victories_count = self.victories_count(id);
            self.victories.insert(id, (victories_count + 1).try_into().unwrap());
            true
        }

        ///Adds the current block number plus a certin number of blocks to the ready_time map
        fn time_constrain(&mut self, account: AccountId, natural: u32) {
            let blocked_natural: BlockNumber = natural.into();
            let limit: BlockNumber = blocked_natural + self.env().block_number();
            self.ready_time.entry(account).or_insert(limit);
            self.env().emit_event(Attack {
                attacker: self.env().caller(),
                victim: account,
                block: limit,
            });
        }
        ///Checks the account id's respective block number, in ready_time map,
        ///if a certain number of blocks have passed
        fn is_account_allowed(&self, id: AccountId) -> bool {
            let last_block = self.is_ready(id);
            return last_block > self.env().block_number()
        }


    }

    fn decrease_counter_of(
        hmap: &mut StorageHashMap<AccountId, u32>,
        of: &AccountId,
    ) -> Result<(), Error> {
        let count = (*hmap).get_mut(of).ok_or(Error::CannotFetchValue)?;
        *count -= 1;
        Ok(())
    }

    fn decrease_counter_of_tokenid(
        hmap: &mut StorageHashMap<TokenId, u32>,
        token: &TokenId
    ) -> Result<(), Error> {
        let count = (*hmap).get_mut(&token).ok_or(Error::CannotFetchValue)?;
        *count -= 1;
        Ok(())
    }

    /// Increase token counter from the `of` AccountId.
    fn increase_counter_of(entry: Entry<AccountId, u32>) {
        entry.and_modify(|v| *v += 1).or_insert(1);
    }

    /// Unit tests
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        use ink_env::{
            call,
            test,
        };
        use ink_lang as ink;

        #[ink::test]
        fn mint_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut erc721 = Erc721::new();
            // Token 1 does not exists.
            assert_eq!(erc721.owner_of(1), None);
            // Alice does not owns tokens.
            assert_eq!(erc721.balance_of(accounts.alice), 0);
            // Create token Id 1.
            assert_eq!(erc721.mint(1), Ok(()));
            // Alice owns 1 token.
            assert_eq!(erc721.balance_of(accounts.alice), 1);
            //Checking getter for credibility and the default bool, works as intended for token id 1
            assert_eq!(erc721.is_credible(1), false);
        }

        #[ink::test]
        fn mint_existing_should_fail() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut erc721 = Erc721::new();
            // Create token Id 1.
            assert_eq!(erc721.mint(1), Ok(()));
            // The first Transfer event takes place
            assert_eq!(1, ink_env::test::recorded_events().count());
            // Alice owns 1 token.
            assert_eq!(erc721.balance_of(accounts.alice), 1);
            // Alice owns token Id 1.
            assert_eq!(erc721.owner_of(1), Some(accounts.alice));
            // Cannot create  token Id if it exists.
            // Bob cannot own token Id 1.
            assert_eq!(erc721.mint(1), Err(Error::TokenExists));
        }

        #[ink::test]
        fn transfer_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut erc721 = Erc721::new();
            // Create token Id 1 for Alice
            assert_eq!(erc721.mint(1), Ok(()));
            // Alice owns token 1
            assert_eq!(erc721.balance_of(accounts.alice), 1);
            // Bob does not owns any token
            assert_eq!(erc721.balance_of(accounts.bob), 0);
            // The first Transfer event takes place
            assert_eq!(1, ink_env::test::recorded_events().count());
            // Alice transfers token 1 to Bob
            assert_eq!(erc721.transfer(accounts.bob, 1), Ok(()));
            // The second Transfer event takes place
            assert_eq!(2, ink_env::test::recorded_events().count());
            // Bob owns token 1
            assert_eq!(erc721.balance_of(accounts.bob), 1);
        }

        #[ink::test]
        fn invalid_transfer_should_fail() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut erc721 = Erc721::new();
            // Transfer token fails if it does not exists.
            assert_eq!(erc721.transfer(accounts.bob, 2), Err(Error::TokenNotFound));
            // Token Id 2 does not exists.
            assert_eq!(erc721.owner_of(2), None);
            // Create token Id 2.
            assert_eq!(erc721.mint(2), Ok(()));
            // Alice owns 1 token.
            assert_eq!(erc721.balance_of(accounts.alice), 1);
            // Token Id 2 is owned by Alice.
            assert_eq!(erc721.owner_of(2), Some(accounts.alice));
            // Get contract address
            let callee = ink_env::account_id::<ink_env::DefaultEnvironment>()
                .unwrap_or([0x0; 32].into());
            // Create call
            let mut data =
                ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])); // balance_of
            data.push_arg(&accounts.bob);
            // Push the new execution context to set Bob as caller
            ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
                accounts.bob,
                callee,
                1000000,
                1000000,
                data,
            );
            // Bob cannot transfer not owned tokens.
            assert_eq!(erc721.transfer(accounts.eve, 2), Err(Error::NotApproved));
        }

        #[ink::test]
        fn approved_transfer_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut erc721 = Erc721::new();
            // Create token Id 1.
            assert_eq!(erc721.mint(1), Ok(()));
            // Token Id 1 is owned by Alice.
            assert_eq!(erc721.owner_of(1), Some(accounts.alice));
            // Approve token Id 1 transfer for Bob on behalf of Alice.
            assert_eq!(erc721.approve(accounts.bob, 1), Ok(()));
            // Get contract address.
            let callee = ink_env::account_id::<ink_env::DefaultEnvironment>()
                .unwrap_or([0x0; 32].into());
            // Create call
            let mut data =
                ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])); // balance_of
            data.push_arg(&accounts.bob);
            // Push the new execution context to set Bob as caller
            ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
                accounts.bob,
                callee,
                1000000,
                1000000,
                data,
            );
            // Bob transfers token Id 1 from Alice to Eve.
            assert_eq!(
                erc721.transfer_from(accounts.alice, accounts.eve, 1),
                Ok(())
            );
            // TokenId 3 is owned by Eve.
            assert_eq!(erc721.owner_of(1), Some(accounts.eve));
            // Alice does not owns tokens.
            assert_eq!(erc721.balance_of(accounts.alice), 0);
            // Bob does not owns tokens.
            assert_eq!(erc721.balance_of(accounts.bob), 0);
            // Eve owns 1 token.
            assert_eq!(erc721.balance_of(accounts.eve), 1);
        }

        #[ink::test]
        fn approved_for_all_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut erc721 = Erc721::new();
            // Create token Id 1.
            assert_eq!(erc721.mint(1), Ok(()));
            // Create token Id 2.
            assert_eq!(erc721.mint(2), Ok(()));
            // Alice owns 2 tokens.
            assert_eq!(erc721.balance_of(accounts.alice), 2);
            // Approve token Id 1 transfer for Bob on behalf of Alice.
            assert_eq!(erc721.set_approval_for_all(accounts.bob, true), Ok(()));
            // Bob is an approved operator for Alice
            assert_eq!(
                erc721.is_approved_for_all(accounts.alice, accounts.bob),
                true
            );
            // Get contract address.
            let callee = ink_env::account_id::<ink_env::DefaultEnvironment>()
                .unwrap_or([0x0; 32].into());
            // Create call
            let mut data =
                ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])); // balance_of
            data.push_arg(&accounts.bob);
            // Push the new execution context to set Bob as caller
            ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
                accounts.bob,
                callee,
                1000000,
                1000000,
                data,
            );
            // Bob transfers token Id 1 from Alice to Eve.
            assert_eq!(
                erc721.transfer_from(accounts.alice, accounts.eve, 1),
                Ok(())
            );
            // TokenId 1 is owned by Eve.
            assert_eq!(erc721.owner_of(1), Some(accounts.eve));
            // Alice owns 1 token.
            assert_eq!(erc721.balance_of(accounts.alice), 1);
            // Bob transfers token Id 2 from Alice to Eve.
            assert_eq!(
                erc721.transfer_from(accounts.alice, accounts.eve, 2),
                Ok(())
            );
            // Bob does not owns tokens.
            assert_eq!(erc721.balance_of(accounts.bob), 0);
            // Eve owns 2 tokens.
            assert_eq!(erc721.balance_of(accounts.eve), 2);
            // Get back to the parent execution context.
            ink_env::test::pop_execution_context();
            // Remove operator approval for Bob on behalf of Alice.
            assert_eq!(erc721.set_approval_for_all(accounts.bob, false), Ok(()));
            // Bob is not an approved operator for Alice.
            assert_eq!(
                erc721.is_approved_for_all(accounts.alice, accounts.bob),
                false
            );
        }

        #[ink::test]
        fn not_approved_transfer_should_fail() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut erc721 = Erc721::new();
            // Create token Id 1.
            assert_eq!(erc721.mint(1), Ok(()));
            // Alice owns 1 token.
            assert_eq!(erc721.balance_of(accounts.alice), 1);
            // Bob does not owns tokens.
            assert_eq!(erc721.balance_of(accounts.bob), 0);
            // Eve does not owns tokens.
            assert_eq!(erc721.balance_of(accounts.eve), 0);
            // Get contract address.
            let callee = ink_env::account_id::<ink_env::DefaultEnvironment>()
                .unwrap_or([0x0; 32].into());
            // Create call
            let mut data =
                ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])); // balance_of
            data.push_arg(&accounts.bob);
            // Push the new execution context to set Eve as caller
            ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
                accounts.eve,
                callee,
                1000000,
                1000000,
                data,
            );
            // Eve is not an approved operator by Alice.
            assert_eq!(
                erc721.transfer_from(accounts.alice, accounts.frank, 1),
                Err(Error::NotApproved)
            );
            // Alice owns 1 token.
            assert_eq!(erc721.balance_of(accounts.alice), 1);
            // Bob does not owns tokens.
            assert_eq!(erc721.balance_of(accounts.bob), 0);
            // Eve does not owns tokens.
            assert_eq!(erc721.balance_of(accounts.eve), 0);
        }

        #[ink::test]
        fn burn_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut erc721 = Erc721::new();
            // Create token Id 1 for Alice
            assert_eq!(erc721.mint(1), Ok(()));
            // Alice owns 1 token.
            assert_eq!(erc721.balance_of(accounts.alice), 1);
            // Alice owns token Id 1.
            assert_eq!(erc721.owner_of(1), Some(accounts.alice));
            // Destroy token Id 1.
            assert_eq!(erc721.burn(1), Ok(()));
            // Alice does not owns tokens.
            assert_eq!(erc721.balance_of(accounts.alice), 0);
            // Token Id 1 does not exists
            assert_eq!(erc721.owner_of(1), None);
        }

        #[ink::test]
        fn burn_fails_token_not_found() {
            // Create a new contract instance.
            let mut erc721 = Erc721::new();
            // Try burning a non existent token
            assert_eq!(erc721.burn(1), Err(Error::TokenNotFound));
        }

        #[ink::test]
        fn burn_fails_not_owner() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            // Create a new contract instance.
            let mut erc721 = Erc721::new();
            // Create token Id 1 for Alice
            assert_eq!(erc721.mint(1), Ok(()));
            // Try burning this token with a different account
            set_sender(accounts.eve);
            assert_eq!(erc721.burn(1), Err(Error::NotOwner));
        }

        fn set_sender(sender: AccountId) {
            let callee = ink_env::account_id::<ink_env::DefaultEnvironment>()
                .unwrap_or([0x0; 32].into());
            test::push_execution_context::<Environment>(
                sender,
                callee,
                1000000,
                1000000,
                test::CallData::new(call::Selector::new([0x00; 4])), // dummy
            );
        }
    }
}
