// Transaction display formatter

use crate::models::{TransactionDetail, TransactionReceipt};
use crate::utils::hex::parse_hex;
use crate::utils::{format_timestamp, get_status_display, wei_to_eth, wei_to_gwei};

#[allow(dead_code)]
pub struct TransactionFormatter<'a> {
    tx: &'a TransactionDetail,
    receipt: &'a Option<TransactionReceipt>,
    timestamp: Option<&'a str>,
    symbol: &'a str,
}

impl<'a> TransactionFormatter<'a> {
    #[allow(dead_code)]
    pub fn new(
        tx: &'a TransactionDetail,
        receipt: &'a Option<TransactionReceipt>,
        timestamp: Option<&'a str>,
        symbol: &'a str,
    ) -> Self {
        Self {
            tx,
            receipt,
            timestamp,
            symbol,
        }
    }

    fn calculate_fee(&self) -> f64 {
        if let Some(r) = self.receipt {
            let gas_used = r.gas_used.as_deref().map(parse_hex).unwrap_or(0);
            let gas_price = parse_hex(&self.tx.gas_price);
            (gas_used as u128 * gas_price as u128) as f64 / 1e18
        } else {
            0.0
        }
    }

    pub fn format(&self) -> String {
        let mut output = String::new();

        let status = self
            .receipt
            .as_ref()
            .map(|r| r.status.as_deref())
            .unwrap_or(None);

        let block_number = parse_hex(self.tx.block_number.as_deref().unwrap_or("0x0"));
        let tx_fee = self.calculate_fee();

        output.push_str("╔═══════════════════════════════════════════════════════════╗\n");
        output.push_str("║                 TRANSACTION DETAILS                       ║\n");
        output.push_str("╠═══════════════════════════════════════════════════════════╣\n");
        output.push_str(&format!(
            "║  {:18} │ {}  ║\n",
            "Transaction Hash:",
            crate::utils::text::truncate_and_pad(&self.tx.hash, 28)
        ));
        output.push_str(&format!(
            "║  {:18} │ {}  ║\n",
            "Status:",
            get_status_display(status)
        ));
        output.push_str(&format!("║  {:18} │ {}  ║\n", "Block:", block_number));
        output.push_str(&format!(
            "║  {:18} │ {}  ║\n",
            "Timestamp:",
            crate::utils::text::truncate_and_pad(&format_timestamp(self.timestamp), 28)
        ));
        output.push_str(&format!(
            "║  {:18} │ {}  ║\n",
            "From:",
            crate::utils::text::truncate_and_pad(&self.tx.from, 28)
        ));
        output.push_str(&format!(
            "║  {:18} │ {}  ║\n",
            "To:",
            crate::utils::text::truncate_and_pad(&self.tx.to, 28)
        ));
        output.push_str(&format!(
            "║  {:18} │ {} {} ║\n",
            "Value:",
            wei_to_eth(&self.tx.value),
            self.symbol
        ));
        output.push_str(&format!(
            "║  {:18} │ {} {} ║\n",
            "Transaction Fee:", tx_fee, self.symbol
        ));
        output.push_str(&format!(
            "║  {:18} │ {} Gwei  ║\n",
            "Gas Price:",
            wei_to_gwei(&self.tx.gas_price)
        ));
        output.push_str("╚═══════════════════════════════════════════════════════════╝");

        output
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        println!("{}", self.format());
    }
}

#[allow(dead_code)]
pub fn print_transaction_details(
    tx: &TransactionDetail,
    receipt: &Option<TransactionReceipt>,
    timestamp: Option<&str>,
    symbol: &str,
) {
    let formatter = TransactionFormatter::new(tx, receipt, timestamp, symbol);
    formatter.print();
}
