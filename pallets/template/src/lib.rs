// #![cfg_attr(not(feature = "std"), no_std)]

// /// Edit this file to define custom logic or remove it if it is not needed.
// /// Learn more about FRAME and the core library of Substrate FRAME pallets:
// /// <https://docs.substrate.io/v3/runtime/frame>
// pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

// #[frame_support::pallet]
// pub mod pallet {
// 	use super::*;
// 	use frame_support::pallet_prelude::*;
// 	use frame_system::pallet_prelude::*;

// 	/// Configure the pallet by specifying the parameters and types on which it depends.
// 	#[pallet::config]
// 	pub trait Config: frame_system::Config {
// 		/// Because this pallet emits events, it depends on the runtime's definition of an event.
// 		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
// 		type MAX: Get<u32>;

// 	}

// 	#[pallet::pallet]
// 	#[pallet::generate_store(pub(super) trait Store)]
// 	pub struct Pallet<T>(_);

// 	#[derive(Encode, Decode, PartialEq, MaxEncodedLen, TypeInfo)]
// 	#[scale_info(skip_type_params(T))]
// 	// #[codec(mel_bound())]

// 	pub struct RootClubMembersship<T: Config> {
// 		added_by: T::AccountId,
// 		person: T::AccountId,
// 		block_time: T::BlockNumber
// 	}

// 	#[pallet::storage]
// 	#[pallet::getter(fn get_RootClubMembers)]
// 	pub type RootClubMembers<T: Config> =
// 		StorageValue<_, BoundedVec<RootClubMembersship<T>, T::MAX>, ValueQuery>;

// 	#[pallet::storage]
// 	#[pallet::getter(fn get_club_house)]
// 	pub type ClubHouse<T: Config> =
// 		StorageValue<_, BoundedVec<RootClubMembersship<T>, T::MAX>, ValueQuery>;


// 	// The pallet's runtime storage items.
// 	// https://docs.substrate.io/v3/runtime/storage
// 	#[pallet::storage]
// 	#[pallet::getter(fn something)]
// 	// Learn more about declaring storage items:
// 	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
// 	pub type Something<T> = StorageValue<_, u32>;

// 	// Pallets use events to inform users when important changes are made.
// 	// https://docs.substrate.io/v3/runtime/events-and-errors
// 	#[pallet::event]
// 	#[pallet::generate_deposit(pub(super) fn deposit_event)]

// 	pub enum Event<T: Config> {
// 		/// Event documentation should end with an array that provides descriptive names for event
// 		/// parameters. [something, who]
// 		SomethingStored(u32, T::AccountId),
// 		ClubStored(T::AccountId),

// 	}
	
	


// 	// Errors inform users that something went wrong.
// 	#[pallet::error]
// 	pub enum Error<T> {
// 		/// Error names should be descriptive.
// 		NoneValue,
// 		/// Errors should have helpful documentation associated with them.
// 		StorageOverflow,
// 		MemberAlreadyExist,
// 		MemberNotFound,
// 		MemberLimitExceeded,
// 		CannotUnwrap,
// 		MemberAlreadyARootClubMembers,
// 		NotAMember,
// 	}

// 	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// 	// These functions materialize as "extrinsics", which are often compared to transactions.
// 	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
// 	#[pallet::call]
// 	impl<T: Config> Pallet<T> {
// 		/// An example dispatchable that takes a singles value as a parameter, writes the value to
// 		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
// 		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
// 		pub fn add_to_club(origin: OriginFor<T>,_account:T::AccountId) -> DispatchResult {
// 			// Check that the extrinsic was signed and get the signer.
// 			// This function will return an error if the extrinsic is not signed.
// 			// https://docs.substrate.io/v3/runtime/origins
			
// 			//let who = ensure_signed(origin.clone())?;
// 			// ensure!(who == Self::key(), "RequireSudo");
// 			ensure_root(origin.clone())?;
// 			let who =  ensure_signed_or_root(origin.clone())?;

//             match who{
// 				Some(account_id) => {
// 					// <Something<T>>::put(something);
//                     let _membership_data: RootClubMembersship<T> = RootClubMembersship{
// 						added_by : account_id.clone(),
// 						person: _account.clone(),
// 						block_time : <frame_system::Pallet<T>>::block_number()

// 					};

// 					<RootClubMembers<T>>::try_mutate(|b_vec| b_vec.try_push(_membership_data))
// 					.map_err(|_| <Error<T>>::MemberLimitExceeded)?;					// <Something<T>>::put(10);

// 					// Self::deposit_event(Event::SomethingStored(10, account_id));
// 					Self::deposit_event(Event::ClubStored(_account));


// 				}
//                 None => ()
// 			}


// 			// ensure!(who == Self::key(), "only the current sudo key can sudo");	

// 			// Update storage.

// 			// Emit an event.
// 			// Return a successful DispatchResultWithPostInfo
// 			Ok(())
// 		}

// 		/// An example dispatchable that may throw a custom error.
// 		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
// 		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
// 			let _who = ensure_signed(origin)?;

// 			// Read a value from storage.
// 			match <Something<T>>::get() {
// 				// Return an error if the value has not been set.
// 				None => return Err(Error::<T>::NoneValue.into()),
// 				Some(old) => {
// 					// Increment the value read from storage; will error in the event of overflow.
// 					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
// 					// Update the value in storage with the incremented result.
// 					<Something<T>>::put(new);
// 					Ok(())
// 				},
// 			}
// 		}

		


// 	}
// }



#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::storage::bounded_vec::BoundedVec;
	use frame_support::{
		dispatch::{DispatchResult, PartialEq},
		pallet_prelude::*,
	};
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type MAX: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	// #[pallet::getter(fn get_RootClubMembers)]
	pub type RootClubMembers<T: Config> =
		StorageValue<_, BoundedVec<RootClubForm<T>, T::MAX>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_request_list)]
	pub type MembershipRequestList<T: Config> = StorageValue<_, BoundedVec<T::AccountId, T::MAX>, ValueQuery>;

	#[derive(Encode, Decode, PartialEq, MaxEncodedLen, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	
	pub struct RootClubForm<T: Config> {
		club_member_id: T::AccountId,
		// root_origin_id: T::AccountId,
		added_block: T::BlockNumber
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		MembershipRequestListMemberAdded(T::AccountId),
		RootClubMembersAdded(T::AccountId),
		RootClubMembersRemoved(T::AccountId),
		MemberRemovedfromMembershipRequestList(T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Already a member.
		AlreadyMember,
		/// Not a member.
		NotMember,
		/// Too many members.
		TooManyMembers,
        /// Unable to find Origin Id
		OriginIdError
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/* 
		1. This function is a signed transaction for normal users (apart from Alice, bcoz Alice is the root/sudo user)
		2. Anyone account can call this transaction and request Alice(root user) to add them into a club
		3. If the requested account is already a member of club, this function will throw a error
		4. This function will also throw error, if member limit exceeds
		*/ 


		#[pallet::weight(10000)]
		pub fn request_to_join_club(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// let who = T::Lookup::lookup(wait_member)?;
			let mut members = <MembershipRequestList<T>>::get();

			
			let location = members.binary_search(&who).err().ok_or(Error::<T>::AlreadyMember)?;
				
			
			members
			.try_insert(location, who.clone())
			.map_err(|_| Error::<T>::TooManyMembers)?;

		    <MembershipRequestList<T>>::put(&members);

			Self::deposit_event(Event::MembershipRequestListMemberAdded(who));
			Ok(())
		}


		/* 
		1. This function is a sudo transaction(for dev chain, by default only Alice can execute this tnx with root privilage)
		2. If Any normal account try to execute this function with returned with 'Bad Origin Error'
		3. This function will remove an already added account from the club
		*/ 


		#[pallet::weight(10000)]
		pub fn remove_member_from_club(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;

			<RootClubMembers<T>>::mutate(|club_members_list| {
				if let Some(index) = club_members_list.iter().position(|member| member.club_member_id == who) {
					club_members_list.remove(index);
					return Ok(());
				}
				Err(())
			})
			.map_err(|_| <Error<T>>::NotMember)?;
			Self::deposit_event(Event::RootClubMembersRemoved(who));
			Ok(())
		}


		/* 
		1. This function is a sudo transaction(for dev chain, by default only Alice can execute this tnx with root privilage)
		2. If Any normal account try to execute this function with returned with 'Bad Origin Error'
		3. This function will add an request user account into the club
		*/ 

		#[pallet::weight(10000)]
		pub fn add_requested_member_to_club(
			origin: OriginFor<T>,
			member: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;
			// let origin_id = ensure_signed_or_root(origin)?;
	 
			ensure!(Self::get_request_list().contains(&member) == true, <Error<T>>::NotMember);

			let current_block = <frame_system::Pallet<T>>::block_number();
			let mem = RootClubForm { club_member_id: member.clone(), added_block: current_block };

			<RootClubMembers<T>>::mutate(|club_members_list| club_members_list.try_push(mem))
				.map_err(|_| <Error<T>>::TooManyMembers)?;

			<MembershipRequestList<T>>::mutate(|remove_member| {
				if let Some(index) = remove_member.iter().position(|mem| *mem == member) {
					remove_member.remove(index);
					return Ok(());
				}
				Err(())
			})
			.map_err(|_| <Error<T>>::NotMember)?;

			Self::deposit_event(Event::MemberRemovedfromMembershipRequestList(member.clone()));
			Self::deposit_event(Event::RootClubMembersAdded(member.clone()));
			Ok(())
		}

	
	}
}