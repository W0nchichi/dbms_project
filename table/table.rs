pub struct Table {
    pub name: String,
    pub field: HashMap<String, DataType>,
    pub columns: HashMap<String, Vec<DataType>>,
    pub select_columns: Vec<String>,
}

impl Table{
    //I looked at SQLite Code and they store their files in binary file formats.
    //I am not entirely sure if I am comfortable with messing with that first so I think
    //I'll try XML files or something first, lemme make an example of the format I'm thinking
    /*
    <employees>
        <employee>
            <id>1</id>
            <name>John Doe</name>
            <position>Manager</position>
        </employee>
        <employee>
            <id>2</id>
            <name>Jane Smith</name>
            <position>Developer</position>
        </employee>
    </employees>
     */
    //It seems pretty annoying to do though, but i'll give it a shot soon, just very tired rn
    pub fn Save(&mut self){

    }

    pub fn Load(&mut self){

    }
    //I noticed on the steel-db page they don't make these public, honestly am too tired to see
    pub fn get_table_name(&self) -> &String;
    pub fn get_columns(&self) -> &HashMap<String, Vec<DataType>>;
    pub fn get_select_columns(&self) -> &Vec<String>;

}