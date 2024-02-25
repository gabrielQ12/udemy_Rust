struct Truc_A_Dire<'a> {
    texte : &'a str,
}

// texte est un pointeur d'une durée de vie 'a

fn main() {
  let a = Truc_A_Dire { texte: "Hello, world!" };
    println!("{}", a.texte);
}

 // a chaque manipulation de pointeur il faudra specifier une durée de vie (lifetime)