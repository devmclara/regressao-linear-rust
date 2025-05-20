
# 📈 Regressão Linear em Rust

Este projeto implementa um sistema de regressão linear simples utilizando a linguagem Rust. Ele foi desenvolvido para demonstrar como calcular a equação da reta ajustada, realizar previsões e calcular métricas como R² e MSE.

## 🔧 Funcionalidades

- Implementação da função de regressão linear
- Cálculo das métricas R² (coeficiente de determinação) e MSE (erro quadrático médio)
- Função para realizar previsões com base no modelo treinado
- Testes unitários cobrindo os principais métodos

## 📁 Estrutura do Projeto

```
recomendacao-produto-grafos/
├── src/
│   ├── main.rs         # Arquivo principal de execução
└── README.md           # Documentação do projeto
```

## 🚀 Como Executar o Projeto

### Pré-requisitos

- Instale o Rust: https://www.rust-lang.org/tools/install

### Passos

Clone o repositório:
```bash
git clone https://github.com/devmclara/recomendacao-produto-grafos.git
cd recomendacao-produto-grafos
```

Compile o projeto:
```bash
cargo build
```

Execute o projeto:
```bash
cargo run
```

## 🧪 Como Executar os Testes

```bash
cargo test
```

## 🧠 Arquitetura do Sistema

- `lib.rs`: contém a estrutura `RegressaoLinear` e suas implementações (`fit`, `prever`, `r2`, `mse`).
- `main.rs`: exemplo de uso do modelo.
- `tests.rs`: testes unitários para regressão, R², MSE e previsões.

## 📊 Algoritmos e Estruturas de Dados Utilizados

- Cálculo da reta usando a fórmula de mínimos quadrados.
- Utilização de vetores e iterações com `zip` para operações paralelas.
- Sem bibliotecas externas (crates), apenas Rust puro.

## 📈 Desempenho e Escalabilidade

- O sistema é leve e ideal para datasets pequenos e médios.
- A estrutura é extensível para aplicações maiores.
- Testes garantem a precisão matemática da implementação.

## 🤝 Contribuições

Contribuições são bem-vindas! Sinta-se livre para abrir issues ou pull requests com melhorias ou correções.




