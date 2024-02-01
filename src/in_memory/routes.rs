use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use axum::{Router, routing::{get, patch}, extract::State, Json, http::StatusCode};
use axum::extract::Path;
use serde::{Deserialize, Serialize};
use crate::in_memory::password_generator::PasswordGenerator;
use crate::in_memory::state::{AppState, Employed};
use rand_core::OsRng;
use axum_auth::AuthBasic;

#[derive(Deserialize)]
struct NewEmployed {
    firstname: String,
    lastname: String,
    age: String,
    diploma:u64,
    onboarded:bool
}
#[derive(Serialize)]
struct JsonResponse {
    message: String,
}


pub fn rest_router() -> Router<AppState> {
    Router::new()
        .route("/employed", get(get_employed).post(create_employed))
        .route("/employed/:id",patch(update_employed).delete(delete_employed))
        .route("/list_users",get(get_onboarded_false))
}

async fn get_employed(AuthBasic((id, password)): AuthBasic, State(state): State<AppState>) -> Result<Json<Vec<Employed>>, (StatusCode,String)>  {
    if let Some(password) = password {
        if password == "" || id == ""{
            println!("User '{}' with password '{}'", id, password);
            return Err((StatusCode::BAD_REQUEST, "User is with empty password or empty id".to_string()))
        }
    } else {
        println!("User '{}' without password", id);
        return Err((StatusCode::BAD_REQUEST, "User is without password".to_string()))
    }

    Ok(Json(state.emp_list.read().expect("lock poisoned").clone()))
}

async fn get_onboarded_false(State(state): State<AppState>) -> Result<Json<Vec<Employed>>, (StatusCode,String)> {
    let employed_list = state.emp_list.write().expect("rwlock poisoned");

    let mut emp_off:Vec<Employed> = vec![];

    for elem in employed_list.clone(){
        if elem.onboarded == false {emp_off.push(elem.clone())  }
    }
    Ok(Json(emp_off))
}



async fn create_employed(State(state):State<AppState>, Json(emp):Json<NewEmployed>) -> Result<Json<Vec<Employed>>, (StatusCode,String)> {
    let mut employed_list = state.emp_list.write().expect("lock poisoned");


    let emp = Employed {
        id: state.get_id(),
        firstname: emp.firstname,
        lastname: emp.lastname,
        age: emp.age,
        diploma: emp.diploma,
        onboarded:emp.onboarded,
        password:None
    };

    match is_at_least_18_years_old(emp.age.clone()) {
        Ok(is_adult) => {
            if !is_adult {
                return Err((StatusCode::BAD_REQUEST, "Person must have at least 18 years".to_string()))
            }
        }
        Err(e) => println!("Failed to parse age: {}", e),
    }

    if emp.diploma <= 0 {
        return Err((StatusCode::BAD_REQUEST, "Person must have at least one diploma".to_string()))
    }


    employed_list.push(emp);
    Ok(Json(employed_list.clone()))
}


async fn update_employed(State(state):State<AppState>, Path(id):Path<i32>, Json(employed):Json<NewEmployed>) -> Json<Vec<Employed>> {
    let mut employed_list = state.emp_list.write().expect("rwlock poisoned");
    let mut password_generator = PasswordGenerator::new();
    let password = password_generator.generate();

    let salt = SaltString::generate(&mut OsRng);

    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Error while hashing password: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })
        .map(|hash| hash.to_string());


    if let Some(index) = employed_list.iter().position(|e| e.id == id) {
        employed_list[index].firstname = employed.firstname.clone();
        employed_list[index].lastname = employed.lastname.clone();
        employed_list[index].age = employed.age.clone();
        employed_list[index].diploma = employed.diploma.clone();
        employed_list[index].onboarded = employed.onboarded.clone();
        employed_list[index].password = Option::from(hashed_password.unwrap());
    }

    Json(employed_list.clone())
}
async fn delete_employed(State(state):State<AppState>, Path(id):Path<i32>) ->Json<Vec<Employed>> {
    let mut employed_list = state.emp_list.write().expect("rwlock poisoned");

    if let Some(index) = employed_list.iter().position(|e| e.id == id) {
        employed_list.remove(index);
    }

    Json(employed_list.clone())
}
fn is_at_least_18_years_old(age_str: String) -> Result<bool, std::num::ParseIntError> {
    let age = age_str.parse::<i32>()?;
    Ok(age >= 18)
}

