use std::io::SeekFrom;

use fltk::{
    app::{self, App}, enums,
    prelude::{GroupExt, WidgetExt},
    window::{self, DoubleWindow}, button::Button,
};
use fltk_table::{SmartTable, TableOpts};

use fltk::{app::*, browser::*, button::*, enums::*, input::*, prelude::*, window::*};
//use serde::__private::de;

const WIDGET_WIDTH: i32 = 70;
const WIDGET_HEIGHT: i32 = 20;
const WIDGET_PADDING: i32 = 10;

#[derive(Clone, Copy)]
enum Message {
    Create,
    Update,
    Delete,
    Select,
    Filter,
    Save,
    Search,
}
use crate::{ViviendasDAO, entidad::{ScreenOutput, TipoVivienda}};
use crate::entidad::{vivienda};
pub struct GUI{
    app :   App,
    wind : DoubleWindow,
    sender : Sender<Message>,
    receiver : Receiver<Message>,
    model : Vec<vivienda>,
    viviDAO : ViviendasDAO,
    filter_input : Input,
    list_browser : HoldBrowser,
   //Cajas de datos 
   // ident_input : Input,
    id_input : Input,
    calle_input : Input,
    numero_input : Input,
    piso_input : Input,
    codpostal_input : Input,
    superficie_input : Input,
    bano_input : Input,
    habitaciones_input : Input,
    tipovivienda_input : Input,
//Botonera
    create_button : Button,
    update_button : Button,
    delete_button : Button,
    save_button : Button,
    search_buton : Button
}

impl GUI {
    pub fn new() -> GUI{
     
        //let mut app = app::App::default().with_scheme(app::Scheme::Gtk);
        let mut wind = Window::default().with_label("Viviendas");
        let mut app = App::default().with_scheme(app::Scheme::Plastic);
       // let mut wind = Window::new(100,100,800,500,"Viviendas Registro");
        let (sender, receiver) = channel::<Message>();

        let mut filter_input = Input::default().with_size( WIDGET_WIDTH * 4, WIDGET_HEIGHT)
        .with_pos(WIDGET_PADDING + WIDGET_WIDTH * 2, WIDGET_PADDING)
        .with_label("Filtro:");

        let mut list_browser = HoldBrowser::default()
        .with_pos(WIDGET_PADDING, filter_input.y() + filter_input.height() + WIDGET_PADDING,)
                 .with_size(WIDGET_WIDTH * 9, WIDGET_HEIGHT * 15);

        let id_input = Input::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .with_pos(list_browser.x() + list_browser.width() + WIDGET_PADDING + WIDGET_WIDTH , list_browser.y(),)
        //.with_pos(x, y)
        .with_label("Id:");

       // let id_input = Input::default()
       // .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
       // .below_of(&ident_input, WIDGET_PADDING)
       // .with_label("ID:");

        let calle_input = Input::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .below_of(&id_input, WIDGET_PADDING)
        .with_label("Calle:");

        let numero_input = Input::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .below_of(&calle_input, WIDGET_PADDING)
        .with_label("Numero :");

        let piso_input = Input::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .below_of(&numero_input, WIDGET_PADDING)
        .with_label("Piso:");
        
        let codpostal_input = Input::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .below_of(&piso_input, WIDGET_PADDING)
        .with_label("Cod.Postal:");
        
        let superficie_input = Input::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .below_of(&codpostal_input, WIDGET_PADDING)
        .with_label("Superficie:");
        
        let bano_input = Input::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .below_of(&superficie_input, WIDGET_PADDING)
        .with_label("Baños:");
        
        let habitaciones_input = Input::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .below_of(&bano_input, WIDGET_PADDING)
        .with_label("Hab.:");

        let tipovivienda_input = Input::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .below_of(&habitaciones_input, WIDGET_PADDING)
        .with_label("T. Vivienda:");

        let mut create_button = Button::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .with_pos(WIDGET_PADDING,list_browser.y() + list_browser.height() + WIDGET_PADDING,)
            .with_label("Crear");

        let mut update_button = Button::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .right_of(&create_button, WIDGET_PADDING)
            .with_label("Modificar");

        let mut delete_button = Button::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .right_of(&update_button, WIDGET_PADDING)
            .with_label("Borrar");

        let mut save_button = Button::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .right_of(&delete_button, WIDGET_PADDING)
            .with_label("Guardar");
        
            let mut search_button = Button::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .right_of(&delete_button, WIDGET_PADDING)
            .with_label("Guardar");

        let ViviendaDao = ViviendasDAO::new();
        let model = ViviendaDao.asVector();   
        GUI{
            app :  app,
            wind : wind,
            sender : sender,
            receiver: receiver,
            filter_input : filter_input,
            list_browser : list_browser,
            
            viviDAO :ViviendaDao,
            model : model,
            //ident_input : ident_input,
            id_input : id_input,
            calle_input : calle_input,
            numero_input : numero_input,
            piso_input : piso_input,
            codpostal_input : codpostal_input,
            superficie_input : superficie_input,
            bano_input : bano_input,
            habitaciones_input : habitaciones_input,
            tipovivienda_input: tipovivienda_input,
            //botonera           
            create_button : create_button,
            update_button : update_button,
            delete_button : delete_button,
            save_button :  save_button,
            search_buton : search_button
        }
        

        
    }

    
    pub fn build(&mut self) {
        self.filter_input.set_trigger(CallbackTrigger::Changed);
        self.filter_input.emit(self.sender, Message::Filter);

        self.list_browser.emit(self.sender, Message::Select);        

        self.sender.send(Message::Filter);

        self.create_button.emit(self.sender, Message::Create);

        self.update_button.emit(self.sender, Message::Update);
        self.update_button.deactivate();

        self.delete_button.emit(self.sender, Message::Delete);
        self.delete_button.deactivate();

        self.save_button.emit(self.sender, Message::Save);

        
        self.wind.set_size(
            self.id_input.width() + self.list_browser.width() + WIDGET_PADDING*11,
            self.create_button.height()*2 + self.list_browser.height() + WIDGET_PADDING*5,
        //self.wind.set_size(width, height)  
        );
        //self.wind.make_resizable(true);

        self.sender.send(Message::Filter);

    }

    fn clear_edit(&mut self) {
       // self.ident_input.set_value("");
        self.id_input.set_value("");
        self.calle_input.set_value("");
        self.numero_input.set_value("");
        self.piso_input.set_value("");
        self.habitaciones_input.set_value("");
        self.bano_input.set_value("");
        
    
    
    
    }

    pub fn show(&mut self) {
        self.wind.end();
        self.wind.show();
        while self.app.wait() {
            match self.receiver.recv() {
                Some(Message::Create) => {
                    self.model.push(vivienda { 
                        indice : self.id_input.value(),
                        calle : self.calle_input.value(),
                        numero : self.numero_input.value(),
                        piso : self.piso_input.value(),
                        codpostal : self.codpostal_input.value(),
                        superficie : self.superficie_input.value(),
                        banos : self.bano_input.value(),
                        habitaciones : self.habitaciones_input.value(),
                        tipovivienda : TipoVivienda::Casa
                    });
                    self.clear_edit();
                    self.sender.send(Message::Filter);
                                self.update_button.deactivate();
                                self.delete_button.deactivate();
                                self.create_button.deactivate();
                                self.save_button.activate();


                }
                Some(Message::Update) => {
                    if self.list_browser.value() > 0 {
                        let text_selection = self.list_browser.text(self.list_browser.value()).unwrap();
                        let search_result = self.model.iter_mut().filter(|e| {
                            return e.toScreen().eq_ignore_ascii_case(&text_selection)
                        }).next();
                        match search_result {
                            Some(persona) => {
                                persona.calle = self.calle_input.value();
                               // persona.apellidos = self.surname_input.value();
                                self.clear_edit();
                                self.sender.send(Message::Filter);
                                self.sender.send(Message::Select);
                            },
                            _ => {
                                println!("ELEMENTO NO ENCONTRADO!!!");
                            } 
                        }
                    } else {
                        println!("NO HAY ELEMENTO PARA MODIFICAR!!!");
                    }
                }
                Some(Message::Delete) => {
                    if self.list_browser.value() > 0 {
                        let text_selection = self.list_browser.text(self.list_browser.value()).unwrap();
                        let search_result = self.model.iter().enumerate().filter(|e| {
                            return e.1.toScreen().eq_ignore_ascii_case(&text_selection)
                        }).next();
                        match search_result {
                            Some((index,persona)) => {
                                self.model.remove(index);
                                self.clear_edit();
                                self.sender.send(Message::Filter);
                                self.sender.send(Message::Select);

                                self.update_button.activate();
                                self.delete_button.deactivate();
                                self.create_button.activate();
                                self.save_button.activate();
                            },
                            _ => {
                                println!("ELEMENTO NO ENCONTRADO!!!");
                            } 
                        }
                    } else {
                        println!("NO HAY ELEMENTO PARA ELIMINAR!!!");
                    }
                }
                Some(Message::Save) => {
                    self.viviDAO.save_and_refresh(&self.model);
                    self.model = self.viviDAO.asVector();
                    self.clear_edit();
                    self.sender.send(Message::Filter);
                    self.sender.send(Message::Select);

                    self.update_button.activate();
                    self.delete_button.activate();
                    self.create_button.activate();
                    self.save_button.deactivate();
                }
                Some(Message::Select) => {
                    if self.list_browser.value() == 0 {
                        self.update_button.deactivate();
                        self.delete_button.deactivate();
                    } else {
                        let text_selection = self.list_browser.text(self.list_browser.value()).unwrap();
                        let search_result = self.model.iter().filter(|e| {
                            return e.toScreen().eq_ignore_ascii_case(&text_selection)
                        }).next();

                        match search_result {
                            Some(vivienda) => {
                                self.id_input.set_value(&vivienda.indice);
                                self.calle_input.set_value(&vivienda.calle);
                                self.numero_input.set_value(&vivienda.numero);
                                self.piso_input.set_value(&vivienda.piso);
                                self.codpostal_input.set_value(&vivienda.codpostal);
                                self.superficie_input.set_value(&vivienda.superficie);
                                self.bano_input.set_value(&vivienda.banos);
                                self.habitaciones_input.set_value(&vivienda.habitaciones);
                               //self.tipovivienda_input.set_value(&vivienda.tipovivienda);
                                self.update_button.activate();
                                self.delete_button.activate();
                                self.create_button.deactivate();
                                self.save_button.deactivate();
                                
                                
                            },
                            _ => {
                                println!("ELEMENTO NO ENCONTRADO!!!");
                            } 
                        }                        
                    }
                }
                Some(Message::Filter) => {
                    let prefix = self.filter_input.value().to_lowercase();
                    let filter_empty = prefix.trim().eq_ignore_ascii_case("");
                    self.list_browser.clear();
                    for (i,p) in self.model.iter().enumerate() {
                     //------
                       println!("dentro de Vivienda {:?} y dentro del imput {:?} ", p.calle.to_ascii_lowercase().to_string().trim(),self.filter_input.value().to_ascii_lowercase().trim());
                     //-----
                       if (p.indice.eq_ignore_ascii_case(prefix.as_str()) && !filter_empty) || (filter_empty)  {
                            let item = p.toScreen(); 
                            self.list_browser.add(&item);  
                              
                        }else{

                                    //---------== self.filter_input.value().trim().to_lowercase().to_string() ------------
                                    if p.calle.to_ascii_lowercase().to_string().trim() == self.filter_input.value().to_ascii_lowercase().trim().to_string()  {
                                       let item = p.toScreen();
                                        self.list_browser.add(&item);
                                    }

                        }
                    }                                 
                    self.sender.send(Message::Select);    
                }
                None => {
                    println!("Error")
                },
                _ => {println!("Error")}
            
            }
        }
    }
    
    pub fn refresh(&mut self, data : Vec<vivienda>) {
        for (i,p) in data.iter().enumerate() {
            println!("{} {:?} ",i, p);
        }    
    }

}



