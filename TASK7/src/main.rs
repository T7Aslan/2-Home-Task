/*функция tuple_sum принимает кортеж из двух целых чисел. Возвращает целое число, равное сумме чисел во входном кортеже*/

fn main() {
let kortesh:(i32, i32) = (81, 12);
let (a, b) = kortesh;
let c = tuple_sum (a,b);
println!("Принимает кортеж из двух целых чисел {}, {}. Возвращает целое число, равное сумме чисел во входном кортеже,{} ", a, b, c);
}  

fn tuple_sum (a: i32, b:i32) -> i32 {
a+b}
