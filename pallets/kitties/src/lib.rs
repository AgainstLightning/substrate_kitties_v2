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
    use frame_support::{
        dispatch::DispatchResult, pallet_prelude::*, traits::Randomness, RuntimeDebug,
    };
    use frame_system::pallet_prelude::*;

    #[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
    pub struct Kitty(pub [u8; 16]);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn kitties)]
    pub(super) type Kitties<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        u32,
        Option<Kitty>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn next_kitty_id)]
    pub(super) type NextKittyId<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A kitty was created. [owner, kitty_id, kitty]
        KittyCreated(T::AccountId, u32, Kitty),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn create(origin: OriginFor<T>, something: u32) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            let seed = <pallet_randomness_collective_flip::Module<T> as Randomness<
                T::Hash,
                T::BlockNumber,
            >>::random_seed();
            let payload = (seed, &sender, <frame_system::Pallet<T>>::extrinsic_index());

            // Update storage.
            // <Something<T>>::put(something);

            // Emit an event.
            Self::deposit_event(Event::SomethingStored(something, sender));
            // Return a successful DispatchResultWithPostInfo
            Ok(())
        }

        /// An example dispatchable that may throw a custom error.
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            // Read a value from storage.
            // match <Something<T>>::get() {
            //     // Return an error if the value has not been set.
            //     None => Err(Error::<T>::NoneValue)?,
            //     Some(old) => {
            //         // Increment the value read from storage; will error in the event of overflow.
            //         let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
            //         // Update the value in storage with the incremented result.
            //         <Something<T>>::put(new);
            //         Ok(())
            //     }
            // }

            Ok(())
        }
    }
}
