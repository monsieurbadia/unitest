use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use super::state::State;
use super::test::Test;

pub struct Store {
  pub state: RefCell<State>,
  pub tests: RefCell<Vec<Test>>,
}

impl Store {
  #[inline]
  pub fn state_mut(&self) -> RefMut<State> {
    self.state.borrow_mut()
  }

  #[inline]
  pub fn tests(&self) -> Ref<Vec<Test>> {
    self.tests.borrow()
  }

  #[inline]
  pub fn tests_mut(&self) -> RefMut<Vec<Test>> {
    self.tests.borrow_mut()
  }
}

#[inline]
pub fn store() -> Rc<Store> {
  thread_local! {
    static STORE: Rc<Store> = Rc::new(Store {
      state: RefCell::new(State::init()),
      tests: RefCell::new(vec![]),
    })
  };

  STORE.with(|s| Rc::clone(s))
}
