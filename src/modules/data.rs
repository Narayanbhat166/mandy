/*
MANDY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the "Entity"
/// enum from the "coutils"
/// crate to determine the type
/// of file.
use coutils::Entity;

/// Importing the "read_file"
/// method from the "coutils"
/// crate to read text files.
use coutils::read_file;

/// Importing the "FileEntry"
/// struct to work with files easier.
use coutils::FileEntry;

/// Importing the "from_str"
/// from the "serde_json"
/// crate to store JSON
/// into Rust data structures.
use serde_json::from_str;

/// Importing the "clean_split"
/// method from the "coutils"
/// method to split strings.
use coutils::clean_split;

/// Importing Rust's standard
/// "HashMap" data structure.
use std::collections::HashMap;

/// Importing Mandy's error
/// struct.
use super::errors::MandyError;

/// Importing the method store
/// information about a directory's
/// contents.
use coutils::list_dir_contents;

/// Deserializing data from the "data"
/// directory and further processing this.
pub fn deserialize_data(
    data_strings: Vec<HashMap<String, String>>
) -> Result<
HashMap<String, HashMap<String, Vec<HashMap<String, String>>>>, 
MandyError
> {
    let mut result: HashMap<String, HashMap<String, Vec<HashMap<String, String>>>> = HashMap::new();
    let mut file_hash_map: HashMap<String, Vec<HashMap<String, String>>> = HashMap::new();
    for item in data_strings.into_iter() {
        for (k,v) in item.into_iter() {
            let file_name: &String = &k;
            let json_op: Result<Vec<HashMap<String, String>>, serde_json::Error> = from_str(&v);
            match json_op {
                Ok(map) => {
                    file_hash_map.insert(file_name.to_owned(), map);
                },
                Err(e) => {
                    return Err::<HashMap<String, HashMap<String, Vec<HashMap<String, String>>>>, MandyError>(MandyError::new(&e.to_string()));
                }
            }
        }
    }
    result.insert(String::from("data"), file_hash_map);
    return Ok(result);
}

/// Storing the file contents
/// from JSON data files into
/// a list of maps.
pub fn find_data_files(
    dir: &String
) -> Vec<HashMap<String, String>> {
    let mut result: Vec<HashMap<String, String>> = Vec::new();
    let dir_items: Vec<FileEntry> = list_dir_contents(dir);
    for item in dir_items {
        if &item.file_type == &Entity::File 
            && &item.name.contains(".json")  == &true{
            let mut map: HashMap<String, String> = HashMap::new();
            let file_contents: &String = &read_file(&item.name);
            let path_list: &Vec<String> = 
                &clean_split(
                    &item.name,
                    &String::from("/")
                );
            let json_file: &String = &path_list[path_list.len()-1];
            let json_file_name_components: &Vec<String> = &clean_split(
                &json_file,
                &String::from(".json")
            );
            let template_key: &String = &json_file_name_components[0];
            map.insert(template_key.to_owned(), file_contents.to_owned());
            result.push(map);
        }
    }
    return result;
}