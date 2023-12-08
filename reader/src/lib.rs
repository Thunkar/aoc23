use std::{
    env,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let pwd = env!("CARGO_MANIFEST_DIR");
    println!("PWD: {}", pwd);

    let workspace_cwd = Path::new(pwd).join("../");

    let file = File::open(workspace_cwd.join(filename)).unwrap();
    Ok(io::BufReader::new(file).lines())
}
