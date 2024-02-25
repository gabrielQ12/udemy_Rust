```Rust
mod mon_module;

// a l'interieur d'un module les fonctions sont privées par défaut 
// pour les rendre publiques il faut les déclarer avec le mot clé pub


fn main() {
    mon_module::hello();
}

// ici on importe le module mon_module


// Pour importer un module dans un autre module il faut utiliser le mot clé use
```
------
```Rust
extern crate rand;
use rand::prelude::*;

// ici on ajoute la librairie rand, on l'importe dans le programme et on precise qu'on veut utiliser tout le prelude de rand
// souvent en Rust "prelude" correspond au module de base d'une librairie

fn main() {
    println!("{}", random::<i8>());
}

// ici on utilise la fonction random de rand pour generer un nombre aleatoire, il est important de preciser le type de nombre qu'on veut generer
```
------
```Rust
enum Choix {
    Pour,
    Contre,
}

// ici nous avons un enum qui contient deux variantes, Pour et Contre.

fn main() {
   choisir(Choix::Pour);
}

fn choisir(choix: Choix) {
    match choix {
        Choix::Pour => println!("Plutôt pour"),
        Choix::Contre => println!("Plutôt contre"),
    }
}


```
------
```Rust
enum Choix {
    Pour,
    Contre(String),
}

// ici contre contient une valeur qui est une chaîne de caractères, 
// cette valeur devra être fournie lors de l'appel de la fonction choisir

fn main() {
   choisir(Choix::Contre("pas d'accord".to_owned()));
}

fn choisir(choix: Choix) {
    match choix {
        Choix::Pour => println!("Plutôt pour"),
        Choix::Contre(x) => println!("Plutôt contre parce que {}", x ),
    }
}


```
------
```Rust


fn main() {
   match op(85) {
    Option::None => println!("Aucune valeur"),
    Option::Some(x) => println!("{}", x )
   }
}

fn op(arg: u8) -> Option<u8> {
    Option::Some(arg)
}

// Option est un type enumératif qui permet de gérer les cas où une valeur peut être absente.

```
------
```Rust
enum Result<T,E> {
    Ok(T),
    Err(E),
}

// ici Result est un type générique qui prend deux paramètres de type T et E
// Ok et Err sont des variantes de Result et seront utiliser pour de la gestion d'erreur



fn main() {
   panic!("crash du programme");
}

```
------
```Rust
use std::fs::File;

fn main() {
  let f = File::open("coucou.txt");
  let file = match f {
    Ok(content) => content,
    Err(error) => {
        panic!("{}", error);
    },
  };
}

```
------
```Rust
use std::fs::File;

fn main() {
  let f = File::open("coucou.txt").unwrap();
  
}

// la methode unwrap() permet de retourner le resultat si il est Ok sinon elle leve une erreur
```
------
```Rust
use std::fs::File;

fn main() {
  let f = File::open("coucou.txt").exept("Impossible de lire le fichier");
  
}

// ici la methode except sera appelée si le fichier n'existe pas et renverra un message d'erreur personnalisé
```
------
```Rust
struct Rectangle<T> {
    x: T,
    y: T,
}
// ici <T> est un type générique (ou un type indefinis)



fn main() {
    let r1 = Rectangle{x:2,y:5};
    let r2 = Rectangle{x:2.0, y:5.0};
    


}

// attention quand on utilise un type générique on peu par erreur passé des choses totalement diférente, 
// par exemple un string a la place des chiffres par exemple let r3 = Rectangle{x:"un", y:"deux"};
```
------
```Rust
struct Rectangle<T> {
    x: T,
    y: T,
}


impl<T> Rectangle<T> {
    fn valeur_x(&self) -> &T {
        &self.x
    }
}



fn main() {
    let r1 = Rectangle{x:2,y:5};
    let r2 = Rectangle{x:2.0, y:5.0};

    print!("{}", r1.valeur_x());

}


```
------
```Rust
struct Rectangle<T> {
    x: T,
    y: T,
}


impl Rectangle<usize> {
    fn surface(&self) -> usize {
        &self.x * &self.y
    }
}
// ici on ne peu pas multiplier deux T, car le compilateur ne sait pas si T est un type qui peut être multiplié
// donc on specifier usize pour la methode surface

impl Rectangle<f32> {
    fn surface(&self) -> f32 {
        &self.x * &self.y
    }
}

// dans le cadre d'un type générique nouq pouvons implémenté plusieurs méthode avec des types différents


fn main() {
    let r1 = Rectangle{x:2,y:5};
    let r2 = Rectangle{x:2.2, y:5.0};

    print!("{}", r1.surface());
    print!("{}", r2.surface());

}


```
------
```Rust

```
------
```Rust

```
------
```Rust

```
------
```Rust

```
------
```Rust

```

