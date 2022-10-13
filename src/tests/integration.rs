use viviendas;

mod integration;

//#[test]
//fn funcion_refresh_ViviendasDAO() {
    
   //let mut  vivi = ViviendasDAO::new();
   //ViviendasDAO::refresh(&mut vivi);
//}
//#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn funcion_refresh_ViviendasDAO() {
       
       let mut  vivi = ViviendasDAO::new();
       ViviendasDAO::refresh(&mut vivi);
       

   }
}