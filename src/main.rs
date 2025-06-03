use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    let code = read_to_string("input.luau").expect("read");
    print!("{}", obfuscate(&code));
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
            0 => ((0x1F600 + (self.next() % 80)) as u32).try_into().unwrap_or('a'),
            _ => ((b'a' + (self.next() % 26) as u8) as char),
        }
    }
    fn key(&mut self) -> u8 { (self.next() % 255) as u8 }
}

#[derive(Clone)]
enum Tok<'a>{Id(&'a str),Str(&'a str,u8),Num(&'a str),Sym(char),Other(char)}

fn lex(code: &str) -> Vec<Tok> {
    let mut t=Vec::new();
    let b=code.as_bytes();
    let mut i=0;
    while i<b.len(){
        let c=b[i] as char;
        if c.is_whitespace(){i+=1;continue}
        if c=='-' && i+1<b.len() && b[i+1]==b'-' {while i<b.len()&&b[i]!=b'\n'{i+=1}continue}
        if c=='"' || c=='\''{
            let q=c; let mut j=i+1; while j<b.len(){if b[j] as char==q{j+=1;break}j+=1}
            t.push(Tok::Str(&code[i+1..j-1],0)); i=j; continue
        }
        if c.is_ascii_digit(){let mut j=i+1;while j<b.len()&&b[j].is_ascii_digit(){j+=1};t.push(Tok::Num(&code[i..j]));i=j;continue}
        if c.is_alphanumeric()||c=='_'{
            let mut j=i+1;while j<b.len()&&(b[j] as char).is_alphanumeric()||b[j]==b'_'{j+=1}
            t.push(Tok::Id(&code[i..j]));i=j;continue}
        t.push(Tok::Sym(c));i+=1
    }
    t
}

fn randomize(names:&mut HashMap<String,String>,token:&mut Tok,rng:&mut Rng){
    if let Tok::Id(s)=token{if !names.contains_key(*s){let n=rng.name(4);names.insert((*s).to_string(),n);}if let Some(n)=names.get(*s){*token=Tok::Id(Box::leak(n.clone().into_boxed_str()));}}
}

fn enc_str(s:&str,k:u8)->String{let mut r=Vec::with_capacity(s.len());for b in s.bytes(){r.push(b^k);}format!("dx('{}',{k})",r.into_iter().map(|b|{(b%10+48)as char}).collect::<String>())}

fn obfuscate(code:&str)->String{
    let mut rng=Rng::new(0x1234);
    let mut toks=lex(code);
    let mut names=HashMap::new();
    for i in 0..toks.len(){match toks[i].clone(){Tok::Id("local")|Tok::Id("function")=>{if let Some(Tok::Id(_))=toks.get_mut(i+1){randomize(&mut names,toks.get_mut(i+1).unwrap(),&mut rng)}},Tok::Str(s,_)=>{let k=rng.key();toks[i]=Tok::Str(Box::leak(enc_str(s,k).into_boxed_str()),k)},Tok::Num(n)=>{toks[i]=Tok::Str(Box::leak(format!("({n})").into_boxed_str()),0)},_=>{}}}
    let mut out=String::from("local function dx(s,k)local r={}for i=1,#s do r[i]=string.char(bit32.bxor(string.byte(s,i),k))end return table.concat(r)end\n");
    let mut idx=0;
    while idx<toks.len(){let mut add=String::new();match &mut toks[idx]{Tok::Id(s)=>add.push_str(s),Tok::Str(s,_u)=>add.push_str(&format!("'{s}'")),Tok::Num(n)=>add.push_str(n),Tok::Sym(c)|Tok::Other(c)=>add.push(*c),}
        out.push_str(&add);
        if idx%5==0{out.push_str(" if false then \n local _=");out.push_str(&rng.name(3));out.push_str(" end end ");}
        idx+=1;
    }
    out
}

