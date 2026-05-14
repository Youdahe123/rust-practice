fn letter_grade(score:u32) -> &'static str {

    let grade = if score >= 90 && score <= 100 {
        1
    }else if score >= 80 && score <= 89{
        2
    }else if score >= 70 && score <= 79{
        3
    }else if score >= 60 && score <= 69 {
        4
    }else{
        0
    };


    if grade == 1{
        "A"
    }else if grade == 2{
        "B"
    }else if grade == 3{
        "C"
    }else if grade == 4{
        "D"
    }else{
        "u failed broski"
    }


}


fn main(){
    let score = 27;
    println!("Score {} : {}",score,letter_grade(score))

}


