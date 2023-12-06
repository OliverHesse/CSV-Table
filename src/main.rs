

#[derive(Debug)]
enum TableType {
    Int(i64),
    Str(String),
    Bool(bool),
    Float(f64),
}
#[derive(Debug)]
struct ColumnMetaData{
    column_id : u32,
    column_type : String,
}

#[derive(Debug)]
struct Table {
    rows: Vec<Vec<TableType>>,
    //here i store extra info about the table
    //allows me to store column independant data
    column_meta_data : Option<std::collections::HashMap<String,ColumnMetaData>>,
}
//supposed to be similars to pythons rstrip()

fn split_as_vec(data : String,pat: &str ) -> Vec<String>{
    data.split(pat).map(|s| s.to_string()).collect()}
fn main() {
    println!("Hello, world!");
    let mut vec = Vec::<TableType>::new();
    vec.push(TableType::Int(2));
    vec.push(TableType::Bool(false));
    vec.push(TableType::Str(String::from("This is text")));
    println!("{:?}", vec);
    let mut new_table = Table{rows:vec![vec],column_meta_data:None};
    println!("{:?}", new_table);
    let input_file_name = String::from("C:/programming/rust/Projects/CSVToTable/src/test_table.csv");


    println!("in the file{}",input_file_name);
    load_csv_into_table(input_file_name);

}

fn load_csv_into_table(file_path : String) -> Table{
    //reads the file into a string
    let contents = std::fs::read_to_string(file_path)
    .expect("could not read file");
    //splits the string into rows which are stored in a vec
    let rows = split_as_vec(contents,"\n");
    
    //here i will start iterating through the rows.
    let mut iter = rows.iter();

    //first row must be the meta data for our file
    let table_meta_data = iter.next();
    println!("{}",table_meta_data.unwrap().trim());

    //splits into individual columns
    let table_meta_data = split_as_vec(String::from(table_meta_data.unwrap().trim()), ",");
    println!("{:?}",table_meta_data);

    //if it is empty then no meta data is provided
    if table_meta_data[0] == ""{panic!("no meta data was provided");}
    //now i will loop through each piece of meta_data. and then split them.
    //they should be in the format column_name:data_type
    let mut new_column_meta_data_map = std::collections::HashMap::<String,ColumnMetaData>::new();
    
    for (i,meta_data) in table_meta_data.iter().enumerate(){
        let meta_split = split_as_vec(meta_data.clone(), ":");
        println!("{:?}",meta_split);
        if meta_split.len() > 2 || meta_split.len() < 2{
            //we expect 2 paramaters
            panic!("expected 2 params. provided {:?}",meta_split.len());
        }
        //now i know we have a valid number of params. i still need to check their presences
        if meta_split[0] == "" || meta_split[1] == ""{
            panic!("invalid column data provided missing arguments");
        }
        //now the basic validation should be done
        
        //i need to check if the data_type is valid
        //after checking i can create a new ColumnMetaData struct instance
        let column_type: String;
        match meta_split[1].as_str(){
            "int"=>{column_type = String::from(meta_split[1].clone())},
            "str"=>{column_type = String::from(meta_split[1].clone())},
            "bool"=>{column_type = String::from(meta_split[1].clone())},
            "float"=>{column_type = String::from(meta_split[1].clone())},
            _=>{panic!("unknown column type of {}",meta_split[1])}
        }
        let index =  u32::try_from(i)
        .ok()
        .expect("index is out of bounds");
        
        new_column_meta_data_map.insert(meta_split[0].clone(), ColumnMetaData{column_type,column_id:index});
        

    }
    println!("{:?}",new_column_meta_data_map);
    //temporary to provide a return value
    let vec = Vec::<TableType>::new();
    Table{rows:vec![vec],column_meta_data:None}
}