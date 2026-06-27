# 🦀 rust-study

Meus estudos de Rust: leitura do [Rust Book (pt-br)](https://rust-br.github.io/rust-book-pt-br/)
e prática de exercícios do [Codewars](https://www.codewars.com/).

Organizado como um **Cargo workspace** único: um só `target/` compartilhado,
um só repositório git, e qualquer exercício novo entra automaticamente.

## Estrutura

```
rust-study/
├── Cargo.toml          # workspace (members por glob)
├── rust-book/          # exercícios acompanhando o livro
│   └── cap02-jogo-adivinhacao/
└── codewars/           # katas resolvidos
    └── reverse-string/
```

## Como usar

Criar um exercício novo (entra no workspace sozinho, graças ao glob):

```bash
cargo new --vcs none codewars/nome-do-kata
cargo new --vcs none rust-book/cap03-variaveis
```

Rodar / testar um exercício específico (`-p` usa o *nome do pacote*):

```bash
cargo run  -p jogo_de_advinhacao
cargo test -p reverse-string
```

Compilar ou testar **tudo** de uma vez:

```bash
cargo build
cargo test
```

## Progresso

### Rust Book (pt-br)

| Capítulo | Exercício | Status |
|----------|-----------|--------|
| 2 — Jogo de adivinhação | `rust-book/cap02-jogo-adivinhacao` | 🚧 em andamento |

### Codewars

| Kata | Pasta | Status |
|------|-------|--------|
| Reverse String ([5168bb5d](https://www.codewars.com/kata/5168bb5dfe9a00b126000018)) | `codewars/reverse-string` | ✅ resolvido |
