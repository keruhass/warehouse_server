use anyhow::Context;
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

// --- СТРУКТУРЫ ДАННЫХ ---

#[derive(Debug, Deserialize)]
pub struct PeriodParams {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

// 1. Название поставщика по ИНН
#[derive(Debug, Serialize)]
pub struct SupplierName {
    pub name: String,
}

// 2. Поставщики, обслуживаемые одним банком
#[derive(Debug, Serialize)]
pub struct SupplierBankInfo {
    pub name: String,
    pub tax_id: Option<String>,
}

// 3. Количество поставщиков, обслуживаемых каждым банком
#[derive(Debug, Serialize)]
pub struct BankSupplierCount {
    pub bank_address_city: Option<String>,
    pub supplier_count: Option<i64>, // SQL count может вернуть None в редких случаях или если GROUP BY сложный, лучше Option
}

// 4. Ассортимент в заданной группе материалов
#[derive(Debug, Serialize)]
pub struct MaterialAssortment {
    pub material_name: String,
    pub class_code: Option<String>,
}

// 5. Сумма денег, прошедших через банки за период
#[derive(Debug, Serialize)]
pub struct TotalAmount {
    pub total_amount: Option<f64>,
}

// 7. Количество каждого материала на складе и его суммарная стоимость
#[derive(Debug, Serialize)]
pub struct InventoryValue {
    pub material_name: String,
    pub total_quantity: Option<f64>,
    pub total_value: Option<f64>,
}

// 8. Доля поставщика в поставке товаров одной группы
#[derive(Debug, Serialize)]
pub struct SupplierShare {
    pub supplier_share: Option<f64>,
}

// 9. Загруженность склада по месяцам одного года
#[derive(Debug, Serialize)]
pub struct MonthlyLoad {
    pub month: Option<f64>, // EXTRACT возвращает float
    pub monthly_value: Option<f64>,
}

// 10. Банк и сумма по ордеру
#[derive(Debug, Serialize)]
pub struct OrderBankInfo {
    pub bank_address_city: Option<String>,
    pub total_amount: Option<f64>,
}

// --- ОБРАБОТКА ОШИБОК (ANYHOW + JSON) ---

// Обертка для ошибок, чтобы реализовать IntoResponse
pub struct AppError(anyhow::Error);

// Позволяет использовать оператор `?` для приведения любых ошибок к AppError
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

// Логика превращения ошибки в HTTP ответ (JSON)
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self.0.downcast_ref::<sqlx::Error>() {
            Some(sqlx::Error::RowNotFound) => {
                (StatusCode::NOT_FOUND, "Resource not found".to_string())
            }
            Some(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".to_string(),
            ),
            None => (
                StatusCode::INTERNAL_SERVER_ERROR,
                self.0.to_string(), // Возвращаем текст ошибки anyhow (будь осторожен с чувствительными данными в проде)
            ),
        };

        let body = Json(json!({
            "status": "error",
            "message": error_message
        }));

        (status, body).into_response()
    }
}

// Тип ответа хендлеров: Успех (JSON) или Ошибка (AppError -> JSON)
type HandlerResult<T> = Result<Json<T>, AppError>;

// --- ХЕНДЛЕРЫ ---

// 1. GET /api/suppliers/by-tax/:tax_id
pub async fn get_supplier_name_by_tax(
    State(pool): State<PgPool>,
    Path(tax_id): Path<String>,
) -> HandlerResult<SupplierName> {
    let result = sqlx::query_as!(
        SupplierName,
        "SELECT name FROM Suppliers WHERE tax_id = $1",
        tax_id
    )
    .fetch_optional(&pool)
    .await?; // Используем ? благодаря AppError

    match result {
        Some(supplier) => Ok(Json(supplier)),
        None => Err(anyhow::anyhow!("Поставщик с ИНН {} не найден", tax_id).into()),
    }
}

// 2. GET /api/suppliers/by-bank-city/:city
pub async fn get_suppliers_by_bank_city(
    State(pool): State<PgPool>,
    Path(city): Path<String>,
) -> HandlerResult<Vec<SupplierBankInfo>> {
    let suppliers = sqlx::query_as!(
        SupplierBankInfo,
        "SELECT name, tax_id FROM Suppliers WHERE bank_address_city = $1",
        city
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(suppliers))
}

// 3. GET /api/analytics/bank-supplier-count
pub async fn get_suppliers_per_bank(
    State(pool): State<PgPool>,
) -> HandlerResult<Vec<BankSupplierCount>> {
    let counts = sqlx::query_as!(
        BankSupplierCount,
        r#"
        SELECT 
            bank_address_city, 
            COUNT(supplier_id) AS "supplier_count"
        FROM Suppliers 
        WHERE bank_address_city IS NOT NULL 
        GROUP BY bank_address_city
        ORDER BY "supplier_count" DESC
        "#
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(counts))
}

// 4. GET /api/materials/by-group/:group_code
pub async fn get_materials_by_group(
    State(pool): State<PgPool>,
    Path(group_code): Path<String>,
) -> HandlerResult<Vec<MaterialAssortment>> {
    let assortment = sqlx::query_as!(
        MaterialAssortment,
        "SELECT material_name, class_code FROM Material_Catalog WHERE group_code = $1",
        group_code
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(assortment))
}

// 5. GET /api/finance/total-spent?start=...&end=...
pub async fn get_total_spent_by_period(
    State(pool): State<PgPool>,
    Query(params): Query<PeriodParams>,
) -> HandlerResult<TotalAmount> {
    let result = sqlx::query_as!(
        TotalAmount,
        r#"
        SELECT SUM(quantity * unit_price)::float AS total_amount 
        FROM Storage_Units 
        WHERE date BETWEEN $1 AND $2
        "#,
        params.start,
        params.end
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(result))
}

// 6. GET /api/inventory/withdrawn (Заглушка)
pub async fn get_withdrawn_materials(
    _state: State<PgPool>,
    _query: Query<PeriodParams>,
) -> HandlerResult<Vec<String>> {
    // Явно создаем ошибку, которая превратится в JSON
    // Т.к. это NotImplemented, можно было бы сделать кастомный статус,
    // но для примера вернем 500 через anyhow или кастомную логику.

    // Для более красивого кода лучше использовать (StatusCode, Json) напрямую,
    // но требование было "через anyhow" или единообразно.
    // Сделаем так:
    Err(anyhow::anyhow!("Метод не реализован: отсутствует таблица расхода").into())
}

// 7. GET /api/inventory/stock-value
pub async fn get_current_inventory_value(
    State(pool): State<PgPool>,
) -> HandlerResult<Vec<InventoryValue>> {
    let inventory = sqlx::query_as!(
        InventoryValue,
        r#"
        SELECT 
            MC.material_name,
            SUM(SU.quantity)::float AS total_quantity,
            SUM(SU.quantity * SU.unit_price)::float AS total_value
        FROM Storage_Units SU
        JOIN Material_Catalog MC ON SU.material_id = MC.material_id
        GROUP BY MC.material_name
        ORDER BY total_value DESC
        "#,
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(inventory))
}

// 8. GET /api/analytics/supplier-share/:supplier_id/:group_code
pub async fn get_supplier_share(
    State(pool): State<PgPool>,
    Path((supplier_id, group_code)): Path<(i32, String)>,
) -> HandlerResult<SupplierShare> {
    let share = sqlx::query_as!(
        SupplierShare,
        r#"
        WITH GroupTotal AS (
            SELECT SUM(SU.quantity * SU.unit_price)::float AS total_group_value
            FROM Storage_Units SU JOIN Material_Catalog MC ON SU.material_id = MC.material_id
            WHERE MC.group_code = $2
        ),
        SupplierTotal AS (
            SELECT SUM(SU.quantity * SU.unit_price)::float AS total_supplier_value
            FROM Storage_Units SU JOIN Material_Catalog MC ON SU.material_id = MC.material_id
            WHERE SU.supplier_id = $1 AND MC.group_code = $2
        )
        SELECT 
            COALESCE(ST.total_supplier_value, 0.0) / NULLIF(GT.total_group_value, 0.0) AS supplier_share
        FROM GroupTotal GT, SupplierTotal ST
        "#,
        supplier_id,
        group_code
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(share))
}

// 9. GET /api/inventory/monthly-load/:year
// pub async fn get_monthly_load(
//    State(pool): State<PgPool>,
//    Path(year): Path<i32>,
//) -> HandlerResult<Vec<MonthlyLoad>> {
//    let load = sqlx::query_as!(
//        MonthlyLoad,
//        r#"
//        SELECT
//            EXTRACT(MONTH FROM date) AS month,
//            SUM(quantity * unit_price)::float AS monthly_value
//        FROM Storage_Units
//        WHERE EXTRACT(YEAR FROM date) = $1
//       GROUP BY month
//        ORDER BY month
//        "#,
//        year
//    )
//    .fetch_all(&pool)
//    .await?;
//
//    Ok(Json(load))
//}

// 10. GET /api/orders/:order_number/bank-info
pub async fn get_bank_info_by_order(
    State(pool): State<PgPool>,
    Path(order_number): Path<i32>,
) -> HandlerResult<OrderBankInfo> {
    let info = sqlx::query_as!(
        OrderBankInfo,
        r#"
        SELECT 
            S.bank_address_city,
            (SU.quantity * SU.unit_price)::float AS total_amount
        FROM Storage_Units SU
        JOIN Suppliers S ON SU.supplier_id = S.supplier_id 
        WHERE SU.order_number = $1
        "#,
        order_number
    )
    .fetch_optional(&pool)
    .await?;

    match info {
        Some(i) => Ok(Json(i)),
        None => Err(anyhow::anyhow!("Ордер с номером {} не найден", order_number).into()),
    }
}

// --- MAIN ---

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").context("DATABASE_URL must be set in .env file or environment")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .context("Couldn't connect to database")?;

    println!("Успешное подключение к БД");

    let app = Router::new()
        // 1. ИСПРАВЛЕНО: {tax_id}
        .route(
            "/api/suppliers/by-tax/{tax_id}",
            get(get_supplier_name_by_tax),
        )
        // 2. ИСПРАВЛЕНО: {city}
        .route(
            "/api/suppliers/by-bank-city/{city}",
            get(get_suppliers_by_bank_city),
        )
        // 3
        .route(
            "/api/analytics/bank-supplier-count",
            get(get_suppliers_per_bank),
        )
        // 4. ИСПРАВЛЕНО: {group_code}
        .route(
            "/api/materials/by-group/{group_code}",
            get(get_materials_by_group),
        )
        // 5
        .route("/api/finance/total-spent", get(get_total_spent_by_period))
        // 6 (Заглушка)
        .route("/api/inventory/withdrawn", get(get_withdrawn_materials))
        // 7
        .route(
            "/api/inventory/stock-value",
            get(get_current_inventory_value),
        )
        // 8. ИСПРАВЛЕНО: {supplier_id} и {group_code}
        .route(
            "/api/analytics/supplier-share/{supplier_id}/{group_code}",
            get(get_supplier_share),
        )
        // 9. ИСПРАВЛЕНО: {year}
        //        .route("/api/inventory/monthly-load/{year}", get(get_monthly_load))
        // 10. ИСПРАВЛЕНО: {order_number}
        .route(
            "/api/orders/{order_number}/bank-info",
            get(get_bank_info_by_order),
        )
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
