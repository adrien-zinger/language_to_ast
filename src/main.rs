mod ast;
mod s_expression;
mod t_expression;
use ast::*;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
type Line = io::Lines<io::BufReader<File>>;

fn read_lines<P>(filename: P) -> io::Result<Line>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn lines_iter_in<P>(filename: P) -> Line
where
    P: AsRef<Path>,
{
    match read_lines(filename) {
        Ok(lines) => lines,
        _ => panic!("Error reading file"),
    }
}

fn main() {
    sample_t_expression();
    sample_s_expression();
}

fn sample_t_expression() {
    let mut ast = Ast::new();
    ast.from_t_expression(lines_iter_in("t_expression.wlan"));
    println!("{}", ast);
    println!("{}", ast.to_t_expression());
}

fn sample_s_expression() {
    let mut ast = Ast::new();
    ast.from_s_expression(lines_iter_in("function.wlan"));
    println!("{}", ast);
    println!("{}", ast.to_s_expression());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_t_expression_basic() {
        let mut expected = Ast::new();

        expected.insert("load-and-use SDL");
        expected.insert("defn tick [state]");
        expected.increment();
        expected.insert("+ state 10");
        expected.decrement();
        expected.insert("defn draw [app rend state]");
        expected.increment();
        expected.insert("bg rend &(rgb (/ @state 2) (/ @state 3) (/ @state 4))");
        expected.increment();
        expected.insert("SDLApp.run-with-callbacks &app SDLApp.quit-on-esc tick draw state");
        expected.decrement();
        expected.decrement();
        expected.insert("defn main []");
        expected.increment();
        expected.insert("let [app (SDLApp.create \"The Minimalistic Color Generator\" 400 300) state 0]");
        expected.increment();
        expected.insert("SDLApp.run-with-callbacks &app SDLApp.quit-on-esc tick draw state");

        let mut ast = Ast::new();
        ast.from_t_expression(lines_iter_in("./tests/basic_t.wlan"));
        for i in 1..9 {
            assert_eq!(ast.nodes[&i], expected.nodes[&i]);
        }
    }

    #[test]
    fn from_t_expression_chain() {
        let mut expected = Ast::new();

        expected.insert("a"); // 1
        expected.increment();
        expected.insert("b"); // 2
        expected.increment();
        expected.insert("c"); // 3
        expected.increment();
        expected.insert("d"); // 4
        expected.increment();
        expected.insert("e"); // 5
        expected.increment();
        expected.insert("f"); // 6
        expected.increment();
        expected.insert("g"); // 7
        expected.increment();
        expected.insert("h"); // 8
        expected.decrement();
        expected.insert("i"); // 9
        expected.decrement();
        expected.insert("j"); // 10
        expected.decrement();
        expected.insert("k"); // 11
        expected.decrement();
        expected.insert("l"); // 12
        expected.decrement();
        expected.insert("m"); // 13
        expected.decrement();
        expected.insert("n"); // 14

        let mut ast = Ast::new();
        ast.from_t_expression(lines_iter_in("./tests/chain_t.wlan"));
        for i in 1..14 {
            assert_eq!(ast.nodes[&i], expected.nodes[&i]);
        }
        assert_eq!(ast.edges.len(), expected.edges.len());
        for i in 0..ast.edges.len() {
            assert_eq!(ast.edges[i].0, expected.edges[i].0);
            assert_eq!(ast.edges[i].1, expected.edges[i].1);
        }
    }

    #[test]
    fn from_t_expression_medium() {
        let mut ast = Ast::new();
        ast.from_t_expression(lines_iter_in("./tests/medium_t.wlan"));
    }
}