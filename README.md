# 🦀 RustChain — Uma Blockchain simples em Rust

Este projeto é uma implementação educacional de uma **blockchain minimalista**, escrita 100% em [Rust](https://www.rust-lang.org/), com os seguintes recursos:

- Blocos encadeados com hash SHA-256
- Prova de trabalho (Proof of Work)
- Transações entre carteiras
- Assinaturas digitais com `ed25519-dalek`
- Verificação de integridade da cadeia
- Persistência da blockchain em `blockchain.json`
- **Carteiras salvas no disco** com geração e recuperação de chaves
- **Saldo por carteira** com rastreamento de transações
- Estrutura modular e extensível

---

## 🚀 Tecnologias Utilizadas

- [Rust](https://www.rust-lang.org/)
- [Cargo](https://doc.rust-lang.org/cargo/)
- `serde`, `serde_json` — serialização
- `sha2` — hashing
- `ed25519-dalek` — criptografia de chave pública
- `chrono` — timestamps
- `hex`, `rand`, `rand_core` — utilitários

---

## 🧱 Estrutura do Projeto

```

src/
├── main.rs          # Entrypoint: inicializa blockchain e transações
├── block.rs         # Define a estrutura de bloco e hashing
├── blockchain.rs    # Gerencia a cadeia e validação
├── transaction.rs   # Transações assinadas com chaves públicas
└── wallet.rs        # Geração, persistência e leitura de carteiras

wallets/
├── alice\_wallet.json  # Carteira com chave privada + pública
├── bob\_wallet.json    # Outra carteira
├── \*.pub              # Arquivos contendo só a chave pública

````

---

## ⚙️ Como Rodar o Projeto

1. Clone o repositório:

```bash
git clone https://github.com/seu-usuario/rustchain.git
cd rustchain
````

2. Compile e execute:

```bash
cargo run
```

3. Gere carteiras com:

```bash
// Exemplo no código:
let wallet = Wallet::generate();
wallet.save_to_file("alice");
```

---

## ✅ O que o projeto faz

* Gera carteiras (chaves públicas/privadas) com persistência
* Cria transações entre carteiras reais
* Assina transações com a chave privada do remetente
* Valida assinatura usando a chave pública
* Agrupa transações em blocos com **prova de trabalho**
* Adiciona os blocos à blockchain
* Salva a blockchain em disco (`blockchain.json`)
* Permite verificar o **saldo de qualquer carteira**

---

## 💰 Verificação de Saldo

Você pode verificar o saldo de uma carteira com:

```rust
let alice_wallet = Wallet::load_from_file("alice").unwrap();
let saldo = blockchain.get_balance(&alice_wallet.keypair.public);
println!("Saldo: {}", saldo);
```

---

## 🛡️ Segurança e Integridade

Cada transação contém:

* `from`: chave pública do remetente
* `to`: chave pública do destinatário
* `amount`: valor
* `signature`: assinatura digital dos dados da transação

A blockchain só é considerada válida se:

* Os hashes de cada bloco estiverem corretos
* Os `previous_hash` estiverem encadeados corretamente
* Todas as transações forem assinadas e verificadas com sucesso

---

## 📦 Exemplo de Saída

```
✅ Bloco minerado: 000abc...
🔗 Blockchain:
Block { index: 0, ... }
Block { index: 1, transactions: [Transaction { from: ..., to: ..., ... }] }
✔️ Blockchain válida? true
💰 Saldo da Alice: 100
```

---

## 🧠 Próximos Passos

* [ ] Expor API REST com Rocket ou Actix
* [x] Persistência da blockchain em JSON
* [x] Carteiras salvas com chave pública/privada
* [x] Verificação de saldo por carteira
* [ ] Pool de transações pendentes
* [ ] Interface de linha de comando (CLI)
* [ ] Suporte a rede P2P com libp2p

---

## 📜 Licença

Este projeto é distribuído sob a licença MIT. Livre para estudar, modificar e distribuir.

---

## 👨‍💻 **Autor**

Desenvolvido por **[Roberto Lima](https://github.com/robertolima-dev)** 🚀✨

---

## 💬 **Contato**

* 📧 **Email**: [robertolima.izphera@gmail.com](mailto:robertolima.izphera@gmail.com)
* 💼 **LinkedIn**: [Roberto Lima](https://www.linkedin.com/in/roberto-lima-01/)
* 💼 **Website**: [robertolima-developer.vercel.app](https://robertolima-developer.vercel.app/)
* 💼 **Gravatar**: [Roberto Lima](https://gravatar.com/deliciouslyautomaticf57dc92af0)

