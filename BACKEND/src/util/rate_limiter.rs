
use governor::{Quota, RateLimiter as GovernorRateLimiter};
use governor::state::keyed::DefaultKeyedStateStore as _DKStore;
use governor::clock::DefaultClock;

use std::sync::Arc;
use std::net::IpAddr;
use std::num::NonZeroU32; // wth is this??
use std::time::Duration;




pub struct RateLimiter {
  pub login: Arc< GovernorRateLimiter<IpAddr, _DKStore<IpAddr>, DefaultClock> > 
}



impl RateLimiter {
  pub fn init(calls_per_hour: &u32) -> Self {


    // quota per 10 mins
    let quota = Quota::with_period(Duration::from_secs(60*10)).unwrap()
      .allow_burst(NonZeroU32::new(*calls_per_hour).unwrap());

    Self {
      login: Arc::new(GovernorRateLimiter::keyed(quota))
    }



  }
}