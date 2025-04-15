/*функция tuple_sum принимает кортеж из двух целых чисел. Возвращает целое число, равное сумме чисел во входном кортеже*/

fn main() {
    let kortesh: (i32, i32) = (81, 12);
    let kortesh_summ: i32 = tuple_sum(kortesh);
    println!("Принимает кортеж из двух целых чисел {}, {}. Возвращает целое число, равное сумме чисел во входном кортеже,{kortesh_summ} ",kortesh.0, kortesh.1);
}

fn tuple_sum(kortesh: (i32, i32)) -> i32 {
    (kortesh.0 + kortesh.1) as i32
}

