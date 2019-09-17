use super::char_categories::CharKind::*;

pub struct Accumulator {
  _item_buffer: String,
  _items: Vec<String>
}

impl Accumulator {
  pub fn new() -> Accumulator {
    Accumulator {
      _item_buffer: String::new(),
      _items: Vec::new(),
    }
  }

  pub fn get_last_added(&self) -> Option<String> {
    if self._item_buffer.len() > 0 {

      return Some(String::from(&self._item_buffer));

    } else if self._items.len() > 0 {
      
      return Some(String::from(&self._items[self._items.len() - 1]));

    }
    None
  }

  pub fn items(&self) -> Option<Vec<String>> {
    if self._items.len() > 0 {
      let collected: Vec<String> = self._items.iter().map(|x| x.to_string()).collect();
      Some(collected)
    } else {
      None
    }
  }
}