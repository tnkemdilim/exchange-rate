use exchange_rate::prelude::*;

fn main() {
    let price_updates = vec![
        PriceUpdate::new(
            NaiveDate::from_ymd(2015, 9, 5).and_hms(23, 56, 4),
            ExchangeType::KRAKEN,
            Currency::BTC,
            Currency::USD,
            Decimal::from_str("1.0").unwrap(),
            Decimal::from_str("0.0000009").unwrap(),
        ),
        PriceUpdate::new(
            NaiveDate::from_ymd(2015, 9, 5).and_hms(23, 56, 4),
            ExchangeType::GDAX,
            Currency::BTC,
            Currency::USD,
            Decimal::from_str("1001.0").unwrap(),
            Decimal::from_str("0.0008").unwrap(),
        ),
    ];

    let result = vec![
        ExchangeRateRequest::new(
            ExchangeType::KRAKEN,
            Currency::USD,
            ExchangeType::GDAX,
            Currency::BTC,
        ),
        ExchangeRateRequest::new(
            ExchangeType::GDAX,
            Currency::BTC,
            ExchangeType::KRAKEN,
            Currency::USD,
        ),
    ];
    let graph = RateGraph::from(price_updates);
    dbg!(&graph.full_path(&result[1]));
}
