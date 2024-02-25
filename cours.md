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
trait Calculs {
    fn surface(&self) -> f32;

}
// ici on utilise un trait pour définir une interface et donc une méthode surface


struct Rectangle {
    long: f32,
    larg: f32,
}

impl Calculs for Rectangle{
    fn surface(&self) -> f32 {
        &self.long * &self.larg
    }
}



struct Cercle {
    rayon: f32,
}

impl Calculs for Cercle{
    fn surface(&self) -> f32 {
        &self.rayon.powf(2.0) * 3.14f32
    }
}

// ici nous ajoutons le trait Calculs à la structure Rectangle et Cercle


fn main() {
    let r = Rectangle{long:2.0, larg:1.4 };
    let c = Cercle{ rayon:3.5};
    calculer(r);
    calculer(c);

}


fn calculer(figure: impl Calculs) {
    println!("{}",figure.surface());
}


```
------
```Rust
trait Surface {
    fn surface(&self) -> f32;
    
}

trait Perimetre {
    fn perimetre(&self) -> f32;
}

struct Rectangle {
    long: f32,
    larg: f32,
}

impl Surface for Rectangle{
    fn surface(&self) -> f32 {
        &self.long * &self.larg
    }
    
}

impl Perimetre for Rectangle{
    fn perimetre(&self) -> f32 {
        (&self.long + &self.larg) *2.0f32
    }
}


fn main() {
    let r = Rectangle{long:2.0, larg:1.4 };
    let r2 = Rectangle{long:5.0, larg:2.4 };
    calculer(r, r2);

}


fn calculer(figure: impl Surface, figure2: impl Perimetre) {
    println!("{}",figure.surface());
    println!("{}",figure2.perimetre());
}


```
------
```Rust


fn main() {
   let a = 8;
   let b = 32;

   println!("{}", plus_grand(&a, &b));
}


fn plus_grand<'a>(v1: &'a u8, v2: &'a u8) -> & 'a u8 {
    if v1 > v2 {
        v1
    } else {
        v2
    }
}

// ici 'a représente la durée de vie de la référence retournée par la fonction plus_grand.

```
------
```Rust


fn main() {
  calcul(multi, 15, 23)
}

fn addition(x: i32, y:i32) -> i32 {
    x+y
}

fn multi(x: i32, y:i32) -> i32 {
    x*y
}


fn calcul<F> (function: F, a:i32, b:i32)
    where F: Fn(i32, i32) -> i32
{
    println!("{}", function(a,b));
}

// ici nous avons une fonction qui utilise une autre fonction comme paramètre
```
------
```Rust


fn main() {
    let un = 56;
    let deux = 89;
  calcul(|x,y| x+y, un , deux);
}

// ici nous utilison une closure ( | | )pour passer une fonction en parametre 



fn calcul<F> (function: F, a:i32, b:i32)
    where F: Fn(i32, i32) -> i32
{
    println!("{}", function(a,b));
}


```
------
```Rust


fn main() {
    let un = 56;
    let deux = 89;
  calcul(|| un ,deux);
}

// ici nous utilison une closure ( | | )pour passer une fonction en parametre 



fn calcul<F> (function: F, a:i32, b:i32)
    where F: Fn
{
    println!("{}", function());
}


```
------
```Rust


fn main() {
  let a = String::from("Hello");
  let clos = || println!("{}", a);
  clos();
}

// une closure a conscience de son environnement
// une closure emprunte la propriété de a et lui la rend en fin de traitement

```
------
```Rust
use std::thread;

use std::time::Duration;

fn main() {
    let th = thread::spawn(|| {
      for i in 1..10 {
        println!("---------------Thread secondaire - {}", i);
        thread::sleep(Duration::from_millis(1000));
      }
    });


   for i in 1..5 {
      println!("Thread principal - {}", i);
      thread::sleep(Duration::from_millis(600));
   }

   th.join();
}
```
------
```Rust
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let(sender,receiver) = mpsc::channel();
    thread::spawn(move || {
      for i in 1..10 {
          sender.send(i).unwrap();
          thread::sleep(Duration::from_millis(500));
      }
    });

    for recu in receiver {
        println!("Got: {}", recu);
    }

    println!("Fin de programme");
}

```
------
```Rust
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let(sender,receiver) = mpsc::channel();
    let sen2 = mpsc::Sender::clone(&sender);


    thread::spawn(move || {
      for i in 1..10 {
          sender.send(i).unwrap();
          thread::sleep(Duration::from_millis(500));
      }
    });

    thread::spawn(move || {
      for i in 100..110 {
          sen2.send(i).unwrap();
          thread::sleep(Duration::from_millis(200));
      }
    });

    for recu in receiver {
        println!("Got: {}", recu);
    }

    println!("Fin de programme");
}

 
```
------
```Rust
use std::thread;
use std::sync::{ Mutex, Arc };

fn main() {
  let mut compteur = Arc::new(Mutex::new(0));
  let mut handles = vec![];

  for _ in 0..1000 {

      let c = Arc::clone(&compteur);
      let handler = thread::spawn(move || {
          let mut num = c.lock().unwrap();
          *num += 1;
    });

    handles.push(handler);
  }

  for handle in handles {
      handle.join();
  }

  println!("{}", *compteur.lock().unwrap());
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
------
```Rust

```

