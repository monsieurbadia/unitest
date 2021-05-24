pub macro run {
  () => (),
  () => (),
}

pub macro unit {
  ( $( $e:item )* ) => { $( $e )* },
  () => {},
}

/// this represents a test case
pub macro test {
  ( $name:ident , $assertions:expr ) => (
    #[test]
    fn $name() { $assertions }
  ),
  () => {},
}

/// this represents an assertion
pub macro must {
  ( die : $rhs:expr ) => ({
    let catched = std::panic::catch_unwind(|| { $rhs; });

    must!(catched.is_err());
  }),
  ( falsy : $rhs:expr ) => ({ assert!(false == $rhs); }),
  ( truthy : $rhs:expr ) => ({ assert!(true == $rhs); }),
  ( ne : $lhs:expr , $rhs:expr ) => ({ assert!($lhs != $rhs); }),
  ( eq : $lhs:expr , $rhs:expr ) => ({ assert!($lhs == $rhs); }),
  ( $lhs:expr ) => ({ assert!($lhs == true); }),
  () => (),
}

pub macro bind {
  () => (),
  () => (),
}

pub macro then {
  () => (),
  () => (),
}

#[test]
fn unit_test_must_eq() {
  assert!(() == must!(eq: 0, 0));
}

#[test]
fn unit_test_must_ne() {
  assert!(() == must!(ne: 1, 0));
}

#[test]
fn unit_test_must_truthy() {
  assert!(() == must!(truthy: true));
}

#[test]
fn unit_test_must_falsy() {
  assert!(() == must!(falsy: false));
}

#[test]
fn unit_test_must_panic() {
  assert!(() == must!(die: { panic!() }));
}