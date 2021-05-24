use std::fmt::Write;
use std::fs::File;
use std::thread;

use rayon::prelude::*;
use rayon::iter::ParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use serde::Serialize;
use uwa::ch::is;
use uwa::fp::and_then;

use super::state::State;
use super::store::store;
use super::test::Test;

pub macro bind {
  () => (),
}

/// represents a main function wrapper
pub macro run {
  () => (),
  ( $e:expr ) => (
    fn main() {
      $e();
    }
  )
}

/// this represents a simple unit test wrapper
pub macro unit {
  () => (),
  ( $( $e:expr ) , * ) => ({
    $( $e(); )*
    || {
      // print
      let mut s = &store().state;
      let mut state = s.borrow_mut();
      state.print();
    }
  })
}

/// this represents a test suite
pub macro test {
  () => (),
  ( $name:ident , $f:expr ) => ({
    let name = stringify!($name);
    let first = name.chars().next().unwrap_or('\0');

    if is!(underscore first) {
      &store().state_mut().inc_skip(1);
      &store().tests_mut().push(Test::new(name, false));
    } else {
      $f();
      &store().tests_mut().push(Test::new(name, true));
    }

    || {
      // print!("out: {:#?}\n", &store().tests_mut());
    }
  })
}

/// this represents an assertion
pub macro must {
  ( $lhs:expr, $rhs:expr ) => ({
    if exec!($lhs, $rhs) {
      &store().state_mut().inc_done(1);
    } else {
      &store().state_mut().inc_fail(1);
    }

    || {

    }
  }),
  () => (),
}

/// executes
pub macro exec {
  ( $a:expr , $b:expr ) => { $a == $b },
  () => (),
}

pub macro then {
  ( export $name:expr, $path:expr ) => ({  
    let mut tests = &store().tests;
    let mut tests = tests.borrow();

    fn serialized(test: &Test, tx: flume::Sender<String>) {
      let serialized = serde_json::to_string(&test).unwrap();
      tx.send(serialized).unwrap();
    }

    let (tx, rx) = flume::unbounded();

    for test in tests.clone() {
      let thread_tx = tx.clone();
      thread::spawn(move || serialized(&test, thread_tx));
    }

    let mut buf = Vec::with_capacity(tests.len());
    for tt in 0..tests.len() {
      buf.push(rx.recv().unwrap());
    }

    &store().state_mut().json(buf.join(""));

    || {
      use std::io::Write;

      let pathname = format!("{}/{}.json", $path, $name);
      let mut file = File::create(pathname).unwrap();
      let json = format!("{}", *store().state_mut().json_mut());

      file.write_all(json.as_bytes()).unwrap();
    }
  }),
}
