use std::sync::{Arc, Mutex, RwLock};
use serde::{Deserialize, Serialize};

#[derive(Serialize,Clone,Deserialize)]
pub(crate) struct Employed {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub age: String,
    pub diploma: u64,
    pub onboarded:bool,
    pub password:Option<String>
}

#[derive(Clone)]
pub(crate) struct AppState{
    next_id:Arc<Mutex<i32>>,
    pub emp_list:Arc<RwLock<Vec<Employed>>>,
}

impl AppState {
    pub fn new(employed_list: Vec<Employed>) -> Self {
        let max_id = employed_list.iter().map(|emp| emp.id).max().unwrap_or(0);
        Self {
            next_id: Arc::new(Mutex::new(max_id + 1)),
            emp_list: Arc::new(RwLock::new(employed_list)),
        }
    }
    pub fn get_id(&self) -> i32 {
        let mut next_id = self.next_id.lock().expect("mutex poisoned");
        let id = *next_id;
        *next_id += 1;
        id
    }
}

//example

pub(crate) fn load_state() -> AppState {
    let employed_list = vec![
        Employed {
            id: 1,
            firstname: "hola".to_string(),
            lastname: "hola".to_string(),
            age: "33".to_string(),
            diploma:1,
            onboarded:true,
            password:None
        },
        Employed {
            id: 2,
            firstname: "hola".to_string(),
            lastname: "hola".to_string(),
            age: "32".to_string(),
            diploma:2,
            onboarded:false,
            password:None
        },
        Employed {
            id: 3,
            firstname: "hola".to_string(),
            lastname: "hola".to_string(),
            age: "42".to_string(),
            diploma:2,
            onboarded:true,
            password:None
        },
    ];
    AppState::new(employed_list)
}

