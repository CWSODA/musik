pub mod fil{
    use std::path::Path;
    use std::path::PathBuf;
    use std::fs::ReadDir;
    use super::log::*;

    /*
    TODO
    - load the two file paths
    - iterate through each folder, copy if there is a new folder
    - if copy of folder, go in and copy all the new content -> or redo for nested folders
    - rmb to make a list of newly added contents
    */

    pub fn copy_over(src: &Path, target: &Path, tgt_ext: &Vec<&'static str>, 
    added_items: &mut Vec<PathBuf>, is_copy_empty: bool) -> Result<(), String>{

        if target.starts_with(src){
            return Err("Target directory within source directory".to_string());
        }

        let src_iter = get_dir_iter(src)?;
        let target_iter = get_dir_iter(target)?;

        // get list of all file name in target dir to compare which ones are new
        let files_in_target: Vec<PathBuf> = 
        target_iter.filter_map(Result::ok).map(|f| f.path()).collect();
        // dbg!(&files_in_target);

        // check which files do not exist, create new file and copy
        'outer: for src_file in src_iter.map(|x| x.unwrap()){
            let Ok(src_type) = src_file.file_type() else { return Err("Cannot get file type".to_string()) };
            match src_file.path().extension() {
                Some(extension) => {
                    // if not in tgt_ext and not a dir, ignore the file
                    if !tgt_ext.contains( &extension.to_str().expect("Cannot get ext") ) { continue 'outer }
                },
                None => {
                    // if no extension and not a directory, ignore the file
                    if !src_type.is_dir() { continue 'outer }
                }
            }

            // check if there are matching names
            for file_in_target in &files_in_target{
                if file_in_target.file_name().unwrap() == src_file.file_name(){
                    // if is same directory, repeat the copying in the directory
                    // otherwise just ignore same name files
                    if src_type.is_dir(){
                        copy_over(&src_file.path(), &file_in_target, tgt_ext, added_items, is_copy_empty)?;
                    }
                    continue 'outer;
                }
            }
            
            // different name so copy everything and files inside directories too
            // use target directory and append name of src file to path
            if src_type.is_dir(){
                create_and_copy_dir(&src_file.path(), &target.join(src_file.file_name()),
                tgt_ext, added_items, is_copy_empty)?;
            }
            else{
                create_and_copy_file(&src_file.path(), &target.join(src_file.file_name()), 
                added_items)?;
            }
        }

        // everything working fine
        Ok(())
    }

    fn get_dir_iter(dir_path: &Path) -> Result<ReadDir, &'static str>{
        match dir_path.read_dir(){
            Ok(x) => Ok(x),
            Err(_) => return Err("Failed to get dir"),
        }
    }

    fn create_and_copy_file(src: &Path, target: &Path, added_items: &mut Vec<PathBuf>) -> Result<(), &'static str> {
        // check not same path
        if src == target { return Err("Same path given, cannot copy file") }

        // create and log created file
        if let Err(_) = std::fs::File::create(target){return Err("Cannot create new file")}
        log(&format!(">> Created file: {}", target.display()));
        added_items.push(target.to_path_buf());

        // copy file
        if let Err(_) =  std::fs::copy(src, target){return Err("Cannot copy content")}

        Ok(())
    }

    // Result<bool> represents if the directory created is empty
    fn create_and_copy_dir(src: &Path, target: &Path, tgt_ext: &Vec<&'static str>,
    added_items: &mut Vec<PathBuf>, is_copy_empty: bool) -> Result<bool, String> {
        // check not same path
        if src == target { return Err("Same path given, cannot copy folder".to_string()) }

        // create new dir and log, then get iter
        if let Err(_) = std::fs::create_dir(target){ return Err("Cannot create new file".to_string()) }
        log(&format!(">> Created directory: {}", target.display()));
        added_items.push(target.to_path_buf());

        let Ok(src_dir) = get_dir_iter(src) else { 
            return Err( format!("Cannot get source directory: {}", src.display()) );
        };

        // is_delete true by default
        // makes file -> dont delete, set false
        // makes dir -> dont delete, but dir can get deleted
        // pass back if is deleted, reuse the result value and pass bool

        let mut is_empty_dir = true;

        for file in src_dir.map(|x| x.unwrap()){
            match file.path().extension() {
                Some(extension) => {
                    // if not target extension ignore
                    if !tgt_ext.contains( &extension.to_str().expect("cannot convert OsStr to str???") ) 
                    { continue }
                    
                    // target extension then create and copy file
                    create_and_copy_file(&file.path(), &target.join(file.file_name()),
                    added_items)?;
                    is_empty_dir = false;
                },
                None => {
                    // ignore if file is not a directory
                    if !file.file_type().unwrap().is_dir() { continue }

                    // create and copy dir
                    if !create_and_copy_dir(&file.path(), &target.join(file.file_name()),
                    tgt_ext, added_items, is_copy_empty)?{
                        is_empty_dir = false;
                    }
                }
            }
        }
        
        // abort if empty
        if is_empty_dir && !is_copy_empty{
            if let Err(_) = std::fs::remove_dir(target){
                return Err( format!("Unable to delete supposed empty directory: {}", target.display()));
            }
            log(&format!(">> Deleted empty directory: {}", target.display()));
            added_items.pop().expect("Cant find created dir?");

            // return true since directory is gone
            return Ok(true);
        }

        // return false since if directory was empty function would have returned true in previous line
        Ok(false)
    }

    // directory is added, then files inside
    // reversing iter allows files to be deleted first before directory 
    // so path to inner files wont be invalidated if directory is deleted
    // iter uses target clone to allow deleted files to be popped out
    // since going in reverse, the front of iter is the back of original targets
    pub fn delete_files(targets: &mut Vec<PathBuf>) -> Result<(), String> {
        for file in targets.clone().iter().rev(){
            if !file.exists(){
                return Err( format!("File or directory does not exist: {}", file.display()) );
            }

            if file.is_dir() {
                if let Err(why) = std::fs::remove_dir(file){
                    return Err( format!("Unable to delete directory: {}\n{why}", file.display()) );
                }
                targets.pop();
                log(&format!(">> Deleted directory: {}", file.display()));
            }
            else{
                if let Err(why) = std::fs::remove_file(file){
                    return Err(format!("Unable to delete file: {}\n{why}", file.display()) );
                }
                targets.pop();
                log(&format!(">> Deleted file: {}", file.display()));
            }
        }

        // success
        Ok(())
    }
}

pub mod log;

pub mod coms;