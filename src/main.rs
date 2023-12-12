use core::panic;
pub mod custom_binary_tree;


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
    println!("==Running Test Environment==");
    let input_file_name = String::from("C:/programming/rust/Projects/CSVToTable/src/test_table.csv");

    //testing my binary tree
    println!("==Creating new Tree==");
    let mut test_tree = custom_binary_tree::ColumnBinaryTree::<i64>{root:None,size:0};
    test_tree.push(4, 0);
    test_tree.push(2, 0);
    test_tree.push(7, 0);
    test_tree.push(3, 0);
    test_tree.push(1, 0);
    test_tree.print();
    println!("==End of Tree==");
    //testing my table
    let My_Table = load_csv_into_table(input_file_name);
    println!("==Column Meta Data== ");
    println!("{:?}",My_Table.column_meta_data.unwrap());
    println!("==Table Data==");
    println!("{:?}",My_Table.rows);

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
    

    //splits into individual columns
    let table_meta_data: Vec<String> = split_as_vec(String::from(table_meta_data.unwrap().trim()), ",");
    
    //if it is empty then no meta data is provided
    if table_meta_data[0] == ""{panic!("no meta data was provided");}
    //now i will loop through each piece of meta_data. and then split them.
    //they should be in the format column_name:data_type
    let mut new_column_meta_data_map = std::collections::HashMap::<String,ColumnMetaData>::new();
    let mut number_of_columns= 0;

    let mut temp_id_to_type_map = std::collections::HashMap::<usize,String>::new();
    for (i,meta_data) in table_meta_data.iter().enumerate(){
        let meta_split = split_as_vec(meta_data.clone(), ":");
        
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
        let index =  match u32::try_from(i){
            Ok(v) =>v,
            Err(_) =>panic!("error casting values"),
        };
 
        new_column_meta_data_map.insert(meta_split[0].clone(), ColumnMetaData{column_type:column_type.clone(),column_id:index});
        temp_id_to_type_map.insert(i,column_type);
        number_of_columns += 1;
    }

    //here i will construct the rest of the table
    let mut rows = Vec::<Vec<TableType>>::new();
    for (i,row) in iter.enumerate(){
        //now time to make the actual ROWS YAAAAAAAAAAAAAAAAAAAAAAAAAY
        let row_vec = split_as_vec(String::from(row.trim()), ",");
        
        //now i need to validate the row
        if row_vec.len() != number_of_columns{
            panic!("not enough columns provided in table row {}",i);
        }
        
        //now i need to loop through each column
        let mut new_row = Vec::<TableType>::new();
        for (column_i,column) in row_vec.iter().enumerate(){
            
            if column == ""{
                panic!("empty column found in table row {}",i);
            }
            //now i will check the data type of the column
            match temp_id_to_type_map.get(&column_i){
                Some(v) =>{match v.as_str() {
                    "int"=>{
                       
                        let new_value = match row_vec[column_i].parse::<i64>(){
                            Ok(n)=>{n},
                            Err(_)=>{panic!("error when parsing {} to int on line {} in column {}",row_vec[column_i],i,column_i)},
                        };
                        new_row.push(TableType::Int(new_value));
                    },
                    "str"=>{
                        
                        //no casting needed here lol
                        new_row.push(TableType::Str(row_vec[column_i].clone()));
                    },
                    "bool"=>{
                        
                        let new_value = match row_vec[column_i].parse::<bool>(){
                            Ok(n)=>{n},
                            Err(_)=>{panic!("error when parsing {} to bool on line {} in column {}",row_vec[column_i],i,column_i)},
                        };
                        new_row.push(TableType::Bool(new_value));
                    },
                    "float"=>{
                        
                        let new_value = match row_vec[column_i].parse::<f64>(){
                            Ok(n)=>{n},
                            Err(_)=>{panic!("error when parsing {} to int on line {} in column {}",row_vec[column_i],i,column_i)},
                        };
                        new_row.push(TableType::Float(new_value));
                    },
                    
                    _=>{panic!("invalid data type found on row {} in column {}",i,column_i)},
                }},
                None =>{panic!("error parsing row on line {} column id of {} was not found",i,column_i)}
            }
            
        }
        //move new row into rows
        rows.push(new_row);
        
    }

    //temporary to provide a return value
    
    Table{rows,column_meta_data:Some(new_column_meta_data_map)}
}