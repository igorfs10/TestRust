// A aplicação inicia com a função main
fn main(){
    //str é uma variável de texto com tamanho imutável
    const NOME: &str = "Igor";
    const IDADE: u8 = 27;
    // Função macro usa "!"
    println! ("Olá, meu nome é {name} e tenho {age} anos.", name = NOME, age = IDADE);

    let resultado = soma(5, 1);
    println! ("A soma de 5 + 1 é {resultado}", resultado = resultado);

    const CARA: Pessoa = Pessoa {nome: "João", idade: 55};
    let dados_cara: Dados = Dados {altura: 1.75, peso: 88.0 };
    println! ("{}", CARA.imprimir_dados());
    println! ("Altura = {}", dados_cara.altura);
    println! ("Peso = {}", dados_cara.peso);

    // Tupla(tuple) é um tipo que pode receber mais de uma variável diferente. Para imprimir tem que usar {:?} na string
    // Tuplas muito grandes não podem ser imprimidas de uma vez. Use ".1" para acessar o item da tupla
    const ITEM_TUPLA: (i8, f64) = (1, 5.5);
    println! ("Printar tupla {:?}", ITEM_TUPLA);
    println! ("Segundo valor da tupla {}", ITEM_TUPLA.1);


    // Match é como o switch case de outras linguagens
    let number: u8 = 5;
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
    }
}

/* Em rust você declara a função com fn + nome + parâmetros entre parenteses e
    uma seta que informa o tipo do retorno.
    Não é necessário usar o return, nesse caso ele retorna o valor da ultima linha sem ;
*/
fn soma(primeiro_numero: i64, segundo_numero: i64) -> i64 {
    // return primeiro_numero + segundo_numero;
    primeiro_numero + segundo_numero // Mais comum
}


// Struct é um tipo criado que pode armazenar variáveis com outros tipos definidos.
struct Dados {
    altura: f64,
    peso: f64
}

// 'a é necessário para criar struct constante
// ' é um alterador de tempo de vida da variável e são usados depois da anotação de ponteiro & com 1 caracter geralmente
// 'static tem o maior tempo de vida, que é a aplicação inteira que é o mesmo da constante
struct Pessoa<'a> {
    nome: &'a str,
    idade: u8
}

// imp implementa funções na struct
// String é uma variável de texto que tem tamanho mutável
// format! é um macro que retorna um texto formatado
// usei "<'_>" que é um lifetime anonimo para sercompativel com a struct
impl Pessoa<'_>{
    fn imprimir_dados(&self) -> String {
        format! ("Nome: {}, Idade: {}", self.nome, self.idade)
    }
}

/*
    let : Declare variable
    const : Declare constant
    fn : Declare function

    bool : The boolean type.
    char : A character type.
    i8 : The 8-bit signed integer type.
    i16 : The 16-bit signed integer type.
    i32 : The 32-bit signed integer type.
    i64 : The 64-bit signed integer type.
    i128 : The 128-bit signed integer type.
    isize : The pointer-sized signed integer type.
    u8 : The 8-bit unsigned integer type.
    u16 : The 16-bit unsigned integer type.
    u32 : The 32-bit unsigned integer type.
    u64 : The 64-bit unsigned integer type.
    u128 : The 128-bit unsigned integer type.
    usize : The pointer-sized unsigned integer type.
    f32 : The 32-bit floating point type.
    f64 : The 64-bit floating point type.
    array : A fixed-size array, denoted [T; N], for the element type, T, and the non-negative compile-time constant size, N.
    slice : A dynamically-sized view into a contiguous sequence, [T].
    str : String slices.
    tuple : A finite heterogeneous sequence, (T, U, ..).

    Decimal : 98_222
    Hex	: 0xff
    Octal : 0o77
    Binary : 0b1111_0000
    Byte (u8 only) : b'A'
*/