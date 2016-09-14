extern crate pty_shell_mode;
extern crate ms;

use ms::the_neko::the_neko;

use pty_shell_mode::prelude as shell;

use std::io::{self, Write};

fn main() {
  let mut shell: shell::Shell = shell::Shell::new(None).unwrap();

  while let Some(event) = shell.next() {
    if let Some(ref o) = event.is_out_text() {
      io::stdout().write(o.as_slice()).unwrap();
      io::stdout().flush().unwrap();
    }
    if let Some(ref s) = event.is_out_screen() {
     // print!("{}", s);
     //let tmp: Vec<Vec<u8>> = s.to_matrix();
     //let tmp2: Vec<Vec<u8>> = tmp.clone();
     //for i in tmp2
     //{ for j in i
     //  { print!("{{{}}}", j); }}
     //the_neko(tmp);
     // if let Some(ref sig) = event.is_signal() {
     //   println!("{}", sig);
     // }
    }
    if let Some(k) = event.is_keydown()  {
      shell.write(&[k]).unwrap();
      shell.flush().unwrap();
    }
/*    if let Some(_) = event.is_line() {
    }*/
  }

  //MOUSE OFF
  print!("\x1b[?1015l\x1b[?1002l\x1b[?1000l");
}
