
/*функция double_float64 принимает 32-х битное число с плавающей точкой и возвращает 64-х битное число с плавающей точкой, равное удвоенному входному.*/

fn main() {
    let a :f32 = 7.2;
    let b :f64 = double_float32(a);
        println!("Принимает 32-х битное число {} с плавающей точкой и возвращает 64-х битное число с плавающей точкой {} равное удвоенному входному", a,b);
    }
    fn double_float32 (a: f32) -> f64 {
        (a*2.0 as f32). into()}