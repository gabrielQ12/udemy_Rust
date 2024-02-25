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
