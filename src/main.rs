/*  Leonardo Matthew Knight

Para obter os pontos relativos a este trabalho, você deverá fazer um programa, usando a
linguagem de programação que desejar, que seja capaz de validar expressões de lógica propisicional
escritas em latex e definir se são expressões gramaticalmente corretas. Você validará apenas a forma
da expressão (sintaxe).

A entrada será fornecida por um arquivo de textos que será carregado em linha de comando,
com a seguinte formatação:

1. Na primeira linha deste arquivo existe um número inteiro que informa quantas expressões
lógicas estão no arquivo.

2. Cada uma das linhas seguintes contém uma expressão lógica que deve ser validada.
A saída do seu programa será no terminal padrão do sistema e constituirá de uma linha de saída
para cada expressão lógica de entrada contendo ou a palavra valida ou a palavra inválida e nada mais.

Gramática:
Formula=Constante|Proposicao|FormulaUnaria|FormulaBinaria.
Constante="T"|"F".
Proposicao=[a−z0−9]+
FormulaUnaria=AbreParen OperadorUnario Formula FechaParen
FormulaBinaria=AbreParen OperatorBinario Formula Formula FechaParen
AbreParen="("
FechaParen=")"
OperatorUnario="¬"
OperatorBinario="∨"|"∧"|"→"|"↔"


Cada expressão lógica avaliada pode ter qualquer combinação das operações de negação,
conjunção, disjunção, implicação e bi-implicação sem limites na combiação de preposições e operações.
Os valores lógicos True e False estão representados na gramática e, como tal, podem ser usados em
qualquer expressão de entrada.

Para validar seu trabalho, você deve incluir no repl.it, no mínimo três arquivos contendo
números diferentes de expressões proposicionais. O professor irá incluir um arquivo de testes extra
para validar seu trabalho. Para isso, caberá ao professor incluir o arquivo no seu repl.it e rodar o seu
programa carregando o arquivo de testes.
 */

mod parser;

use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let argument = args.get(1);
    let file_path: &str;
    if argument.is_none() {
        // caso executar sem argumento, executa o arquivo de teste padrao
        file_path = "src/input.txt";
    } else {
        file_path = argument.unwrap().as_str();
    }

    let text_contents = fs::read_to_string(file_path).expect("Arquivo nao encontrado");
    let mut lines = text_contents.lines();
    let mut n: isize = lines.next().unwrap().parse().unwrap();
    let parser = &parser::Parser::new();
    while n > 0 {
        let line = lines.next().unwrap();
        //print!("{} : ", line);
        if parser.run(&String::from(line)) {
            println!("Valido!");
        } else {
            println!("Invalido!");
        }
        n -= 1;
    }
}
