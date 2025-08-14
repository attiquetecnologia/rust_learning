fn main(){
    println!("---Algoritmos em Rust!---\n");
    // Area do retângulo
    let base = 10;
    let altura = 20;
    println!("À área do retângulo é {}", base*altura);

    
    let idade = 18;
    println!("\n---Condições---\nIdade da pessoa {idade}");
    if idade < 18{
        println!("Pessoa menor de idade!");
    } else {
        println!("Pessoa maior de idade!");
    }

    println!("\n---Se valor existe---");
    let number = 3;
    if number != 0{
        println!("Sim valor existe?");
    }

    println!("\n---Divisao por zero---");
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    
    println!("\n---Match case---");
    match number {
        1 => println!("Número é 1"),
        _=>println!("Esse é o default!")
    }

    println!("\n---Laços---");
    println!("loop: contar até 10 ---");
    let mut contador = 0;
    loop {
        println!("{contador}");
        contador = contador + 1;
        if contador == 10{ break ; }
    }

    println!("\nwhile: contar até 10 ---");
    contador = 0; //restart
    while contador < 10{
        println!("{contador}");
        contador += 1;
    }

    println!("\nfor: contar até 10 iterator ---");
    for contador in 0..10{ //range python
        println!("{contador}");
    }

    println!("\nfor: contar até 10 iterator reverse ---");
    for contador in (0..10).rev() { //range python
        println!("{contador}");
    }

    println!("\nfor: ordenação por bolha ---");
    let mut elemento = [40, 1, 90 ,18, 66, 5, 9, 10, 98, 11];
    let n = elemento.len();
    println!("Número de elementos {}", n);
    for x in 0..n {
        for y in 0..n - 1 - x {
            println!("x:{x}, y:{y}");
            if elemento[y] > elemento[y+1] {
                elemento.swap(y, y+1);
            }
        }
    }


}//fim programa