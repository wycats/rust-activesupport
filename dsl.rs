use TimeChange;

pub trait Period {
  fn seconds(&self) -> TimeChange;
  fn minutes(&self) -> TimeChange;
  fn hours(&self) -> TimeChange;
  fn days(&self) -> TimeChange;
  fn weeks(&self) -> TimeChange;
  fn months(&self) -> TimeChange;
  fn years(&self) -> TimeChange;
}

impl Period for uint {
  fn seconds(&self) -> TimeChange {
    TimeChange::new().seconds(*self as f32)
  }

  fn minutes(&self) -> TimeChange {
    TimeChange::new().minutes(*self as f32)
  }

  fn hours(&self) -> TimeChange {
    TimeChange::new().hours(*self as f32)
  }

  fn days(&self) -> TimeChange {
    TimeChange::new().days(*self as f32)
  }

  fn weeks(&self) -> TimeChange {
    TimeChange::new().weeks(*self as f32)
  }

  fn months(&self) -> TimeChange {
    TimeChange::new().months(*self as f32)
  }

  fn years(&self) -> TimeChange {
    TimeChange::new().years(*self as f32)
  }
}
