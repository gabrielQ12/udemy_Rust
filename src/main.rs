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

