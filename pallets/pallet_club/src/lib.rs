
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;


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
		#[pallet::constant]
		type MaxValue: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	// #[pallet::getter(fn get_RootClubMembers)]
	pub type RootClubMembers<T: Config> =
		StorageValue<_, BoundedVec<RootClubForm<T>, T::MaxValue>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_request_list)]
	pub type MembershipRequestList<T: Config> = StorageValue<_, BoundedVec<T::AccountId, T::MaxValue>, ValueQuery>;

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


		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
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



