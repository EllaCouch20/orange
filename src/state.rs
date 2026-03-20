
pub struct Address;
impl Address {
    pub fn generate() -> String { "bcp1ceax843sTOhuad2lahteau29uxxTHoxalo".to_string() }
}

#[derive(Clone, Debug, Default, PartialEq)]    
pub struct BitcoinAmount {
    pub btc: String,
    pub usd: String,
}

impl BitcoinAmount {
    pub fn usd(&self) -> Option<f32> {
        self.usd.trim_start_matches('$').replace(',', "").parse::<f32>().ok()
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Transaction {
    pub address: String,
    pub amount: BitcoinAmount,
    pub is_priority: bool,
    pub fee: String,
    pub total: String,
    pub is_received: bool,
    pub txid: String,
    pub date: String,
}

impl Transaction {
    pub fn test() -> Vec<Transaction> {
        vec![
            Transaction::new(true, "$24.00", "12:28 PM"),
            Transaction::new(true, "$11.99", "Yesterday"),
            Transaction::new(false, "$23.44", "Yesterday"),
            Transaction::new(false, "$2.50", "Thursday"),
            Transaction::new(true, "$199.99", "Monday"),
            Transaction::new(false, "$38.56", "01-04-2026"),
            Transaction::new(false, "$150.00", "28-12-2025"),
        ]
    }

    pub fn new(is_received: bool, usd: &str, date: &str) -> Self {
        Transaction {
            address: "bcp1ceax843sTOhuad2lahteau29uxxTHoxalo".to_string(),
            amount: BitcoinAmount {
                btc: "0.00001234 BTC".to_string(),
                usd: usd.to_string(),
            },
            is_priority: !is_received,
            fee: "$3.00".to_string(),
            total: "$536.99".to_string(),
            is_received,
            txid: "TXID".to_string(),
            date: date.to_string(),
        }
    }
}
