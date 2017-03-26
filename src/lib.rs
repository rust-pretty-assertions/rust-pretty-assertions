extern crate difference;

#[doc(hidden)]
pub use difference::Changeset;

#[macro_export]
macro_rules! assert_eq {
    ($left:expr , $right:expr) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let left_dbg = format!("{:?}", *left_val);
                    let right_dbg = format!("{:?}", *right_val);
                    let diff = $crate::Changeset::new(&left_dbg, &right_dbg, " ");

                    panic!("assertion failed: `(left == right)` \
                           (left: `{:?}`, right: `{:?}`, diff: `{}`)", *left_val, *right_val, diff)
                }
            }
        }
    })
}
