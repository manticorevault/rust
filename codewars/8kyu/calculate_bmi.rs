fn bmi(weight: u32, height: f32) -> &'static str {
    let mut bmi_calc = weight as f32 / (height * height);
    
    if bmi_calc <= 18.5 {
        return "Underweight"
    } else if bmi_calc <= 25.0 {
        return "Normal"
    } else if bmi_calc <= 30.0 {
        return "Overweight"
    } else {
        return "Obese"
    }
    
}


// Good Practice

//fn bmi(weight: u32, height: f32) -> &'static str {
//    let index = weight as f32 / height.powi(2);
//    match index {
//        index if index <= 18.5 => "Underweight",
//        index if index <= 25.0 => "Normal",
//        index if index <= 30.0 => "Overweight",
//        _ => "Obese"
//    }