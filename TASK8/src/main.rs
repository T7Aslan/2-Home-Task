/*функция array_sum принимает массив из трёх целых чисел. Возвращает целое число, равное сумме чисел во входном массиве*/

fn main() {
    let mass: [i32; 3] = [8, 1, 100];
    let summ_mass = array_sum(mass);
    println!(" Функция array_sum принимает массив из трех целых чисел {}, {}, {} . Возвращает целое число, равное сумме чисел во входном массиве,{} ", mass[0] , mass[1] , mass[2], summ_mass);
}

fn array_sum(mass: [i32; 3]) -> i32 {
    (mass[0] + mass[1] + mass[2]) as i32
}
