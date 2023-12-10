use crate::compression::CompressionType as CT;

pub struct Table {
    pub name: String,
    pub path: String,
    pub fields: Vec<Field>,
}

pub struct Field {
    pub name: String,
    pub compression_type: CT,
}

pub struct Database {
    pub name: String,
    pub tables: Vec<Table>,
}


// macros: create_database, create_table, create_field.
//              ^               ^           ^
//              |               |           |
//              |               -           |
//              | - alice_fs.create_dir()   |
//                              -           | - Table.create_field( Field {*} )
//                              |
//                              | - alice_fs.create_file(*)

// Database must gets data from {database_name}.conf.adb;
//
// Example of {database_name}.conf.adb:
//
// {
//      tables: [
//          {   name: string,
//              fields: [
//                          {
//                              field_name: string,
//                              compression: string,
//                          }
//                      ]
//          },
//          { ... }
//      ],
//
//
// }
