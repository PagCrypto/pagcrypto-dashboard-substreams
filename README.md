# 📊 PagCrypto Dashboard Substreams

Este projeto usa [The Graph Substreams](https://thegraph.com) para indexar transações na blockchain Solana que contenham o memo `paycrypto:{hash}`, representando operações do protocolo **PagCrypto**.

## 🧠 O que ele faz?

- Escaneia todos os blocos da Solana em busca de transações com memo padrão do PagCrypto.
- Extrai:
  - `signature`
  - `slot`
  - `memo`
  - `fee`
  - `success` (indica se a transação falhou ou não)
- Expõe os dados para dashboards, analytics ou consumo por IA/alertas.

## 🚀 Como rodar

### 1. Requisitos

- `Rust` + `cargo`
- `substreams` CLI  
  Instale com:
  ```bash
  curl -s https://get.substreams.dev | bash
  ```

### 2. Clonar e preparar

```bash
git clone https://github.com/pagcrypto/pagcrypto-dashboard-substreams.git
cd pagcrypto-dashboard-substreams
```

### 3. Gerar os tipos Protobuf

```bash
substreams protogen
```

### 4. Compilar o projeto

```bash
cargo build --release
```

### 5. Gerar o pacote `.spkg`

```bash
substreams package
```

### 6. Rodar localmente

```bash
substreams run map_pagcrypto_tx \
  --start-block 0 \
  --stop-block +100 \
  --output json
```

## 🛠 Estrutura do Projeto

```
.
├── Cargo.toml
├── proto/
│   └── pagcrypto.proto
├── src/
│   └── lib.rs
├── substreams.yaml
├── README.md
```

## 📦 Deployment

1. Publique o `.spkg` gerado em [https://substreams.dev](https://substreams.dev).
2. Use um sink como:
   - `substreams-sink-postgres`
   - `substreams-sink-sqlite`
   - `substreams-sink-grpc` para backend personalizado

## 🤝 Contribuição

Pull requests são bem-vindos. Para maiores informações, fale com o time da [PagCrypto](https://pagcrypto.finance).

## 🧾 Licença

MIT
