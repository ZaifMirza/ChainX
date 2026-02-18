// Address display formatter

use crate::utils::{text::truncate_and_pad, wei_to_eth};

#[allow(dead_code)]
pub struct AddressFormatter<'a> {
    address: &'a str,
    balance: &'a str,
    code: &'a str,
    nonce: u64,
    symbol: &'a str,
}

impl<'a> AddressFormatter<'a> {
    #[allow(dead_code)]
    pub fn new(
        address: &'a str,
        balance: &'a str,
        code: &'a str,
        nonce: u64,
        symbol: &'a str,
    ) -> Self {
        Self {
            address,
            balance,
            code,
            nonce,
            symbol,
        }
    }

    #[allow(dead_code)]
    pub fn format(&self) -> String {
        let mut output = String::new();
        let is_contract = !self.code.is_empty() && self.code != "0x";
        let balance_eth = wei_to_eth(self.balance);

        output.push_str("╔═══════════════════════════════════════════════════════════╗\n");
        output.push_str("║                  ADDRESS DETAILS                          ║\n");
        output.push_str("╠═══════════════════════════════════════════════════════════╣\n");
        output.push_str(&format!(
            "║  {:18} │ {}  ║\n",
            "Address:",
            truncate_and_pad(self.address, 28)
        ));
        output.push_str(&format!(
            "║  {:18} │ {}  ║\n",
            "Type:",
            if is_contract {
                "Smart Contract"
            } else {
                "EOA (Wallet)"
            }
        ));
        output.push_str(&format!(
            "║  {:18} │ {:.8} {} ║\n",
            "Balance:", balance_eth, self.symbol
        ));
        output.push_str(&format!(
            "║  {:18} │ {}  ║\n",
            "Nonce (TX Count):", self.nonce
        ));
        output.push_str("╠═══════════════════════════════════════════════════════════╣\n");
        output.push_str(&format!(
            "║  Contract Code: {}  ║\n",
            if is_contract { "Present" } else { "None (EOA)" }
        ));
        if is_contract {
            output.push_str(&format!(
                "║  Code Length: {} bytes  ║\n",
                (self.code.len() - 2) / 2
            ));
        }
        output.push_str("╚═══════════════════════════════════════════════════════════╝");

        output
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        println!("{}", self.format());
    }
}

#[allow(dead_code)]
pub fn print_address_details(address: &str, balance: &str, code: &str, nonce: u64, symbol: &str) {
    let formatter = AddressFormatter::new(address, balance, code, nonce, symbol);
    formatter.print();
}
