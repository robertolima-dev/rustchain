# 🦀 RustChain — Uma Blockchain simples em Rust

Este projeto é uma implementação educacional de uma **blockchain minimalista**, escrita 100% em [Rust](https://www.rust-lang.org/), com os seguintes recursos:

- Blocos encadeados com hash SHA-256
- Prova de trabalho (Proof of Work)
- Transações entre carteiras
- Assinaturas digitais com `ed25519-dalek`
- Verificação de integridade da cadeia
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
└── transaction.rs   # Transações assinadas com chaves públicas

Cargo.toml           # Configurações e dependências
README.md            # Este arquivo

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

---

## ✅ O que o projeto faz

* Gera três carteiras (chaves públicas)
* Cria transações entre elas
* Assina cada transação com a chave privada do remetente
* Agrupa transações em blocos com **prova de trabalho**
* Adiciona os blocos na blockchain
* Valida a integridade de toda a cadeia

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
```

---

## 🧠 Próximos Passos

* [ ] Expor API REST com Rocket ou Actix
* [ ] Adicionar carteira CLI
* [ ] Persistência da blockchain em JSON
* [ ] Pool de transações
* [ ] Suporte a rede P2P com libp2p

---

## 📜 Licença

Este projeto é distribuído sob a licença MIT. Livre para estudar, modificar e distribuir.

---

## 👨‍💻 **Autor**

Desenvolvido por **[Roberto Lima](https://github.com/robertolima-dev)** 🚀✨

---

## 💬 **Contato**

- 📧 **Email**: robertolima.izphera@gmail.com
- 💼 **LinkedIn**: [Roberto Lima](https://www.linkedin.com/in/roberto-lima-01/)
- 💼 **Website**: [Roberto Lima](https://robertolima-developer.vercel.app/)
- 💼 **Gravatar**: [Roberto Lima](https://gravatar.com/deliciouslyautomaticf57dc92af0)

---