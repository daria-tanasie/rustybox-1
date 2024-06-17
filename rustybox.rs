use std::env;       // pentru a extrage argumentele
use std::io::Error;     
use std::path::Path;        // pentru a prelua path ul unui fisier
use std::path::PathBuf;
use std::process;       // pentru erori si comenzi gresite
use std::fs;        // pentru lucrul cu fisiere
use std::os::unix::fs as other_fs;
use std::fs::File;
use std::os::unix::fs::PermissionsExt;


fn pwd() {          // functie comanda pwd
    let dir = env::current_dir();

    match dir {
        Ok(path) => {
            let newpath = path.to_str().unwrap();
            println!("{}", newpath);
    },
        Err(_e) => panic!("error"),
    }

}

fn echo() {         // functie comanda echo
    let to_echo: Vec<String> = env::args().collect();
    let mut cnt = 0;
    let mut size = to_echo.len();
    if size < 2 {
        process::exit(-10);
    }
    if to_echo[2] == "-n" {
        size -= 3;
        for iter in &to_echo[3..] {
            print!("{}", iter);
            cnt += 1;
            if cnt != size {
                print!(" ");
            }
        }
        process::exit(0);
    } else {
        size -= 2;
        for iter in &to_echo[2..] {
            print!("{}", iter);
            cnt += 1;
            if cnt != size {
                print!(" ");
            }
        }
        println!("");
        process::exit(0);
    }
}

fn cat(args: Vec<String>) -> Result<String, Error> {        // functie comanda cat
    if args.len() == 3 {
        let file_contents = fs::read_to_string(&args[2])?;
        Ok(file_contents)
    } else {
           let mut all_contents = String::new();
        for arg in &args[2..] {
            let single_file = fs::read_to_string(arg)?;
            all_contents.push_str(&single_file);
        }
        Ok(all_contents)
    }
}

fn mkdir(args: Vec<String>) -> Result<(), std::io::Error> {     // functie comanda mkdir
    for arg in &args[2..] {
        fs::create_dir(arg)?;
    }
    Ok(())
}

fn mv(args: Vec<String>) -> Result<(), Error> {     // functie comanda echo
    let source = &args[2];
    let destination = &args[3];
    fs::rename(source, destination)?;
    Ok(())
}

fn ln(args: Vec<String>) -> Result<(), Error> {     // functie comanda ln
    if args[2] == "-s" || args[2] == "--symbolic" {
        let source_s = &args[3];
        let link_name_s = &args[4];
        other_fs::symlink(source_s, link_name_s)?;
    }else if args.len() == 4 {
        let source = &args[2];
        let link_name = &args[3];
        fs::hard_link(source, link_name)?;
    }
    Ok(())
}

fn rmdir(args: Vec<String>) -> Result<(), Error> {      // functie comanda rmdir
    if args[2] == "-s" || args[2] == "--symbolic" {
        let source_s = &args[3];
        let link_name_s = &args[4];
        other_fs::symlink(source_s, link_name_s)?;
    }else { 
        for arg in &args[2..] {
            fs::remove_dir(arg)?;
        }
    }
    Ok(())
}

fn rm(args: Vec<String>) -> Result<(), Error> {         // functie comanda rm
    let mut cnt = 0;

    let flag = &args[2];

    match flag.as_str() {
        "-d" | "--dir" => {
            if args[3] == "-r" || args[3] == "--recursive" || args[3] == "-R" {
                for arg in &args[4..] {
                    let path = Path::new(arg);
                    if path.is_dir() {
                        fs::remove_dir_all(arg)?;
                     } else {
                         fs::remove_file(arg)?;
                     }
                }
            } else {
                for arg in &args[3..] {
                    fs::remove_dir(arg)?;
                }
            }
        }

        "-R" | "-r" | "--recursive" => {
            if args[3] == "-d" || args[3] == "--dir" {
                for arg in &args[4..] {
                    let path = Path::new(arg);
                    if path.is_dir() {
                        fs::remove_dir_all(arg)?;
                     } else {
                         fs::remove_file(arg)?;
                     }
                }
            } else {
                for arg in &args[3..] {
                    let path = Path::new(arg);
                    if path.is_dir() {
                        fs::remove_dir_all(arg)?;
                    } else {
                        fs::remove_file(arg)?;
                    }
                }
            }
        }

        _ => {
            for arg in &args[2..] {
                let path = Path::new(arg);
                if path.is_dir() {
                   cnt +=1;
                } else {
                    fs::remove_file(arg)?;
                }
            }    
            if cnt > 0 {
                process::exit(-70);
            }    
        }
    }

    Ok(())
}

fn ls(args: Vec<String>) {          // functie comanda ls
    
    if args.len() > 2 {
        let flags = &args[2];

        if args.len() == 3 && (flags != "-a" || flags != "--all" || flags != "R" || flags != "--recursive"){
            let dir = &args[2];
            let path = Path::new(&args[2]);
            if path.is_dir() {
                let contents = fs::read_dir(dir).unwrap();

                for content in contents {
                    let name = content.unwrap().file_name();
                    let name2 = name.to_string_lossy();
                    if name2.chars().nth(0) != Some('.') {
                        println!("{}", &name2);
                    }
                }
                return
            } else {
                println!("{}", dir);
            }
        }

        match flags.as_str() {
            "-a" | "--all" => {
                let dir;
                if args.len() == 3 {
                    dir = ".";
                } else {
                    if args[3] == "-R" || args[3] == "--recursive" {
                        let directory = Path::new(&args[4]);
                        ls_rec(directory, true);
                        return
                    }
                    dir = &args[3];
                }
                let contents = fs::read_dir(dir).unwrap();

                println!(".");
                println!("..");

                for content in contents {
                    let name = content.unwrap().file_name();
                    let name2 = name.to_string_lossy();
                        println!("{}", &name2);
                }
            }

            "-R" | "--recursive" => {
                let dir_or_flag = &args[3];
                match dir_or_flag.as_str() {
                    "-a" | "--all" => {
                        let dir = Path::new(&args[4]);
                        ls_rec(dir, true);
                    }
                    _=> {
                        let dir = Path::new(&args[3]);
                        ls_rec(dir, false);
                    }
                }
            }

            _=> {
                    process::exit(-80);
                }
            }
        } else {
            let dir = ".";
            let contents = fs::read_dir(dir).unwrap();

            for content in contents {
                let name = content.unwrap().file_name();
                let name2 = name.to_string_lossy();
                if name2.chars().nth(0) != Some('.') {
                    println!("{}", &name2);
                }
            }
        }

}

fn ls_rec(dir: &Path, tip: bool) {      // functie comanda ls --recursive/-R
    if dir.is_dir() {
        let curr = Path::new(&dir);
        if tip {
            println!(".");
            println!("..")
        }
        println!("{}:", curr.display());
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let name = path.file_name().unwrap();
            let name2 = name.to_string_lossy();
            println!("{}", &name2);
            ls_rec(&path, tip);
        }
    }
}

fn touch(args: Vec<String>) {       // functie comanda touch
    if args[2].chars().nth(0) != Some('-') {
        let name = &args[2];
        match fs::metadata(name) {
            Ok(_t) => {
                let _file = File::create(name);
                let _f = File::open(name);
                let _content = fs::read_to_string(name);
                return
            }
            Err(_e) => {    
                let _file = File::create(name);
                return
            }
        }
    }

    let flags = &args[2];

    match flags.as_str() {
        "-a" => {
            let file = &args[3];
            match File::open(file) {
                Ok(_t) => {
                    let mut _f = File::open(file);
                    let _content = fs::read_to_string(file);
                    return
                }

                Err(_e) => {
                    return
                }
            }
            
        }
        "-c" | "--no-create" => {
            return
        }
        "-m" => {
            let name = &args[3];
            let mut _file = File::create(name);
        }
        _=> {
            process::exit(-100);
        }
    }
}

fn cp(args: Vec<String>) -> Result<(), Error> {         // functie comanda cp
    let option = &args[2];

    match option.as_str() {
        "-r" | "-R" | "--recursive" => {
            let source = Path::new(&args[3]);
            let destination = Path::new(&args[4]);
            if source.is_dir() {
                let to_check = &args[4];
                if find_nr_char(to_check.as_str(), '/') != 0 {
                    let initial_source = Path::new(&args[3]);
                    fs::create_dir(destination)?;
                    cp_rec_rename(initial_source, source, destination)?;
                } else {
                    cp_rec(source, destination)?;
                }    
            } else {
                if destination.is_dir() {
                    let mut dest_file = args[4].clone();
                    dest_file.push_str("/");
                    if let Some(file_name) = source.file_name(){
                        if let Some(file) = file_name.to_str() {
                            let to_push = &file;
                            dest_file.push_str(to_push);
                        }
                    }
                    File::create(&dest_file)?;
                    fs::copy(source, dest_file)?;
                    
                } else {
                    fs::copy(source, destination)?;
                }
            }
            Ok(())
        }

        _=> {
            if args.len() == 3 {
                let source = Path::new(&args[2]);
                let destination = Path::new(&args[2]);
                fs::copy(source, destination)?;
            } else {
                let source = Path::new(&args[2]);
                let destination = Path::new(&args[3]);
                if destination.is_dir() {
                    let mut dest_file = args[3].clone();
                    dest_file.push_str("/");
                    if let Some(file_name) = source.file_name(){
                        if let Some(file) = file_name.to_str() {
                            let to_push = &file;
                            dest_file.push_str(to_push);
                        }
                    }
                    File::create(&dest_file)?;
                    fs::copy(source, dest_file)?;
                    
                } else {
                    fs::copy(source, destination)?;
                }
            }
            
            Ok(())
        }

    }
}

fn cp_rec(source: &Path, destination: &Path) -> Result<(), Error> {         // functie comanda cp -r/-R/--recursive
    if source.is_dir() {
        for entry in fs::read_dir(source)? {
            let mut p_buf = PathBuf::from(destination);
            p_buf.push(source);
            fs::create_dir(p_buf)?;
            let entry = entry?;
            let path = entry.path();
            cp_rec(&path, destination)?;
        }
    } else {
        let mut p_buf = PathBuf::from(destination);
        p_buf.push(source);
        File::create(&p_buf)?;
        fs::copy(source, p_buf)?;
    }
    Ok(())
}

fn cp_rec_rename(initial_source: &Path, source: &Path, destination: &Path) -> Result<(), Error> {           // functie comanda cp recursive daca tb redenumit directorul
    for entry in fs::read_dir(source)? {
        let mut p_buf = PathBuf::from(destination);
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().unwrap();
            p_buf.push(name);
            fs::create_dir(p_buf)?;
            cp_rec_rename(initial_source, &path, destination)?;
        } else {
            let base = PathBuf::from(initial_source);
            let name = path.strip_prefix(base).unwrap();
            p_buf.push(name);
            File::create(&p_buf)?;
            fs::copy(path, p_buf)?;
        }
    }
    Ok(())
}

fn chmod(args: Vec<String>) ->  Result<(), Error> {         // functie comanda chmod
    let file = Path::new(&args[3]);
    let type_of_perm = &args[2];
    let users_and_perm = &args[2];
    if type_of_perm.chars().nth(0) == Some('-') {
        println!("Invalid command");
        process::exit(-1);
    }

    let metadata = file.metadata()?;
    let all_perms = metadata.permissions().mode();
    let mask = 0b111111111;
    let mut init_perms = all_perms & mask;
    let perms = find_perms(&users_and_perm);
    let mut perm_number = get_perm_number(perms);

    if find_char(type_of_perm, '+') {
        if find_char(&users_and_perm, 'a') {
            init_perms = init_perms | perm_number;
            perm_number = perm_number << 3;
            init_perms = init_perms | perm_number;
            perm_number = perm_number << 3;
            init_perms = init_perms | perm_number;
        }

        if find_char(&users_and_perm, 'u') {
            perm_number = perm_number << 6;
            init_perms = init_perms | perm_number;
            perm_number = perm_number >> 6;
        }

        if find_char(&users_and_perm, 'g') {
            perm_number = perm_number << 3;
            init_perms = init_perms | perm_number;
            perm_number = perm_number >> 3;
        }

        if find_char(&users_and_perm, 'o') {
            init_perms = init_perms | perm_number;
        }

        let permission = PermissionsExt::from_mode(init_perms);
        fs::set_permissions(file, permission)?;

    } else if find_char(&type_of_perm, '-') {
        if find_char(&users_and_perm, 'a') {
            init_perms = init_perms ^ perm_number;
            perm_number = perm_number << 3;
            init_perms = init_perms ^ perm_number;
            perm_number = perm_number << 3;
            init_perms = init_perms ^ perm_number;
        }

        if find_char(&users_and_perm, 'u') {
            perm_number = perm_number << 6;
            init_perms = init_perms ^ perm_number;
            perm_number = perm_number >> 6;
        }

        if find_char(&users_and_perm, 'g') {
            perm_number = perm_number << 3;
            init_perms = init_perms ^ perm_number;
            perm_number = perm_number >> 3;
        }

        if find_char(&users_and_perm, 'o') {
            init_perms = init_perms ^ perm_number;
        }

        let permission = PermissionsExt::from_mode(init_perms);
        fs::set_permissions(file, permission)?;
    } else {
        let nmb = u32::from_str_radix(type_of_perm, 8).unwrap();
        let permission = PermissionsExt::from_mode(nmb);
        fs::set_permissions(file, permission)?;
    }
    Ok(())
}

fn find_char(string: &str, sign: char) -> bool {        // functie pentru a gasi un caracter intr un string
    for char in string.chars() {
        if char == sign {
            return true
        }
    }
    return false
}

fn find_nr_char(string: &str, sign: char) -> i32 {          // functie pt a gasi cate anumite caractere se afla in string
    let mut nr = 0;
    for char in string.chars() {
        if char == sign {
            nr += 1;
        }
    }
    return nr
}

fn find_perms(string: &String) -> &str {        // functie pentru separa tipul de permisiuni de restul stringului
    let bytes = string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'-' || item == b'+' {
            return &string[i+1..];
        }
    }

    &string[..]
}

fn get_perm_number(string: &str) -> u32 {       // functie pentru a returna tipul de permisiune in format binar pentru a fi setate
    let perm_number = match string {
        "x" => 0b001,
        "w" => 0b010,
        "wx" => 0b011,
        "r" => 0b100,
        "rx" => 0b101,
        "rw" => 0b110,
        "rwx" => 0b111,
        _=> 0b000,
    };
    perm_number
}

fn main() {

    // TODO 1: Read the command line arguments

    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    // TODO 2: If the first argument is pwd, call pwd()

    match command.as_str() {
        "pwd" => {
            pwd()
        }    

        "echo" => {
            echo();
        }

        "ls" => {
            ls(args);
        }

        "cat" => {
            match cat(args) {
                Ok(file_contents) => {
                    print!("{file_contents}");
                }
                Err(_error) => {
                    process::exit(-20);
                }
            }
        }

        "mkdir" => {
            match mkdir(args) {
                Ok(_t) => {

                }
                Err(_error) => {
                    process::exit(-30);
                }
            }
        }

        "mv" => {
            match mv(args) {
                Ok(_t) => {

                }
                Err(_error) => {
                    process::exit(-40);
                }
            }
        }

        "ln" => {
            if args[2] != "-s" && args[2] != "--symbolic" {
                if args.len() != 4 {
                    println!("Invalid command");
                    process::exit(-1);
                }    
            }

            match ln(args) {
                Ok(_t) => {
                    
                }
                Err(_error) => {
                    process::exit(-50);
                }
            }
        }

        "rmdir" => {
            match rmdir(args) {
                Ok(_t) => {

                }
                Err(_error) => {
                    process::exit(-60);
                }
            }
        }

        "rm" => {
            let option = &args[2];

            if option.chars().nth(0) == Some('-') {
                if args.len() < 4 {   
                        println!("Invalid command");
                        process::exit(-1);
                }
            }

            match rm(args) {
                Ok(_t) => {

                }
                Err(_error) => {
                    process::exit(-70);
                }
            }
        }

        "touch" => {
            touch(args);
        }

        "cp" => {
            match cp(args) {
                Ok(_t) => {

                }
                Err(_e) => {
                    process::exit(-90);
                }
            }
        }

        "chmod" => {
            match chmod(args) {
                Ok(_t) => {

                }
                Err(_e) => {
                    process::exit(-25);
                }
            }

        }

        _ => {
            println!("Invalid command");
            process::exit(-1);
        }
    }
}