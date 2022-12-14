use crate::shell::{MAX_ARRAY_SIZE};
use crate::{print, println};

const MAX_DIRS : usize = 20;
const MAX_CHILDREN : usize = 10;
pub const NAME_SIZE : usize = 16;
pub const MAX_MESSAGE_SIZE : usize = 256;
pub const LINE_END : u8 = '^' as u8;


pub struct FileSys{
    dirs_count: usize,
    dirs: [Dir; MAX_DIRS],
    curr_dir: usize,
    last_index: usize
}

#[derive(Debug, Clone, Copy)]
pub struct Dir {
    index: usize,
    name: [u8; NAME_SIZE],
    parent_index: usize,
    child_count: usize,
    child_indexes: [usize; MAX_CHILDREN]
}


impl FileSys{

    pub fn new(array: [u8; MAX_ARRAY_SIZE]) -> FileSys {
        let directories_count : usize = array[0] as usize;
        if directories_count == 0{
            FileSys::new_default()
        }
        else{
            deserialize(array)
        }
    }

    pub fn new_default() -> FileSys{
        let mut directories = [Dir::new_root(); MAX_DIRS];

        directories[1] = directories[0].new_child(1, str_name_to_arr("first dir"));
        directories[2] = directories[0].new_child(2, str_name_to_arr("second dir"));
        directories[3] = directories[2].new_child(3, str_name_to_arr("third dir"));
        directories[4] = directories[3].new_child(4, str_name_to_arr("forth dir"));
        directories[5] = directories[0].new_child(5, str_name_to_arr("fifth dir"));
        directories[6] = directories[5].new_child(6, str_name_to_arr("sixth dir"));

        FileSys {
            dirs_count: 7,
            dirs: directories,
            curr_dir: 0,
            last_index: 6
        }
    }

    pub fn get_arr(self) -> [u8; MAX_ARRAY_SIZE] {
        serialize(self)
    }

    pub fn recognize_command(&mut self, 
        text_left: [u8; NAME_SIZE], text_right: [u8; NAME_SIZE]){
        
        //cur dir
        if compare_text_arrs(text_left, str_name_to_arr("dir_curr")){
            self.cmd_curr_dir();
        }
        //make dir
        else if compare_text_arrs(text_left, str_name_to_arr("dir_make")){
            self.cmd_make_dir(text_right);
        }
        //change dir
        else if compare_text_arrs(text_left, str_name_to_arr("dir_go")){
            if compare_text_arrs(text_right, str_name_to_arr(".")){
                self.cmd_dir_back();
            }
            else{
                self.cmd_change_dir(text_right);
            }
        }
        //remove dir
        else if compare_text_arrs(text_left, str_name_to_arr("dir_rm")){
            self.cmd_remove_dir(text_right);
        }
        //dir tree
        else if compare_text_arrs(text_left, str_name_to_arr("dir_tree")){
            self.cmd_get_dir_tree();
        }
        //error message
        else{
            print!("[failed] Command: '");
            print_name(text_left, false, false);
            print!("' is not supported!\n");
        }
    }

    // commands

    fn cmd_remove_dir(&mut self, name: [u8; NAME_SIZE]){
            
        let (name_found_result, delete_dir_index) = self.find_child_by_name(name);
        if name_found_result{
            
            let delete_dir_name = self.dirs[self.find_dir_index(delete_dir_index)].name;
            self.cascade_dir_delete(delete_dir_index);

            print!("[ok] Deleted dir: '");
            print_name(delete_dir_name, false, false);
            print!("'\n");
        }
        else{
            print!("[failed] Cannot find directory.\n");
        }
    }

    fn cmd_change_dir(&mut self, name: [u8; NAME_SIZE]){

        let (name_found_result, new_current_dir_index) = self.find_child_by_name(name);
        if name_found_result{
            
            self.curr_dir = new_current_dir_index;
            let curr_dir = self.dirs[self.find_dir_index(self.curr_dir)];

            print!("[ok] Changed current dir to: '");
            print_name(curr_dir.name, false, false);
            print!("'\n");
        }
        else{
            print!("[failed] Cannot find directory.\n");
        }
    }

    fn cmd_dir_back(&mut self){
            
        let mut curr_dir = self.dirs[self.find_dir_index(self.curr_dir)];

        if curr_dir.index != curr_dir.parent_index{
            
            self.curr_dir = curr_dir.parent_index;
            curr_dir = self.dirs[self.find_dir_index(self.curr_dir)];

            print!("[ok] Changed current dir to: '");
            print_name(curr_dir.name, false, false);
            print!("'\n");
        }
        else{
            print!("[failed] There is no parent in the: '");
            print_name(curr_dir.name, false, false);
            print!("'\n");
        }
    }

    fn cmd_make_dir(&mut self, name: [u8; NAME_SIZE]){
        
        if is_name_empty(name){
            print!("[failed] Cannot create directory with empty name!\n");
            return;
        }

        let (name_found_result, dir_with_same_name_index) = self.find_child_by_name(name);
        if name_found_result{
            print!("[failed] Directory with same name already exist!\n");
            return;
        }

        let curr_dir_index = self.find_dir_index(self.curr_dir);
        let curr_dir = self.dirs[curr_dir_index];
        if (curr_dir.child_count == MAX_CHILDREN){
            print!("[failed] Cannot create more then {} of children in directory!\n", MAX_CHILDREN);
            return;
        }
        if (self.dirs_count == MAX_DIRS){
            print!("[failed] Cannot create more then {} directories!\n", MAX_DIRS);
            return;
        }

        self.last_index += 1;
        self.dirs_count += 1;
        self.dirs[self.dirs_count - 1] = self.dirs[curr_dir_index].new_child(self.last_index, name);

        print!("[ok] Created new dir: '");
        print_name(self.dirs[self.dirs_count - 1].name, false, false);
        print!("'\n");
    }

    fn cmd_curr_dir(&mut self){
        
        let mut curr_dir = self.dirs[self.find_dir_index(self.curr_dir)];
        print_name(curr_dir.name, true, true);
    }

    fn cmd_get_dir_tree(&mut self){
        
        let mut curr_dir = self.dirs[self.find_dir_index(self.curr_dir)];
        print_name(curr_dir.name, true, true);

        self.recursive_directories_name_reading(curr_dir, 1);
    }

    // auxiliary

    fn recursive_directories_name_reading(&mut self, parent_dir: Dir, nesting: usize){

        for i in 0..parent_dir.child_count{
            let dir_index = self.find_dir_index(parent_dir.child_indexes[i]);
            let mut child = self.dirs[dir_index];
            
            for i in 0..nesting{   
                let backspaces_count = 4;
                for j in 0..backspaces_count{
                    print!(" ");
                }
            }
            print_name(child.name, true, true);
            self.recursive_directories_name_reading(child, nesting + 1);
        }
    }

    fn find_dir_index(&mut self, index: usize) -> usize{
        for i in 0..self.dirs_count{
            if self.dirs[i].index == index{
                return i;
            }
        }
        return 0;
    }

    fn find_child_by_name(&mut self, name: [u8; NAME_SIZE]) -> (bool, usize){
        
        let mut curr_dir = self.dirs[self.find_dir_index(self.curr_dir)];
        for i in 0..curr_dir.child_count{
            let index = curr_dir.child_indexes[i];
            let dir_index = self.find_dir_index(index);
            let mut child = self.dirs[dir_index];

            if compare_text_arrs(child.name, name){
                return (true, index);
            }
        }
        return (false, 0);
    }

    fn cascade_dir_delete(&mut self, dir_index: usize){
        
        let mut dir = self.dirs[self.find_dir_index(dir_index)];
        //print!("\ndirectory to remove: {}", dir_index);

        for i in (0..dir.child_count).rev(){
            let mut child = self.dirs[self.find_dir_index(dir.child_indexes[i])];            
            self.cascade_dir_delete(child.index);            
        }

        // remove directory
        let index = self.find_dir_index(dir_index);
        dir = self.dirs[index];
        self.dirs[self.find_dir_index(dir.parent_index)]
            .remove_child_index_from_list(dir_index);

        self.dirs_count -= 1;
        for i in index..self.dirs_count{
            self.dirs[i] = self.dirs[i + 1];
        }
        self.dirs[self.dirs_count] = Dir::new_root();
    }
}

impl Dir{

    pub fn new_root() -> Dir {
        return Dir {
            index: 0,
            name: str_name_to_arr
            ("root"),
            parent_index: 0,
            child_count: 0,
            child_indexes: [0; MAX_CHILDREN]
        };
    }

    pub fn new_child(&mut self, dir_index: usize, dir_name: [u8; NAME_SIZE]) -> Dir {
        self.child_indexes[self.child_count] = dir_index;
        self.child_count += 1;

        return Dir {
            index: dir_index,
            name: dir_name,
            parent_index: self.index,
            child_count: 0,
            child_indexes: [0; MAX_CHILDREN]
        };
    }

    pub fn remove_child_index_from_list(&mut self, child_index: usize){
        let mut flag : bool = false;
        for i in 0..self.child_count{
            if self.child_indexes[i] == child_index{
                flag = true;
            }
            else if flag{
                self.child_indexes[i - 1] = self.child_indexes[i];
            }
        }
        if flag{
            self.child_indexes[self.child_count - 1] = 0;
            self.child_count -= 1;
            //print!("\nremains children: {}", self.child_count);
        }
    }
}

fn serialize(file_system : FileSys) -> [u8; MAX_ARRAY_SIZE]{
    let arr_len : usize = (1 + (3 + NAME_SIZE + MAX_CHILDREN) * file_system.dirs_count) as usize;
    let mut array : [u8; MAX_ARRAY_SIZE] = [0; MAX_ARRAY_SIZE];
    
    array[0] = file_system.dirs_count as u8;
    array[1] = file_system.curr_dir as u8;
    array[2] = file_system.last_index as u8;
    
    let mut arr_index = 3;
    for i in 0..file_system.dirs_count{
        array[arr_index] = file_system.dirs[i].index as u8;
        arr_index += 1;

        for j in 0..NAME_SIZE{
            array[arr_index] = file_system.dirs[i].name[j];
            arr_index += 1;
        }

        array[arr_index] = file_system.dirs[i].parent_index as u8;
        arr_index += 1;
        array[arr_index] = file_system.dirs[i].child_count as u8;
        arr_index += 1;

        for j in 0..MAX_CHILDREN{
            array[arr_index] = file_system.dirs[i].child_indexes[j] as u8;
            arr_index += 1;
        }
    }
    return array;
}

fn deserialize(array: [u8; MAX_ARRAY_SIZE]) -> FileSys{
    let dirs_count = array[0] as usize;
    let curr_dir = array[1] as usize;
    let last_index = array[2] as usize;
    let arr_len : usize = (3 + (3 + NAME_SIZE + MAX_CHILDREN) * dirs_count) as usize;
    let mut dirs = [Dir::new_root(); MAX_DIRS];


    let mut arr_index = 3;
    for i in 0..dirs_count{
        let dir_index = array[arr_index] as usize;
        arr_index += 1;
        
        let mut dir_name = [0; NAME_SIZE];
        for j in 0..NAME_SIZE{
            dir_name[j] = array[arr_index];
            arr_index += 1;
        }

        let dir_parent_index = array[arr_index] as usize;
        arr_index += 1;
        let dir_child_count = array[arr_index] as usize;
        arr_index += 1;
        
        let mut dir_children = [0; MAX_CHILDREN];
        for j in 0..MAX_CHILDREN{
            dir_children[j] = array[arr_index] as usize;
            arr_index += 1;
        }

        dirs[i] = Dir {
            index: dir_index,
            name: dir_name,
            parent_index: dir_parent_index,
            child_count: dir_child_count,
            child_indexes: dir_children
        };
    }
    
    return FileSys { dirs_count, dirs, curr_dir, last_index };
}

pub fn str_name_to_arr(text: &str) -> [u8; NAME_SIZE]{
    let mut array : [u8; NAME_SIZE] = [0; NAME_SIZE];

    let mut index : usize = 0;
    for (i, byte) in text.bytes().enumerate() {
        array[i] = byte;
        index += 1;
    }
    if index < NAME_SIZE{
        array[index] = LINE_END; 
    }
    return array;
}

pub fn compare_text_arrs(text1 : [u8; NAME_SIZE], text2: [u8; NAME_SIZE]) -> bool{
    for i in 0..NAME_SIZE{
        if text1[i] != text2[i]{
            return false;
        }
        if text1[i] == text2[i] && text1[i] == LINE_END{
            return true;
        }
    }
    return true;
}

pub fn is_name_empty(name : [u8; NAME_SIZE]) -> bool{
    for i in 0..NAME_SIZE{
        if name[i] == LINE_END{
            return true;
        }
        if name[i] != (b' ' as u8){
            return false;
        }
    }
    return true;
}

fn print_name(name: [u8; NAME_SIZE], add_backslash: bool, add_new_line: bool){
    
    if add_backslash{
        print!("/");
    }

    for i in 0..NAME_SIZE{
        if name[i] == LINE_END {
            break;
        }
        
        print!("{}", name[i] as char);
    }

    if add_new_line{
        print!("\n");
    }
}

fn str_text_to_arr(text: &str) -> [u8; MAX_MESSAGE_SIZE]{
    let mut array : [u8; MAX_MESSAGE_SIZE] = [0; MAX_MESSAGE_SIZE];

    let mut index : usize = 0;
    for (i, byte) in text.bytes().enumerate() {
        array[i] = byte;
        index += 1;
    }
    if index < MAX_MESSAGE_SIZE{
        array[index] = LINE_END; 
    }
    return array;
}