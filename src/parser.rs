/* Leonardo Matthew Knight
entrada -> o que deve ser seguido

cada uma das entradas vai ser um estado, pra poder verificar exatamente se o proximo character esta correto

os nomes sao iguais aos do enum dos estados

Opener -> operadorBin, operadorUn
operadorBin -> firstPreposition
operadorUn -> secondPreposition (ele conta a primeira preposicao que encontrar como segunda, pq e a unica q tenq aparecer dps de um operador unario)
firstPreposition -> secondPreposition
secondPreposition -> closer

pra comecar uma formula, deve sempre ter (

// os comandos padrao so pra n me perde

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

use std::ops::RangeInclusive;
use std::str::Chars;

pub struct Parser {
    latex_command_start: char,
    binary_commands: Vec<char>,
    latex_binary_commands: Vec<String>,
    unary_commands: Vec<char>,
    latex_unary_commands: Vec<String>,
    propositions: (RangeInclusive<u32>, RangeInclusive<u32>),
    inner_formula_opener: char,
    inner_formula_closer: char,
    constants: (char, char),
}

enum FormulaState {
    Opener, // esse e o estado inicial
    FirstProposition,
    SecondProposition,
    BinOperator,
    UnOperator,
    CloseFormula,
    Null, // esse e o estado quando da erro
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            latex_command_start: '\\',
            binary_commands: vec!['∧', '∨', '↔', '→'],
            latex_binary_commands: vec![
                String::from("land"),
                String::from("lor"),
                String::from("leftrightarrow"),
                String::from("rightarrow"),
            ],
            unary_commands: vec!['¬'],
            latex_unary_commands: vec![String::from("not")],
            propositions: (97..=122, 48..=57),
            inner_formula_opener: '(',
            inner_formula_closer: ')',
            constants: ('T', 'F'),
        }
    }
    pub fn run(&self, formula: &String) -> bool {
        let mut formula_chars = formula.chars();
        let next = formula_chars.next();
        if next.unwrap() == self.inner_formula_opener {
            return self.new_formula(&mut formula_chars) && formula_chars.next().is_none();
        }
        if next.unwrap() == self.constants.0 || next.unwrap() == self.constants.1 {
            return formula_chars.next().is_none();
        }
        false
    }

    // abre -> operadorbin -> preposicao -> preposicao -> fecha
    // abre -> operadorun -> preposicao -> fecha

    fn new_formula(&self, formula: &mut Chars) -> bool {
        let mut formula_state = FormulaState::Opener;
        let mut c = formula.next();
        while c.is_some() && !matches!(formula_state, FormulaState::Null) {
            if c.unwrap().is_whitespace() {
                c = formula.next();
                continue;
            }
            if c.unwrap() == self.constants.0 || c.unwrap() == self.constants.1 {
                return formula.next().unwrap() == self.inner_formula_closer;
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
            c = formula.next();
            formula_state = input_type;
        }
        false
    }

    fn input_type(&self, input: char, state: &FormulaState, formula: &mut Chars) -> FormulaState {
        if &input == &self.latex_command_start {
            let mut c = formula.next();
            let mut command: String = String::new();
            while c.is_some() && !c.unwrap().is_whitespace() {
                command.push(c.unwrap());
                c = formula.next();
            }
            return self.latex_command_check(command);
        }
        if self.binary_commands.contains(&input) || self.unary_commands.contains(&input) {
            return self.command_check(input);
        }
        if input == self.constants.0 || input == self.constants.1 {
            return if matches!(state, FormulaState::FirstProposition) {
                FormulaState::SecondProposition
            } else {
                FormulaState::FirstProposition
            };
        }
        if self.propositions.0.contains(&(input as u32)) {
            let mut proposition = String::from(input);
            let mut peekable = formula.clone().peekable(); // TUDO ISSO SO PQ O advance_back_by() NAO FOI IMPLEMENTADO AINDA E O PRIMEIRO PEEK() AINDA DA NEXT()
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
            if !self.new_formula(formula) {
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

    fn latex_command_check(&self, com: String) -> FormulaState {
        if self.latex_binary_commands.contains(&com) {
            return FormulaState::BinOperator;
        } else if self.latex_unary_commands.contains(&com) {
            return FormulaState::UnOperator;
        }
        return FormulaState::Null;
    }

    fn command_check(&self, com: char) -> FormulaState {
        if self.binary_commands.contains(&com) {
            return FormulaState::BinOperator;
        } else if self.unary_commands.contains(&com) {
            return FormulaState::UnOperator;
        }
        return FormulaState::Null;
    }
}
