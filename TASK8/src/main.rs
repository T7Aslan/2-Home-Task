/*функция array_sum принимает массив из трёх целых чисел. Возвращает целое число, равное сумме чисел во входном массиве*/

fn main() {
    let mass = [8, 1, 100];
    let [a, b, c] = mass;
    let d = array_sum (a,b,c);
    println!("Принимает кортеж из двух целых чисел {}, {}, {} . Возвращает целое число, равное сумме чисел во входном кортеже,{} ", a, b, c, d);
    }  
    
    fn array_sum (a: i32, b:i32, c:i32) -> i32 {
    a+b+c}
    