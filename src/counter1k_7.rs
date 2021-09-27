pub(crate) fn calculate() {
    let mut number = 1000;
    let mut number_calculated = number - 7;

    while number_calculated > 0 {

        println!("{} - 7 = {}", number, number_calculated);

        number = number_calculated;

        number_calculated = number - 7;
    }
}