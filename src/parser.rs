/*
entrada -> o que deve ser seguido

cada uma das entradas vai ser uma estado, pra poder verificar exatamente se o proximo character esta correto

operador unario -> primeiraPreposicao, (, segundaPreposicao
primeiraPreposicao -> operador binario
( -> operador unario, primeiraPreposicao      } chama a propria funcao mais uma vez ate terminar com )
operador binario -> segundaPreposicao, (, operadorUnario
segundaPreposicao -> nada, ), operador binario
) -> nada, operadorBinario


pra comecar uma formula, deve sempre ter uma preposicao, parenteses ou operador unario

// por padrao so pra n me perde
- \ : inicio de comando:
    \neg : ¬
    \lor : ∨
    \land : ∧
    \rightarrow : →
    \leftrightarrow : ↔

- ( : nova formula unaria ou binaria
- ) : fim formula unario ou binaria

- [a-z0-9]+ : proposicoes validas

- T, F : constantes

 */

use std::iter::Peekable;
use std::ops::RangeInclusive;
use std::str::Chars;

pub struct Parser {
    command_start: char,
    binary_commands: Vec<String>,
    unary_commands: Vec<String>,
    propositions: (RangeInclusive<u32>, RangeInclusive<u32>),
    inner_formula_opener: char,
    inner_formula_closer: char,
}

enum FormulaState {
    Opener, // esse e o estado inicial, onde vai aceitar as mesmas entradas que o (
    FirstProposition,
    SecondProposition,
    BinOperator,
    UnOperator,
    CloseFormula,
    Null,
}

impl Parser {
    /* CODIGO DESNECESSARIO
    pub fn new_with_grammar(
        command_start: char,
        binary_commands: Vec<String>,
        unary_commands: Vec<String>,
        inner_formula_opener: char,
        inner_formula_closer: char,
        propositions: (RangeInclusive<u32>, RangeInclusive<u32>, u32),
    ) -> Parser {
        Parser {
            command_start,
            binary_commands,
            unary_commands,
            propositions,
            inner_formula_opener,
            inner_formula_closer,
        }
    }
    */
    pub fn new() -> Parser {
        Parser {
            command_start: '\\',
            // bem ridiculo isso aqui
            binary_commands: vec![
                String::from("land"),
                String::from("lor"),
                String::from("leftrightarrow"),
                String::from("rightarrow"),
            ],
            unary_commands: vec![String::from("not")],
            propositions: (97..=122, 48..=57),
            inner_formula_opener: '(',
            inner_formula_closer: ')',
        }
    }
    pub fn run(&self, formula: &String) -> bool {
        let mut formula_chars = formula.chars();
        if formula_chars.next().unwrap() == self.inner_formula_opener {
            return self.new_formula(&mut formula_chars, false);
        }
        false
    }

    // abre -> operadorbin -> preposicao -> preposicao -> fecha
    // abre -> operadorun -> preposicao -> fecha

    fn new_formula(&self, formula: &mut Chars, inner: bool) -> bool {
        let mut formula_state = FormulaState::Opener;
        let mut c = formula.next();
        while c.is_some() && !matches!(formula_state, FormulaState::Null) {
            if c.unwrap().is_whitespace() {
                c = formula.next();
                continue;
            }
            let mut input_type = self.input_type(c.unwrap(), &formula_state, formula);
            match formula_state {
                FormulaState::Opener => {
                    if !matches!(input_type, FormulaState::BinOperator)
                        && !matches!(input_type, FormulaState::UnOperator)
                    {
                        return false;
                    }
                }
                FormulaState::FirstProposition => {
                    if !matches!(input_type, FormulaState::SecondProposition) {
                        return false;
                    }
                }
                FormulaState::SecondProposition => {
                    return if !matches!(input_type, FormulaState::CloseFormula) {
                        false
                    } else {
                        true
                    }
                }
                FormulaState::BinOperator => {
                    if !matches!(input_type, FormulaState::FirstProposition) {
                        return false;
                    }
                }
                FormulaState::UnOperator => {
                    if !matches!(input_type, FormulaState::FirstProposition) {
                        return false;
                    } else {
                        input_type = FormulaState::SecondProposition;
                    }
                }
                FormulaState::CloseFormula => return true,
                FormulaState::Null => return false,
            }
            // o dia que os cara termina de implementa o advance_back_by() pq o peekable nao faz o que eu quero eu vo
            // realmente comecar a gostar de rust
            c = formula.next();
            formula_state = input_type;
        }
        false
    }

    fn input_type(&self, input: char, state: &FormulaState, formula: &mut Chars) -> FormulaState {
        if &input == &self.command_start {
            let mut c = formula.next();
            let mut command: String = String::new();
            while c.is_some() && !c.unwrap().is_whitespace() {
                command.push(c.unwrap());
                c = formula.next();
            }
            return self.command_check(command);
        }
        if self.propositions.0.contains(&(input as u32)) {
            let mut proposition = String::from(input);
            let mut peekable = formula.clone().peekable(); // TUDO ISSO SO PQ O advance_back_by() NAO FOI IMPLEMENTADO AINDA
            let mut next_peek = peekable.next();
            while next_peek.is_some()
                && next_peek.unwrap() != self.inner_formula_closer
                && !next_peek.unwrap().is_whitespace()
            {
                proposition.push(formula.next().unwrap());
                next_peek = peekable.next();
            }
            if self.proposition_check(proposition) {
                return if matches!(state, FormulaState::FirstProposition) {
                    FormulaState::SecondProposition
                } else {
                    FormulaState::FirstProposition
                };
            }
        }
        if &self.inner_formula_opener == &input {
            if !self.new_formula(formula, true) {
                return FormulaState::Null;
            }
            return if matches!(state, FormulaState::FirstProposition) {
                FormulaState::SecondProposition
            } else {
                FormulaState::FirstProposition
            };
        }
        if &self.inner_formula_closer == &input {
            return FormulaState::CloseFormula;
        }
        FormulaState::Null
    }

    fn proposition_check(&self, propo: String) -> bool {
        let mut chars = propo.chars();
        let mut c = chars.next();
        while c.is_some() && self.propositions.0.contains(&(c.unwrap() as u32)) {
            c = chars.next(); // vai ate encontrar alguma coisa que nao esteja entre a-z
        }
        while c.is_some() && self.propositions.1.contains(&(c.unwrap() as u32)) {
            // agora verifica se tem numeros e para qnd nao tiver mais
            c = chars.next();
        }
        if c.is_none() || c.unwrap().is_whitespace() {
            // se o ultimo valor for + ou espaco vazio, e uma preposicao valida
            return true;
        }
        false
    }

    fn command_check(&self, com: String) -> FormulaState {
        if self.binary_commands.contains(&com) {
            return FormulaState::BinOperator;
        } else if self.unary_commands.contains(&com) {
            return FormulaState::UnOperator;
        }
        return FormulaState::Null;
    }
}
