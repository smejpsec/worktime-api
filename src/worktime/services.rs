use super::structs::*;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/api/worktime")]
async fn get_worktime(query: web::Query<EmployeeIDQuery>) -> impl Responder {
    let id = String::from(&query.id);
    let year = query.year;
    let month = query.month;
    let hours = 35.5;
    let details = EmployeeWorkTimeDetails {};

    let worktime = EmployeeWorkTime {
        id,
        year,
        month,
        hours,
        details,
    };
    HttpResponse::Ok().json(worktime)
}
