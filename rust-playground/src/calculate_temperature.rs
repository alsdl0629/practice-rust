pub fn convert_temperature_unit(temperature: String) -> f64 {
    let temperature = temperature.trim();

    return if temperature.ends_with("C") {
        convert_celsius_to_fahrenheit(temperature)
    } else {
        convert_fahrenheit_to_celsius(temperature)
    };
}

fn convert_celsius_to_fahrenheit(temperature: &str) -> f64 {
    let temperature: f64 = temperature
        .replace("C", "")
        .parse()
        .expect("변환 실패");

    let result = (temperature * 1.8) + 32f64;

    return result;
}

fn convert_fahrenheit_to_celsius(temperature: &str) -> f64 {
    let temperature: f64 = temperature
        .replace("F", "")
        .parse()
        .expect("변환 실패");

    let result = (temperature - 32f64) * 5f64 / 9f64;

    return result
}

fn get_current_temperature() {

}
