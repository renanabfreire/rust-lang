# 🦀 Desafios de Rust - Capítulo 5 (Structs)

Abaixo estão 5 desafios projetados para te ajudar a praticar os conceitos do capítulo 5 do *Rust Book*, incluindo o uso de `struct`, `impl`, métodos e tipos como `Option` e `match`.  
**Obs:** Como ainda não foi estudado `Vec`, substitua por arrays fixos sempre que necessário.

---

## 🧩 Desafio 1: Conversor de Temperaturas com Struct

Crie uma struct `Temperatura` que guarde o valor e a escala (Celsius ou Fahrenheit).  
Implemente métodos para:

- Converter de Celsius para Fahrenheit e vice-versa
- Exibir como string (ex: `"30°C"` ou `"86°F"`)
- Criar uma nova temperatura a partir de uma string como `"30C"` ou `"86F"`

**Extras:**

- Trate entradas inválidas com `Option` ou `Result`

---

## 🧩 Desafio 2: Sistema de Cadastro de Livros

Crie uma struct `Livro` com os seguintes campos:

- `titulo: String`
- `autor: String`
- `ano: u32`
- `emprestado: bool`

**Implemente:**

- Um método `.novo()` que cria um livro
- Métodos `.emprestar()` e `.devolver()`
- Uma função que recebe uma lista de livros e retorna os disponíveis para empréstimo

**Extras:**

- Exiba os livros ordenados por ano
- Crie um menu interativo no terminal

---

## 🧩 Desafio 3: Analisador de Notas

Estruture um programa que:

1. Define uma struct `Aluno` com nome e notas (`[f32; N]`, por exemplo)
2. Implemente métodos para:
   - Calcular média
   - Verificar se está aprovado (média ≥ 7)
   - Exibir boletim formatado
3. Crie uma função que receba vários alunos e retorne aquele com a melhor média

**Extras:**

- Leia os dados da entrada do usuário (nome e notas)

---

## 🧩 Desafio 4: Jogo de Adivinhação com Registro de Tentativas

Implemente uma struct `Jogo` que guarda:

- Número secreto
- Número de tentativas
- Array de palpites (`[u32; N]`)

**Adicione métodos para:**

- Processar um palpite
- Verificar se acertou
- Mostrar histórico de tentativas

**Extras:**

- Permita jogar novamente sem perder o histórico geral (usar outra struct ou array externo)

---

## 🧩 Desafio 5: Gerenciador de Tarefas (Mini Projeto)

Crie uma struct `Tarefa` com:

- `descricao: String`
- `concluida: bool`

Crie uma struct `ListaTarefas` com:

- Array de tarefas

**Métodos:**

- Adicionar nova tarefa
- Marcar tarefa como concluída
- Listar tarefas pendentes
- Remover tarefas concluídas

**Extras:**

- Use `Option` ao buscar tarefas por índice
- Crie um menu simples com `loop` e entrada do usuário

---

