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

  pub fn list_rows(&self) -> &Vec<Row> {
    if self.rows.is_empty() {
      println!("Table is empty!");
    }

    for row in &self.rows {
      println!("{:?}", row);
    }

    return &self.rows;
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_insertion_retrieval() {
    // Prepare test data
    let mut table = Table {
      rows: Vec::with_capacity(TABLE_MAX_ROWS),
    };

    let test_row = Row {
      id: String::from("uid-0137"),
      username: String::from("mihilmy"),
      email: String::from("mihilmy@gmail.com"),
    };

    // Invoke the test
    table.insert_row(test_row);
    let rows = table.list_rows();

    // Assert there is only a single row
    assert_eq!(rows.len(), 1);

    let row1 = rows.get(0).unwrap();
    assert_eq!(row1.username, "mihilmy");
    assert_eq!(row1.email, "mihilmy@gmail.com");
  }
}
