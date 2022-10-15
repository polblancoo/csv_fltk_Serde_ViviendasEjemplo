//use std::string;
//use Vista;
use crate::entidad::ViviendasDAO;

mod entidad;
mod Vista;
fn main() {
    println!("Construyendo  VIVIENDAS");
    println!("----------------------"); 
    let mut  vivi = ViviendasDAO::new();
    let mut gui = Vista::GUI::new();
   gui.build();
   gui.show();  
   
}
