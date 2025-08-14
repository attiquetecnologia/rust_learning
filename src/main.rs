#[derive(Debug)]
struct Pessoa {
    pub id_p: u64,
    pub nome: String
}
fn texto(){
    let v = "Texto";
    v
}
fn main() {
    println!("Hello, world!");
    let p1 = Pessoa {
        id_p: 1, nome: String::from("Rodrigo") };
    print!("{:?}", p1.nome );
    print!("{:?}", texto());
}
