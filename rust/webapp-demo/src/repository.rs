use crate::domain::model;

pub fn item_find_all() -> [model::Item; 2] {
  let items = [
      model::Item {
          id: 1,
          name: "item".to_string(),
      },
      model::Item {
          id: 2,
          name: "item".to_string(),
      },
  ];
  return items
}

pub fn item_find_by_id(id: u32) -> model::Item {
  let item = model::Item {
    id: id,
    name: "item".to_string(),
  };
  return item
}
