use crate::conn::MongoConnection;
use crate::err::Result;
use crate::stmt::MongoStatement;
use bson::Bson;

#[derive(Debug)]
pub struct MongoDatabases {
    // The list of all the databases
    databases_names: Vec<String>,
    // The current database index.
    current_db_index: u32,
}

// Statement for SQLTables(SQL_ALL_CATALOGS, "","").
impl MongoDatabases {
    // Create a new MongoStatement to list all the valid databases.
    // Correspond to SQLTables(SQL_ALL_CATALOGS, "","").
    // All columns except the TABLE_CAT column contain NULLs.
    // The query timeout comes from the statement attribute SQL_ATTR_QUERY_TIMEOUT. If there is a
    // timeout, the query must finish before the timeout or an error is returned.
    pub fn list_all_catalogs(_client: &MongoConnection, _query_timeout: Option<i32>) -> Self {
        unimplemented!()
    }
}

impl MongoStatement for MongoDatabases {
    // Increment current_db_index.
    // Return true if current_db_index index is < for databases_names.length.
    fn next(&mut self) -> Result<bool> {
        unimplemented!()
    }

    // Get the BSON value for the value at the given colIndex on the current row.
    fn get_value(&self, _col_index: u16) -> Result<Option<&Bson>> {
        // The mapping for col_index <-> Value will be hard-coded and handled in this function
        // 1-> databases_names[current_row_index]
        unimplemented!()
    }
}
