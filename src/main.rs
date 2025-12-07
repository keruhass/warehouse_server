use std::env;
use std::sync::Arc;

use anyhow::Context;
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
};
use chrono::NaiveDate;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, postgres::PgPoolOptions};

// --- СТРУКТУРЫ ДАННЫХ ---

#[derive(Debug, Deserialize)]
pub struct PeriodParams {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

#[derive(Debug, Serialize, Clone)]
pub struct SupplierName {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct SupplierBankInfo {
    pub name: String,
    pub tax_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BankSupplierCount {
    pub bank_address_city: Option<String>,
    pub supplier_count: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct MaterialAssortment {
    pub material_name: String,
    pub class_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TotalAmount {
    pub total_amount: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct InventoryValue {
    pub material_name: String,
    pub total_quantity: Option<f64>,
    pub total_value: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct SupplierShare {
    pub supplier_share: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct OrderBankInfo {
    pub bank_address_city: Option<String>,
    pub total_amount: Option<f64>,
}

// --- СОСТОЯНИЕ (State) ---

pub struct AppState {
    pub pool: PgPool,
    // Кэш: ИНН -> Имя. DashMap обеспечивает потокобезопасность
    pub supplier_cache: DashMap<String, String>,
}

// --- ХЕНДЛЕРЫ ---
// Теперь каждый хендлер возвращает: Result<Json<T>, (StatusCode, String)>
// В случае успеха: JSON. В случае ошибки: Статус + Текст ошибки.

// 1. GET /api/suppliers/by-tax/:tax_id
pub async fn get_supplier_name_by_tax(
    State(state): State<Arc<AppState>>,
    Path(tax_id): Path<String>,
) -> Result<Json<SupplierName>, (StatusCode, String)> {
    // 1. Проверяем кэш
    if let Some(name) = state.supplier_cache.get(&tax_id) {
        return Ok(Json(SupplierName { name: name.clone() }));
    }

    // 2. Идем в БД
    let result = sqlx::query_as!(
        SupplierName,
        "SELECT name FROM Suppliers WHERE tax_id = $1",
        tax_id
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?; // Конвертируем ошибку БД вручную

    match result {
        Some(supplier) => {
            // 3. Сохраняем в кэш
            state.supplier_cache.insert(tax_id, supplier.name.clone());
            Ok(Json(supplier))
        }
        None => Err((
            StatusCode::NOT_FOUND,
            format!("Поставщик с ИНН {} не найден", tax_id),
        )),
    }
}

// 2. GET /api/suppliers/by-bank-city/:city
pub async fn get_suppliers_by_bank_city(
    State(state): State<Arc<AppState>>,
    Path(city): Path<String>,
) -> Result<Json<Vec<SupplierBankInfo>>, (StatusCode, String)> {
    let suppliers = sqlx::query_as!(
        SupplierBankInfo,
        "SELECT name, tax_id FROM Suppliers WHERE bank_address_city = $1",
        city
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(suppliers))
}

// 3. GET /api/analytics/bank-supplier-count
pub async fn get_suppliers_per_bank(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<BankSupplierCount>>, (StatusCode, String)> {
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
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(counts))
}

// 4. GET /api/materials/by-group/:group_code
pub async fn get_materials_by_group(
    State(state): State<Arc<AppState>>,
    Path(group_code): Path<String>,
) -> Result<Json<Vec<MaterialAssortment>>, (StatusCode, String)> {
    let assortment = sqlx::query_as!(
        MaterialAssortment,
        "SELECT material_name, class_code FROM Material_Catalog WHERE group_code = $1",
        group_code
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(assortment))
}

// 5. GET /api/finance/total-spent?start=...&end=...
pub async fn get_total_spent_by_period(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PeriodParams>,
) -> Result<Json<TotalAmount>, (StatusCode, String)> {
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
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(result))
}

// 6. GET /api/inventory/withdrawn
pub async fn get_withdrawn_materials() -> Result<Json<Vec<String>>, (StatusCode, String)> {
    // Просто возвращаем код 501
    Err((
        StatusCode::NOT_IMPLEMENTED,
        "Метод еще не реализован".to_string(),
    ))
}

// 7. GET /api/inventory/stock-value
pub async fn get_current_inventory_value(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<InventoryValue>>, (StatusCode, String)> {
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
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(inventory))
}

// 8. GET /api/analytics/supplier-share/:supplier_id/:group_code
pub async fn get_supplier_share(
    State(state): State<Arc<AppState>>,
    Path((supplier_id, group_code)): Path<(i32, String)>,
) -> Result<Json<SupplierShare>, (StatusCode, String)> {
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
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(share))
}

// 10. GET /api/orders/:order_number/bank-info
pub async fn get_bank_info_by_order(
    State(state): State<Arc<AppState>>,
    Path(order_number): Path<i32>,
) -> Result<Json<OrderBankInfo>, (StatusCode, String)> {
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
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match info {
        Some(i) => Ok(Json(i)),
        None => Err((
            StatusCode::NOT_FOUND,
            format!("Ордер {} не найден", order_number),
        )),
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

    // Инициализируем состояние с пулом и пустым DashMap
    let app_state = Arc::new(AppState {
        pool,
        supplier_cache: DashMap::new(),
    });

    let app = Router::new()
        .route(
            "/api/suppliers/by-tax/{tax_id}",
            get(get_supplier_name_by_tax),
        )
        .route(
            "/api/suppliers/by-bank-city/{city}",
            get(get_suppliers_by_bank_city),
        )
        .route(
            "/api/analytics/bank-supplier-count",
            get(get_suppliers_per_bank),
        )
        .route(
            "/api/materials/by-group/{group_code}",
            get(get_materials_by_group),
        )
        .route("/api/finance/total-spent", get(get_total_spent_by_period))
        .route("/api/inventory/withdrawn", get(get_withdrawn_materials))
        .route(
            "/api/inventory/stock-value",
            get(get_current_inventory_value),
        )
        .route(
            "/api/analytics/supplier-share/{supplier_id}/{group_code}",
            get(get_supplier_share),
        )
        .route(
            "/api/orders/{order_number}/bank-info",
            get(get_bank_info_by_order),
        )
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
