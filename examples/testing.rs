fn sqrt(x: u64) -> u64 { x * x }

unit!(
  test!(integration_test_sqrt_should_be_0, must!(eq: sqrt(0), 0));
  test!(integration_test_sqrt_should_be_49, must!(eq: sqrt(7), 49));
  test!(integration_test_sqrt_should_be_falsy, must!(ne: sqrt(7), sqrt(9)));
);
