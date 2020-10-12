#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

mod other {
    use rocket::http::RawStr;
    use std::path::Path;
    use std::path::PathBuf;
    use rocket::response::NamedFile;

    // Ejemplo de una ruta dentro de un modulo
    #[get("/mundo")]
    pub fn world() -> &'static str {
        "Hola dentro de un modulo"
    }

    // Ejemplo de un path param
    #[get("/dinamico/<nombre>")]
    pub fn mundo_dinamico(nombre: &RawStr) -> String {
        format!("Hola, {}", nombre.as_str())
    }

    // Ejemplo de un recurso static
    #[get("/page/<file..>")]
    pub fn static_resource(file: PathBuf) -> Option<NamedFile> {
        NamedFile::open(Path::new("static/").join(file)).ok()
    }

    
    #[get("/multiple/<name>/<age>/<cool>")]
    pub fn multiple(name: String, age: u8, cool: bool) -> String {
        if cool {
            format!("You're a cool {} year old, {}!", age, name)
        } else {
            format!("{}, we need to talk about your coolness.", name)
        }
    }

    // Rank
    #[get("/user/<id>")]
    pub fn user(id: usize) -> String { 
        format!("Id {}",id)
    }

    #[get("/user/<id>", rank = 2)]
    pub fn user_int(id: isize) -> String { 
        format!("Id int {}",id)
    }

    #[get("/user/<id>", rank = 3)]
    pub fn user_str(id: &RawStr) -> String { 
        format!("Id str {}",id)
    }

    #[get("/hello?<name>")]
    pub fn hello(name: &RawStr) -> String {
        format!("Hello, {}!", name.as_str())
    }

    // Es opcional enviar el name
    #[get("/hello-opt?wave&<name>")]
    pub fn hello_opt(name: Option<String>) -> String {
        name.map(|name| format!("Hi, {}!", name))
            .unwrap_or_else(|| "Hello!".into())
    }

    use rocket::request::Form;
    #[derive(FromForm)]
    pub struct User {
        name: String,
        account: usize,
    }

    #[get("/item?<id>&<user..>")]
    pub fn item(id: usize, user: Form<User>) -> String { 
        format!("ID {} Name {} Account {}!", id, user.name, user.account)      
    }

    //use rocket::request::Form;

    #[derive(FromForm)]
    pub struct Task {
        complete: bool,
        description: String,
    }

    // Form url encoded
    #[post("/todo", data = "<task>")]
    pub fn todo_new(task: Form<Task>) -> String{
        format!("Ejemplo {} {}", task.complete, task.description )
    }

    use rocket_contrib::json::Json;

    // Requiere extern macro
    #[derive(Deserialize)]
    pub struct Task2 {
        description: String,
        complete: bool
    }

    #[post("/todo-json", data = "<task2>")]
    pub fn todo_json(task2: Json<Task2>) -> String {
        format!("Ejemplo JSON {} {}", task2.complete, task2.description )
    }

    use rocket::http::Status;

    #[get("/error-ex")]
    pub fn just_fail() -> Status {
        Status::NotAcceptable
    }
}

#[get("/world")]
fn index() -> &'static str {
    "Hello, world esto es una prueba!"
}



use rocket_contrib::serve::StaticFiles;

fn main() {
    // base /hello y las rutas asociadas
    rocket::ignite()
        .mount("/hello", routes![index, other::world, other::mundo_dinamico, other::static_resource, other::multiple,
            other::user, other::user_int, other::user_str, other::hello, other::hello_opt, other::item,
            other::todo_new, other::todo_json, other::just_fail])
        .mount("/public", StaticFiles::from("static/"))
        .launch();
    //rocket::ignite().mount("/public", StaticFiles::from("/static"));
}