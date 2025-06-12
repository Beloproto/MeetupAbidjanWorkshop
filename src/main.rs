use std::io;

struct Wallet {
    balance: f64,
}

impl Wallet {
    fn new() -> Wallet {
        Wallet { balance: 0.0 }
    }

    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
            println!("✅ Dépôt de {:.2} tokens effectué. Nouveau solde : {:.2}", amount, self.balance);
        } else {
            println!("⚠️ Montant invalide !");
        }
    }

    fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
            println!("✅ Retrait de {:.2} tokens effectué. Nouveau solde : {:.2}", amount, self.balance);
        } else {
            println!("⛔ Fonds insuffisants ou montant invalide !");
        }
    }

    fn show_balance(&self) {
        println!("💰 Solde actuel : {:.2} tokens", self.balance);
    }
}

fn main() {
    let mut wallet = Wallet::new();

    loop {
        println!("\n📌 Menu - Portefeuille Crypto");
        println!("1️⃣ Voir le solde");
        println!("2️⃣ Déposer des tokens");
        println!("3️⃣ Retirer des tokens");
        println!("4️⃣ Quitter");
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("❌ Erreur de lecture");
        
        match choice.trim() {
            "1" => wallet.show_balance(),
            "2" => {
                println!("💵 Montant à déposer : ");
                let mut amount = String::new();
                io::stdin().read_line(&mut amount).expect("❌ Erreur de lecture");
                let amount: f64 = amount.trim().parse().unwrap_or(0.0);
                wallet.deposit(amount);
            },
            "3" => {
                println!("🏦 Montant à retirer : ");
                let mut amount = String::new();
                io::stdin().read_line(&mut amount).expect("❌ Erreur de lecture");
                let amount: f64 = amount.trim().parse().unwrap_or(0.0);
                wallet.withdraw(amount);
            },
            "4" => {
                println!("👋 Au revoir !");
                break;
            },
            _ => println!("⚠️ Choix invalide, réessayez !"),
        }
    }
}
