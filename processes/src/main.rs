
use std::process::Command;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
//    let pid: pid_t;
//    let mut wstatus: c_int = 0;
//    unsafe {
//        pid = fork();
//    };
//    println!("{}", pid);
//    if pid == 0 {
//        println!("This is the child process!!");
//        Ok(())
//    } else {
//        println!("This is the parent process!!");
//        println!("Child process has id: {}", pid);
//        let a: pid_t;
//        unsafe {
//            a = wait(&mut wstatus);
//        };
//        println!("Process {} completed execution", a);
//        Ok(())
//    }

    let mut list_dir = Command::new("ls");

    // Execute `ls` in the current directory of the program.
    list_dir.status().expect("process failed to execute");

    println!();

    //change `ls` to execute in the root directory.
    list_dir.current_dir("/");

    // And then execute `ls` again but in the root directory.
    list_dir.status().expect("process failed to execute");
    Ok(())
}
