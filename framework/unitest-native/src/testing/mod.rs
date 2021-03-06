mod testing;

pub use self::testing::{bind, must, run, test, then, unit};

// /// this represents a test case
// pub macro test {
//   ( $name:ident , $assertions:expr ) => (
//     #[test]
//     fn $name() {
//       let mut state = &store().state;
//       let mut fail = state.borrow_mut();

//       let mut fail = fail.fail(1);

//       print!("\njkjkjk: {:?}\n", fail);

//       $assertions
//     }
//   ),
//   () => {},
// }

// /// this represents an assertion
// pub macro must {
//   ( die : $rhs:expr ) => ({
//     let catched = std::panic::catch_unwind(|| { $rhs; });

//     must!(catched.is_err());
//   }),
//   ( falsy : $rhs:expr ) => ({ assert!(false == $rhs); }),
//   ( truthy : $rhs:expr ) => ({ assert!(true == $rhs); }),
//   ( ne : $lhs:expr , $rhs:expr ) => ({ assert!($lhs != $rhs); }),
//   ( eq : $lhs:expr , $rhs:expr ) => ({ assert!($lhs == $rhs); }),
//   ( $lhs:expr ) => ({ assert!($lhs == true); }),
//   () => (),
// }

// #[test]
// fn unit_test_must_eq() {
//   assert!(() == must!(eq: 0, 0));
// }

// unit!(
//   test!(should_be_eq, must!(eq: 0, 0));
//   test!(should_be_ne, must!(eq: 0, 0));
//   test!(should_be_neee, must!(eq: 0, 0));
// );

// #[test]
// fn unit_test_must_ne() {
//   assert!(() == must!(ne: 1, 0));
// }

// #[test]
// fn unit_test_must_truthy() {
//   assert!(() == must!(truthy: true));
// }

// #[test]
// fn unit_test_must_falsy() {
//   assert!(() == must!(falsy: false));
// }

// #[test]
// fn unit_test_must_panic() {
//   assert!(() == must!(die: { panic!() }));
// }
