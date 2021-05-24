use unitest::prelude::*;

fn sqrt(x: u64) -> u64 {
  x * x
}

run!(
  bind!( rayon::prelude::* ),
  unit!(
    test!(should_be_0, must!(0, 0)),
    test!(should_be_49, must!(0, 0)),
    test!(should_be_false, must!(0, 0)),
    then!(export "unitest-report", "./examples/out")
  )
);
