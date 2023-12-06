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
    let input_file_name = "C:/programming/rust/Projects/CSVToTable/src/test_table.txt";


    println!("in the file{}",input_file_name);
    let contents = std::fs::read_to_string(input_file_name)
        .expect("could not read file");
    let rows = split_as_vec(contents,"\n");
    println!("{}",rows[2]);
}
