use std::path::Path;
use crate::dbms_rust_project::config::DEFAULT_DIR;

pub struct Table {
    pub name: String,
    pub field: HashMap<String, DataType>,
    pub columns: HashMap<String, Vec<DataType>>,
    pub select_columns: Vec<String>,
}

impl Table{
    //I looked at SQLite Code and they store their files in binary file formats.
    //I am not entirely sure if I am comfortable with messing with that first so I think
    //After sleeping on it and checking Steel-db, they use something called a "columnar"
    //file format which speeds up data retreival since you can just find the column that matches
    //a condition right away. If 10GB of data is scanned in columnar, 300GB would have to be
    //Scanned in CSV/XML

    //I'm going to make a Columnar CSV file format, inspired by Matthew Rathbone's explanation
    //if this was how you would store the data in CSV
    /*
    {
    id: Integer,
    first_name: String,
    last_name: String,
    age: Integer,
    cool: Boolean,
    favorite_fruit: Array[String]
    }
    Example:
    1, Matthew, Rathbone, 19, True, ['bananas', 'apples']
    2, Joe, Bloggs, 102, True,
     */

    //We'll store it like this in our CCSV
    /*
    Field Name/Field Type/Number of Characters:[data in csv format]
    (except replace the Field Type with things from datatypes.rs[i32,f32,String])
    For example:
    ID/INT/3:1,2
    FIRST_NAME/STRING/11:Matthew,Joe
    LAST_NAME/STRING/15:Rathbone,Bloggs
    AGE/INT/6:19,102
    COOL/BOOL/3:1,1
    FAVORITE_FRUIT/ARRAY[STRING]/19:[bananas,apples],[]
     */
    pub fn new() -> Table {
        Table {
            name: String::new(),
            fields: HashMap::<String, DataType>::new(),
            columns: HashMap::<String, Vec<DataType>>::new(),
            select_columns: Vec::<String>::new(),
        }
    }

    //for transparency, I couldn't figure out how to make a file, so this was taken from steel_db
    pub fn instantiate_path_dir() {
        if !Path::new(DEFAULT_DIR).exists(){
            let result = std::fs::create_dir_all(DATA_DIR);
            result.unwrap();
        }
    }

    pub fn get_table_file_path(name: &String) -> String{
        format!("{}/{}.ccsv", DATA_DIR, name)
    }

    pub fn Save(&mut self) -> Result<(), TableErrors> {
        let in_memory_file_path = Table::get_table_file_path(&self.name);
        let file_path = Path::new(&s);
    }

    pub fn Load(&mut self){

    }
    //I noticed on the steel-db page they don't make these public, honestly am too tired to see
    pub fn get_table_name(&self) -> &String {
        return self.name;
    }
    pub fn get_columns(&self) -> &HashMap<String, Vec<DataType>>{
        return self.columns;
    }
    pub fn get_select_columns(&self) -> &Vec<String>{
        return self.select_columns;
    }

}