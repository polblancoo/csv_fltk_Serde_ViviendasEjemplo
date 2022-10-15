use std::{path::Path, fs::{File, self}, collections::HashMap, hash::Hash};
use serde::{Deserialize, Serialize};
use std::error::Error;
use csv::ReaderBuilder;

pub trait ScreenOutput {
    fn toScreen(&self) -> String;
}

#[derive(Debug, Deserialize,Serialize,Clone)]
#[serde(rename_all = "camelCase")]
pub struct vivienda{
    pub indice : String,    
    pub calle : String,
    pub numero : String,
    pub piso : String,
    pub codpostal : String,
    pub superficie :String,
    pub banos : String,
    pub habitaciones :String,
    pub tipovivienda : TipoVivienda,
}


impl ScreenOutput for vivienda {
    fn toScreen(&self) -> String {
        format!("{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}",
        self.indice,
        self.calle,
        self.numero,
        self.piso,
        self.codpostal,
        self.superficie,
        self.banos,
        self.habitaciones,
        self.tipovivienda)
    }
}
#[derive(Debug, Deserialize,Serialize,Clone)]
pub enum TipoVivienda{
    Departamento,
    Casa,

}

impl ScreenOutput for ViviendasDAO {
    fn toScreen(&self) -> String {
        format!("{:?}",self.indice)
    }
}
pub struct ViviendasDAO{
    indice : HashMap<String , vivienda>

}

impl  ViviendasDAO{
    pub fn new() -> ViviendasDAO {
        let mut p = ViviendasDAO { indice : HashMap::new() };
        p.refresh();
        p
    }

    pub fn refresh(&mut self)  {
        //lee del arcvhivo cvs
       let path_cvs =  Path::new("./src/cvs/personas.cvs");
       let data_str = fs::read_to_string(path_cvs).expect("Unable to read file. / No se encuentra el Archivo");
       let mut reader = ReaderBuilder::new().delimiter(b',').from_reader(data_str.as_bytes());
        self.indice.clear();
        
        for record in reader.deserialize::<vivienda>(){
            //self.indice.insert(a.to_string(),  record.unwrap() as vivienda);
            let casa : vivienda = record.unwrap() as vivienda;
            self.indice.insert( casa.indice.to_string() ,  casa as vivienda);
            
        }
      // println!("{:?}",self);
        println!("leyendo datos de {:?}", path_cvs.to_str() );
        println!("Datos es un ---Struct ViviendaDao(key ,Vector<viviendas>) y contiene : 
                {:?} unidades.", reader.byte_records().count());
        

    }
    pub fn eliminar_viviendas(&mut self, key : &String) -> Option<vivienda>{
        self.indice.remove(key)
    }
    pub fn guardar_vivienda(&self){
        //Guarda en cvs
        let datos = self.indice.values().cloned().collect::<Vec<vivienda>>();
        //self.save(&datos);
        let f = self.save(&datos);
         match f {
            Ok(_f)=>{
                
                println!("ok Datos Guardados")
            },
            Err(error)=>println!("fallo{}",error),
        };
    
    }
        fn save(&self, datos : &Vec<vivienda>) -> Result<() , Box<dyn Error> > {

        //let file_path = get_first_arg()?;
         let file_path =  Path::new("./src/cvs/personas.cvs");
         println!("Grabando datos en {:?}", file_path.to_str() );
        let mut wtr = csv::Writer::from_path(file_path)?;
       //let rec1 = vivienda {calle: "calle".to_string(),
        //                               numero: "numero".to_string(),
        //                               piso: "piso".to_string(),
        //                               codpostal: "codpostal".to_string(),
        //                               superficie: "superficie".to_string(),
       //                                banos: "banos".to_string(),
        //                               habitaciones: "habitaciones".to_string(),
        //                               tipovivienda: TipoVivienda::Casa};
        println!("Datos es un ---Vector<vivienda> y contiene :  {:?} unidades.", datos.len());
        
        let mut  count = 0;
        for i in datos.iter(){
            let rec1 = &datos[count];
            wtr.serialize(rec1)?;
            count += 1;
            
        }
                
        wtr.flush( )?;
         Ok(())
       
    }
    pub fn save_and_refresh(&mut self, datos: &Vec<vivienda>) {
        println!("dentro de save & refresh");
       let f = self.save(datos);
           match f{
          Ok(_f) => self.refresh(),
          Err(_error)=> println!("Error al Actualizar Registros."),
        };
          //self.refresh();
    }
    pub fn asVector(&self) -> Vec<vivienda> {
        let datos = self.indice.values().cloned().collect::<Vec<vivienda>>();
        datos
    }
    pub fn add(&mut self, p : vivienda) {
        if !self.indice.contains_key(&p.indice) {
            self.indice.insert(p.clone().indice, p);
        }
    } 

}

