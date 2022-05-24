//! Chainlink Chain Extension
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

use frame_support::{
	log::{error},
};
pub use frame_support::dispatch::DispatchError;
use frame_support::dispatch::Encode;
use log;

use pallet_contracts::chain_extension::{
    RetVal, ChainExtension, Environment, Ext, InitState, SysConfig, UncheckedFrom,
};

use pallet_chainlink_feed::{FeedOracle, FeedInterface};


/// The chain Extension for ChainLink
pub struct SubLinkInkExtension<Runtime>(sp_std::marker::PhantomData<Runtime>);


impl<Runtime> ChainExtension<Runtime> for SubLinkInkExtension<Runtime> 
where   Runtime: pallet_contracts::Config,
        Runtime: sublink_parachain_oracle::Config,
        Runtime: pallet_chainlink_feed::Config,
        Runtime: sublink_xcm::Config,
    {
    fn call<E: Ext>(
        func_id: u32,
        env: Environment<E, InitState>,
    ) -> Result<RetVal, DispatchError>
    where
        <E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
    {
        log::info!("***** Chainlink extension called {:?}", func_id);
        match func_id {
            // latest_data by id
            70930000 => {
				let mut env = env.buf_in_buf_out();
				// let feed_id: <Runtime as pallet_chainlink_feed::Config>::FeedId = env.read_as_unbounded(env.in_len())?;
				let feed_id: <<Runtime as sublink_xcm::Config>::Oracle as FeedOracle<Runtime>>::FeedId = env.read_as_unbounded(env.in_len())?;
                let feed = sublink_parachain_oracle::Pallet::<Runtime>::feed(feed_id.clone()).unwrap();
                let answer = feed.latest_data();
                log::info!("called latest_data extension with feed_id {:?} = {:?}", feed_id, answer);
                let r = answer.encode();
				env.write(&r, false, None).map_err(|_| {
                    log::info!("Error when writing result");
					DispatchError::Other(
						"SubLinkInkExtension failed to return result",
					)
				})?;
            }

            _ => {
                error!("Called an unregistered `func_id`: {:}", func_id);
                return Err(DispatchError::Other("Unimplemented func_id"))
            }
        }

        Ok(RetVal::Converging(0))
    }
}
