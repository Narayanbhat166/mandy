/*
MANDY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the "dir_is"
/// method from the "coutils"
/// crate to check if a
/// directory exists.
use coutils::dir_is;

/// Importing the method to
/// delete a directory from
/// the "coutils" crate.
use coutils::del_dir;

/// Importing the method
/// to split strings into
/// vector of strings from
/// the "coutils" crate.
use coutils::clean_split;

/// Importing Mandy's error
/// struct.
use super::errors::MandyError;

/// Gets the file name of a file in a path string plus the
/// file name's base.
pub fn get_name_base(path_name: &String) -> Vec<String> {
    let path_list: Vec<String> = clean_split(
        &path_name,
        &String::from("/")
    );
    let file_name: &String = &path_list[path_list.len()-1];
    let name_components: Vec<String> = clean_split(
        &file_name,
        &String::from(".markdown")
    );
    let name_base: String = name_components[0].clone();
    let mut file_vec: Vec<String> = Vec::new();
    file_vec.push(name_base);
    file_vec.push(file_name.to_owned());
    return file_vec;
}

/// Gets the lowest-level directory of a path string.
pub fn get_last_dir(path_name: &String) -> String {
    let path_list: Vec<String> = clean_split(
        &path_name,
        &String::from("/")
    );
    let dir_name: &String = &path_list[path_list.len()-2];
    return dir_name.to_owned();
}

/// Cleans the "dist" directory in a Mandy project.
pub fn clean_project(dir: &String) -> Result<(), MandyError> {
    let dir_to_be_cleaned: &String = &format!("{}/dist", dir);
    if dir_is(dir_to_be_cleaned){
        del_dir(&dir_to_be_cleaned);
    }
    else {
        let err_msg: &String = &format!("Built site \"{}\" not found.", dir_to_be_cleaned);
        return Err::<(), MandyError>(MandyError::new(&err_msg.to_string()));
    }
    return Ok(());
}

/// Cleans a file path and returns it as an url.
pub fn clean_url(file_path: &String, dir: &String, file_dir: &String) -> String {
    let file_name: String = get_name_base(&file_path)[0].clone();
    let mut url: String = file_dir.clone();

    url = url.replace(dir, &"");
    let mut final_url = format!("{}/{}", url, file_name );
    if url == String::from("") {
        final_url = String::from("");
    }
    else {}
    return final_url;
}

pub fn generate_api_url(file_path: &String, dir: &String, file_dir: &String) -> String {
    let mut result: String = String::from("");
    let file_name: String = get_name_base(&file_path)[0].clone();
    let json_file_name: String = format!("{}.json", &file_name);
    result = format!(
        "{}/{}",
        clean_url(file_path, dir, file_dir),
        json_file_name
    );
    return result;
}
