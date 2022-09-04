use std::io;

fn calculadora(v: Vec<&str>, tam: usize){
    let mut calculo: f32 = 0.0;
    let mut n: usize = 0;
    let mut cont = 0;
    loop {
        if n == 0 {
            let operador = v[n + 1];
            let n1: f32 = v[n].trim().parse().expect("DEU ERRO NO N1");
            let n2: f32 = v[n + 2].trim().parse().expect("DEU ERRO NO N2");
            let operacao: f32 = match operador {
                "+" => n1 + n2,
                "-" => n1 - n2,
                "x"|"*"|"X" => n1 * n2,
                "/" => n1 / n2,
                &_ => todo!(),
            };
            calculo = operacao;
            n += 1;
            cont += 3;
            if cont == tam{
                break;
            }else{
                continue;
            };
        }else {
            let operador = v[n + 2];
            let n2: f32 = v[n + 3].trim().parse().expect("DEU ERRO NO N2");
            let operacao: f32 = match operador {
                "+" => calculo + n2,
                "-" => calculo - n2,
                "x"|"*"|"X" => calculo * n2,
                "/" => calculo / n2,
                &_ => todo!(),
            };
            calculo = operacao;
            n += 2;
            cont += 2;
            if cont == tam{
                break;
            }else{
                continue;
            };
        };
    };
    println!("{:?}", calculo);
}



fn main() {
    let mut numero1 = String::new();
    io::stdin().read_line(&mut numero1).unwrap();
    let v: Vec<&str> = numero1.trim().split_whitespace().collect();
    let tam = v.len();
    calculadora(v, tam);
//    println!("tamanho do Vec {}", tam);
//   println!("n° 3 {}", v[2]);
//    for s in numero1{
//        println!('{:?}', s)
//      }
//    println!('Esse e seu numero: {}', numero1);

}
