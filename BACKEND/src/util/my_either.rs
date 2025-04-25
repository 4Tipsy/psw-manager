
/*
* Cuz either::Either does not support serde_untagged for types
*
*/
#[allow(unused_imports)]
use either::Either as _i_will_leave_this_in_proj_anyway;
/* */



use serde::{Serialize, Deserialize};









#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum MyEither<L, R> {
  Left(L),
  Right(R),
}





impl <L, R> MyEither<L, R> {

  pub fn either<F, G, T>(self, f: F, g: G) -> T
  where
    F: FnOnce(L) -> T,
    G: FnOnce(R) -> T,
  {
    match self {
      MyEither::Left(l) => f(l),
      MyEither::Right(r) => g(r),
    }
  }
}