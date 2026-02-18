// Chain list display

#[allow(dead_code)]
pub fn list_supported_chains() {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║              SUPPORTED BLOCKCHAINS                       ║");
    println!("╠═══════════════════════════════════════════════════════════╣");
    println!("║  Name              │ Chain ID │ Symbol                   ║");
    println!("╠═══════════════════════════════════════════════════════════╣");

    let chains = vec![
        ("Ethereum", "1", "ETH"),
        ("Polygon", "137", "MATIC"),
        ("BSC", "56", "BNB"),
        ("Avalanche", "43114", "AVAX"),
        ("Arbitrum", "42161", "ETH"),
        ("Optimism", "10", "ETH"),
        ("Base", "845", "ETH"),
        ("Celo", "42220", "CELO"),
        ("Fantom", "250", "FTM"),
        ("Goerli (Testnet)", "5", "ETH"),
        ("Sepolia (Testnet)", "11155111", "ETH"),
    ];

    for (name, id, symbol) in chains {
        println!("║  {:17} │ {:8} │ {:23} ║", name, id, symbol);
    }

    println!("╚═══════════════════════════════════════════════════════════╝");
}

#[allow(dead_code)]
pub fn print_chain_selected(chain_name: &str, chain_id: &str) {
    println!(
        "\n🌐 Selected Chain: {} (ID: {})\n",
        chain_name.to_uppercase(),
        chain_id
    );
}
