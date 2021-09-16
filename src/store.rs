pub const TABLE_MAX_ROWS: usize = 1500;

pub struct Table {
  pub rows: Vec<Row>,
}

#[derive(Debug)]
pub struct Row {
  pub id: String,
  pub username: String,
  pub email: String,
}

impl Table {
  pub fn insert_row(&mut self, row: Row) {
    // NOTE: We are maintaining a fixed size table that cannot grow at the moment
    if self.rows.len() < self.rows.capacity() {
      return self.rows.push(row);
    }

    println!("ERR: Table is at MAX_CAPACITY={}", TABLE_MAX_ROWS);
  }

  pub fn list_rows(&self) {
    if self.rows.is_empty() {
      println!("Table is empty!");
    }

    for row in &self.rows {
      println!("{:?}", row);
    }
  }
}

impl Row {
  pub fn from(captured_groups: regex::Captures) -> Row {
    let id = Row::get_captured_value(&captured_groups, "id");
    let username = Row::get_captured_value(&captured_groups, "username");
    let email = Row::get_captured_value(&captured_groups, "email");
    return Row {
      id,
      username,
      email,
    };
  }

  fn get_captured_value(captured: &regex::Captures, name_key: &str) -> String {
    let captured_value = captured.name(name_key).unwrap().as_str();
    String::from(captured_value)
  }
}
