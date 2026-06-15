use actix_web::{web, HttpResponse};
use serde_json::json;
use log::info;
use chrono::Utc;
use uuid::Uuid;

use crate::models::{ApiResponse, MarketData, TradingSignal, Pattern, BacktestResult, CorrelationPair};
use crate::state::AppState;

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "status": "healthy",
        "service": "market-maker-backend",
        "version": "0.1.0",
        "timestamp": Utc::now().to_rfc3339()
    }))
}

pub async fn get_markets(state: web::Data<AppState>) -> HttpResponse {
    info!("📊 Fetching all markets...");
    
    let markets = vec![
        MarketData {
            symbol: "BTC/USDT".to_string(),
            exchange: "Binance".to_string(),
            timeframe: "1h".to_string(),
            open: 43000.0,
            high: 43500.0,
            low: 42800.0,
            close: 43250.50,
            volume: 1250000.0,
            timestamp: Utc::now(),
        },
        MarketData {
            symbol: "ETH/USDT".to_string(),
            exchange: "Binance".to_string(),
            timeframe: "1h".to_string(),
            open: 2250.0,
            high: 2300.0,
            low: 2240.0,
            close: 2280.75,
            volume: 850000.0,
            timestamp: Utc::now(),
        },
        MarketData {
            symbol: "AAPL".to_string(),
            exchange: "NASDAQ".to_string(),
            timeframe: "1h".to_string(),
            open: 194.0,
            high: 196.0,
            low: 193.5,
            close: 195.50,
            volume: 500000.0,
            timestamp: Utc::now(),
        },
        MarketData {
            symbol: "EUR/USD".to_string(),
            exchange: "Forex".to_string(),
            timeframe: "1h".to_string(),
            open: 1.0800,
            high: 1.0870,
            low: 1.0795,
            close: 1.0850,
            volume: 2000000.0,
            timestamp: Utc::now(),
        },
    ];

    state.update_markets(markets.clone());
    
    HttpResponse::Ok().json(ApiResponse::ok(json!({
        "markets": markets,
        "count": 4
    })))
}

pub async fn get_market_by_symbol(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> HttpResponse {
    let symbol = path.into_inner();
    info!("📈 Fetching market data for {}", symbol);
    
    let markets = state.get_markets();
    if let Some(market) = markets.iter().find(|m| m.symbol == symbol) {
        HttpResponse::Ok().json(ApiResponse::ok(market.clone()))
    } else {
        HttpResponse::NotFound().json(ApiResponse::<()>::error(
            format!("Market {} not found", symbol)
        ))
    }
}

pub async fn get_patterns() -> HttpResponse {
    info!("🔍 Analyzing patterns...");
    
    let patterns = vec![
        Pattern {
            name: "Bull Flag".to_string(),
            symbol: "BTC/USDT".to_string(),
            confidence: 0.87,
            similarity: 0.92,
            timeframe: "1h".to_string(),
            description: "Bullish flag pattern detected with strong historical similarity".to_string(),
        },
        Pattern {
            name: "Double Bottom".to_string(),
            symbol: "ETH/USDT".to_string(),
            confidence: 0.79,
            similarity: 0.85,
            timeframe: "4h".to_string(),
            description: "Double bottom formation suggesting potential reversal".to_string(),
        },
        Pattern {
            name: "Head & Shoulders".to_string(),
            symbol: "AAPL".to_string(),
            confidence: 0.72,
            similarity: 0.78,
            timeframe: "1d".to_string(),
            description: "Head and shoulders pattern with bearish implications".to_string(),
        },
    ];

    HttpResponse::Ok().json(ApiResponse::ok(json!({
        "patterns": patterns,
        "count": patterns.len()
    })))
}

pub async fn analyze_pattern(body: web::Json<serde_json::Value>) -> HttpResponse {
    info!("🔬 Analyzing custom pattern...");
    
    HttpResponse::Ok().json(ApiResponse::ok(json!({
        "pattern_id": Uuid::new_v4().to_string(),
        "analysis": "Pattern analysis completed",
        "confidence": 0.85
    })))
}

pub async fn get_signals() -> HttpResponse {
    info!("📡 Generating trading signals...");
    
    let signals = vec![
        TradingSignal {
            id: Uuid::new_v4().to_string(),
            symbol: "BTC/USDT".to_string(),
            signal_type: "BUY".to_string(),
            entry_price: 43200.0,
            stop_loss: 42800.0,
            take_profit: 44000.0,
            confidence: 0.88,
            risk_reward_ratio: 1.82,
            pattern: "Bull Flag".to_string(),
            timestamp: Utc::now(),
        },
        TradingSignal {
            id: Uuid::new_v4().to_string(),
            symbol: "ETH/USDT".to_string(),
            signal_type: "SELL".to_string(),
            entry_price: 2290.0,
            stop_loss: 2350.0,
            take_profit: 2150.0,
            confidence: 0.75,
            risk_reward_ratio: 1.75,
            pattern: "Double Bottom".to_string(),
            timestamp: Utc::now(),
        },
    ];

    HttpResponse::Ok().json(ApiResponse::ok(json!({
        "signals": signals,
        "count": signals.len()
    })))
}

pub async fn generate_signal(body: web::Json<serde_json::Value>) -> HttpResponse {
    info!("⚡ Generating new signal...");
    
    let signal = TradingSignal {
        id: Uuid::new_v4().to_string(),
        symbol: body.get("symbol").and_then(|s| s.as_str()).unwrap_or("BTC/USDT").to_string(),
        signal_type: "BUY".to_string(),
        entry_price: 43200.0,
        stop_loss: 42800.0,
        take_profit: 44000.0,
        confidence: 0.86,
        risk_reward_ratio: 1.82,
        pattern: "Generated".to_string(),
        timestamp: Utc::now(),
    };

    HttpResponse::Ok().json(ApiResponse::ok(signal))
}

pub async fn get_correlations() -> HttpResponse {
    info!("🔗 Calculating cross-market correlations...");
    
    let correlations = vec![
        CorrelationPair {
            pair1: "BTC/USDT".to_string(),
            pair2: "ETH/USDT".to_string(),
            correlation: 0.89,
            timeframe: "1h".to_string(),
        },
        CorrelationPair {
            pair1: "BTC/USDT".to_string(),
            pair2: "SPY".to_string(),
            correlation: 0.65,
            timeframe: "4h".to_string(),
        },
        CorrelationPair {
            pair1: "EUR/USD".to_string(),
            pair2: "Gold".to_string(),
            correlation: 0.72,
            timeframe: "1d".to_string(),
        },
    ];

    HttpResponse::Ok().json(ApiResponse::ok(json!({
        "correlations": correlations,
        "count": correlations.len()
    })))
}

pub async fn get_correlation_matrix() -> HttpResponse {
    info!("📊 Building correlation matrix...");
    
    HttpResponse::Ok().json(ApiResponse::ok(json!({
        "matrix": [
            [1.00, 0.89, 0.65, 0.42],
            [0.89, 1.00, 0.58, 0.51],
            [0.65, 0.58, 1.00, 0.72],
            [0.42, 0.51, 0.72, 1.00]
        ],
        "symbols": ["BTC/USDT", "ETH/USDT", "EUR/USD", "Gold"]
    })))
}

pub async fn run_backtest(body: web::Json<serde_json::Value>) -> HttpResponse {
    info!("🔄 Running backtest...");
    
    let result = BacktestResult {
        id: Uuid::new_v4().to_string(),
        symbol: body.get("symbol").and_then(|s| s.as_str()).unwrap_or("BTC/USDT").to_string(),
        total_trades: 145,
        winning_trades: 98,
        losing_trades: 47,
        win_rate: 0.6759,
        total_profit: 12450.50,
        max_drawdown: 0.1234,
        sharpe_ratio: 1.65,
        roi: 0.2450,
        profit_factor: 2.15,
    };

    HttpResponse::Ok().json(ApiResponse::ok(result))
}

pub async fn get_backtest_result(path: web::Path<String>) -> HttpResponse {
    let id = path.into_inner();
    info!("📋 Fetching backtest result: {}", id);
    
    let result = BacktestResult {
        id,
        symbol: "BTC/USDT".to_string(),
        total_trades: 145,
        winning_trades: 98,
        losing_trades: 47,
        win_rate: 0.6759,
        total_profit: 12450.50,
        max_drawdown: 0.1234,
        sharpe_ratio: 1.65,
        roi: 0.2450,
        profit_factor: 2.15,
    };

    HttpResponse::Ok().json(ApiResponse::ok(result))
}

pub async fn train_model(body: web::Json<serde_json::Value>) -> HttpResponse {
    info!("🧠 Starting ML model training...");
    
    HttpResponse::Accepted().json(ApiResponse::ok(json!({
        "task_id": Uuid::new_v4().to_string(),
        "status": "training",
        "progress": 0.0,
        "eta_seconds": 300
    })))
}

pub async fn get_training_status() -> HttpResponse {
    info!("⏳ Checking training status...");
    
    HttpResponse::Ok().json(ApiResponse::ok(json!({
        "status": "training",
        "progress": 0.65,
        "eta_seconds": 120,
        "epoch": 65,
        "total_epochs": 100
    })))
}

pub async fn predict(body: web::Json<serde_json::Value>) -> HttpResponse {
    info!("🔮 Making predictions...");
    
    HttpResponse::Ok().json(ApiResponse::ok(json!({
        "predictions": [0.75, 0.82, 0.68, 0.91],
        "confidence": 0.84,
        "timestamp": Utc::now().to_rfc3339()
    })))
}
