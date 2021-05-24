use std::cell::{RefCell, RefMut};

use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use uwa::string::strbuf;

#[derive(Debug)]
pub struct State {
  pub done: RefCell<usize>,
  pub skip: RefCell<usize>,
  pub fail: RefCell<usize>,
  pub pass: RefCell<usize>,
  pub only: RefCell<usize>,
  pub json: RefCell<String>,
}

impl State {
  #[inline]
  pub fn init() -> Self {
    Self {
      done: RefCell::new(0),
      skip: RefCell::new(0),
      fail: RefCell::new(0),
      pass: RefCell::new(0),
      only: RefCell::new(0),
      json: RefCell::new(strbuf![]),
    }
  }

  #[inline]
  pub fn json(&self, s: String) {
    *self.json_mut() = s;
  }

  #[inline]
  pub fn done_mut(&self) -> RefMut<usize> {
    self.done.borrow_mut()
  }

  #[inline]
  pub fn skip_mut(&self) -> RefMut<usize> {
    self.skip.borrow_mut()
  }

  #[inline]
  pub fn fail_mut(&self) -> RefMut<usize> {
    self.fail.borrow_mut()
  }

  #[inline]
  pub fn pass_mut(&self) -> RefMut<usize> {
    self.pass.borrow_mut()
  }

  #[inline]
  pub fn only_mut(&self) -> RefMut<usize> {
    self.only.borrow_mut()
  }

  #[inline]
  pub fn json_mut(&self) -> RefMut<String> {
    self.json.borrow_mut()
  }

  #[inline]
  pub fn inc_done(&self, v: usize) {
    *self.done_mut() += v;
    // binop!(self: done inc v);
  }

  #[inline]
  pub fn inc_skip(&self, v: usize) {
    *self.skip_mut() += v;
  }

  #[inline]
  pub fn inc_fail(&self, v: usize) {
    *self.fail_mut() += v;
  }

  #[inline]
  pub fn inc_pass(&self, v: usize) {
    *self.pass_mut() += v;
  }

  #[inline]
  pub fn inc_only(&self, v: usize) {
    *self.only_mut() += v;
  }

  #[inline]
  pub fn print(&self) {
    let (pass, done, skip, fail) = self.result();

    print!(
      "result: {}\ndone: {}\nskip: {}\nfail: {}\n",
      pass, done, skip, fail
    );
  }

  #[inline]
  pub fn result(&self) -> (usize, usize, usize, usize) {
    let done = *self.done.borrow_mut();
    let skip = *self.skip.borrow_mut();
    let fail = *self.fail.borrow_mut();
    let pass = vec![done, skip, fail].par_iter().sum::<usize>();

    (pass, done, skip, fail)
  }
}

pub macro binop {
  ( $me:ident : $name:ident add $rhs:expr ) => ({ *$me.$name.borrow_mut() + $rhs }),
  ( $me:ident : $name:ident sub $rhs:expr ) => ({ *$me.$name.borrow_mut() - $rhs }),
  ( $me:ident : $name:ident div $rhs:expr ) => ({ *$me.$name.borrow_mut() / $rhs }),
  ( $me:ident : $name:ident mul $rhs:expr ) => ({ *$me.$name.borrow_mut() * $rhs }),
  ( $me:ident : $name:ident inc $rhs:expr ) => ({ *$me.$name.borrow_mut() += $rhs }),
  ( $me:ident : $name:ident dec $rhs:expr ) => ({ *$me.$name.borrow_mut() -= $rhs }),
}
