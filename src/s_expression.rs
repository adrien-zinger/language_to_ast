use super::ast::*;
use std::fs::File;
type Line = std::io::Lines<std::io::BufReader<File>>;

impl Ast {
    pub fn from_s_expression(&mut self, lines: Line) {
        let mut parentheses = 0;
        let mut code = String::new();
        let mut decrement_flag = false;
        for line in lines.flatten() {
            for c in line.trim().chars() {
                if c == '(' {
                    decrement_flag = false;
                    if !code.trim().is_empty() {
                        self.insert(code.trim());
                        code.drain(..);
                    }
                    self.increment();
                    parentheses += 1;
                } else if c == ')' {
                    if !code.trim().is_empty() {
                        if decrement_flag {
                            self.insert_suffix(code.trim());
                            code.drain(..);
                        } else {
                            self.insert(code.trim());
                            code.drain(..);
                        }
                    }
                    decrement_flag = true;
                    self.decrement();
                    parentheses -= 1;
                } else {
                    code.push(c);
                }
            }
        }
        if parentheses != 0 {
            panic!("Error missing parentheses");
        }
    }
    // Todo optimise the next function in Ast that is called to much time here
    // Or find an other way to do this.
    pub fn to_s_expression(&self) -> String {
        let mut ret = String::new();
        let mut indexes = vec![[0, 0, self.nexts(0).len()]];
        while !indexes.is_empty() {
            let index = indexes.last().unwrap();
            if index[0] != 0 && index[1] == 0 {
                ret.push('(');
                ret.push_str(&self.nodes[&index[0]]);
            }
            if self.nexts(index[0]).is_empty() || index[1] == index[2] {
                if index[0] != 0 {
                    if let Some(suffixes) = self.suffixes.get(&index[0]) {
                        ret.push_str(&suffixes.join(" "));
                    }
                    ret.push(')');
                }
                indexes.pop();
            } else {
                let next = self.nexts(index[0])[index[1]];
                let len = indexes.len() - 1;
                indexes[len][1] += 1;
                indexes.push([next, 0, self.nexts(next).len()]);
            }
        }
        ret
    }
}
