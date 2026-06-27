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

## Guardando várias versões (sem perder código)

Quando um exercício tem etapas/tentativas que você quer preservar, **não use
arquivos `bkp-*.rs` soltos em `src/`** — o Cargo os ignora, eles nunca compilam
e podem quebrar sem aviso.

Use a pasta `examples/`: cada arquivo com `fn main()` vira um programa rodável
que **continua sendo compilado** (`cargo build --examples`), então nunca apodrece.
Convenção: prefixo numérico por etapa.

```
cap02-jogo-adivinhacao/
├── src/main.rs              # versão atual/final
└── examples/
    ├── 01-ler-palpite.rs    # etapa: só lê o palpite
    └── 02-numero-secreto.rs # etapa: gera o número secreto
```

Rodar uma versão específica:

```bash
cargo run --example 01-ler-palpite
```

> O git é a rede de segurança principal: nada que já foi commitado se perde.
> O `examples/` complementa, deixando as versões visíveis e sempre compilando.

## Progresso

### Rust Book (pt-br)

| Capítulo | Exercício | Status |
|----------|-----------|--------|
| 2 — Jogo de adivinhação | `rust-book/cap02-jogo-adivinhacao` | 🚧 em andamento |

### Codewars

| Kata | Pasta | Status |
|------|-------|--------|
| Reverse String ([5168bb5d](https://www.codewars.com/kata/5168bb5dfe9a00b126000018)) | `codewars/reverse-string` | ✅ resolvido |
