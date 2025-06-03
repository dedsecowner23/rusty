#![forbid(unsafe_code)]
use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if let Some(path) = std::env::args().nth(1) {
        input = std::fs::read_to_string(path).expect("read file");
    } else {
        io::stdin().read_to_string(&mut input).expect("read");
    }
    print!("{}", obfuscate(&input));
}

struct Rng(u64);
impl Rng {
    fn new(seed: u64) -> Self { Self(seed) }
    fn next(&mut self) -> u32 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
        (self.0 >> 33) as u32
    }
    fn name(&mut self, n: usize) -> String {
        let mut s = String::with_capacity(n);
        for _ in 0..n { s.push(self.char()); }
        s
    }
    fn char(&mut self) -> char {
        match self.next() % 4 {
            0 => char::from_u32(0x1F600 + (self.next() % 80) as u32).unwrap_or('a'),
            _ => (b'a' + (self.next() % 26) as u8) as char,
        }
    }
    fn key(&mut self) -> u8 { (self.next() % 255) as u8 }
}

#[derive(Clone)]
enum Tok { Id(String), Str(String,u8), Num(String), Sym(char) }

fn lex(code: &str) -> Vec<Tok> {
    let mut t=Vec::new();
    let b=code.as_bytes();
    let mut i=0;
    while i<b.len(){
        let c=b[i] as char;
        if c.is_whitespace(){i+=1;continue}
        if c=='-' && i+1<b.len() && b[i+1]==b'-' {
            i+=2;
            if i+1<b.len() && b[i]==b'[' && b[i+1]==b'[' {
                i+=2; while i+1<b.len() && !(b[i]==b']'&&b[i+1]==b']'){i+=1} i+=2;
            } else { while i<b.len() && b[i]!=b'\n'{i+=1}; }
            continue
        }
        if c=='"' || c=='\''{
            let q=c; let mut j=i+1; while j<b.len(){if b[j] as char==q{j+=1;break}j+=1}
            t.push(Tok::Str(code[i+1..j-1].to_string(),0)); i=j; continue
        }
        if c.is_ascii_digit(){let mut j=i+1;while j<b.len()&&(b[j] as char).is_ascii_digit(){j+=1};t.push(Tok::Num(code[i..j].to_string()));i=j;continue}
        if c.is_alphanumeric()||c=='_'{
            let mut j=i+1;while j<b.len()&&((b[j] as char).is_alphanumeric()||b[j]==b'_'){j+=1}
            t.push(Tok::Id(code[i..j].to_string()));i=j;continue}
        t.push(Tok::Sym(c));i+=1
    }
    t
}

fn obfuscate(code:&str)->String{
    let mut rng=Rng::new(0x1234);
    let mut toks=lex(code);
    let mut names=HashMap::new();
    let mut i=0;
    while i<toks.len(){
        match &toks[i]{
            Tok::Id(id) if id=="local" => {
                if let Some(Tok::Id(name))=toks.get(i+1){names.entry(name.clone()).or_insert_with(|| rng.name(4));}
            }
            Tok::Id(id) if id=="function" => {
                if let Some(Tok::Id(name))=toks.get(i+1){names.entry(name.clone()).or_insert_with(|| rng.name(4));}
                let mut j=i+2;
                while j<toks.len(){
                    match &toks[j]{
                        Tok::Sym('(')=>{j+=1;continue},
                        Tok::Sym(')')=>break,
                        Tok::Id(p)=>{names.entry(p.clone()).or_insert_with(|| rng.name(4));j+=1},
                        _=>j+=1,
                    }
                }
            }
            _=>{}
        }
        i+=1;
    }
    for tok in &mut toks{
        match tok{
            Tok::Id(id)=>{if let Some(n)=names.get(id){*id=n.clone();}}
            Tok::Str(s,k)=>{let key=rng.key();*k=key;*s=enc_str(s,key);}
            Tok::Num(n)=>{*n=format!("({})",n);}
            _=>{}
        }
    }
    let mut out=String::from("local function dx(s,k)local r={}for i=1,#s do r[i]=string.char(bit32.bxor(string.byte(s,i),k))end return table.concat(r)end\n");
    for (idx,t) in toks.iter().enumerate(){
        match t{
            Tok::Id(s)|Tok::Num(s)=>out.push_str(s),
            Tok::Str(s,_k)=>{out.push('\'');out.push_str(s);out.push('\'');}
            Tok::Sym(c)=>out.push(*c),
        }
        if idx%7==0{out.push_str(" if false then local _='");out.push_str(&rng.name(5));out.push_str("' end ");}
    }
    out
}

fn enc_str(s:&str,k:u8)->String{let mut r=Vec::with_capacity(s.len());for b in s.bytes(){r.push(b^k);}format!("dx('{}',{k})",r.into_iter().map(|b|{(b%10+48)as char}).collect::<String>())}
