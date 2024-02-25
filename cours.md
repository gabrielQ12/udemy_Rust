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
------
```Rust

```
------
```Rust

```
