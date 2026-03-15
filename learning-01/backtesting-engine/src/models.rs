use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Candle {
    pub timestamp: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tick {
    pub symbol: String,
    pub price: f64,
    pub timestamp: u64,
}
#[derive(Debug, Clone, PartialEq)]
pub enum OrderStatus {
    Pending, 
    Filled,
    Cancelled,
    Rejected,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub symbol: String,
    pub side: OrderSide,
    pub quantity: f64,
    pub price: f64,
    pub order_type: OrderType,
    pub id: u64,
    pub created_at: u64,
    pub updated_at: u64,
}
#[derive(Debug, Clone)]
pub struct Position {
    pub symbol: String,
    pub side: OrderSide,
    pub quantity: f64,
    pub entry_price: f64,
    pub unrealized_pnl: f64,
    pub realized_pnl: f64,
    pub leverage: f64,
}
#[derive(Debug, Clone)]
pub struct Trade {
    pub order_id: u64,
    pub symbol: String,
    pub side: OrderSide,
    pub quantity: f64,
    pub price: f64,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
pub enum OrderType {
    Market,
    Limit,
    Stop,
}

#[derive(Debug)]
pub struct EngineError {
    pub message: String,
    pub timestamp: u64,
    pub 
}