/*
한 목록
어떤 명령을 수행할 것인지를 입력받는 인터페이스 작성
TODO
구조체 작성
구조체를 기반으로 작동하는 Linked list 작성
*/

fn main() {
    println!("Start the linked list test code");
    loop{
        let mut input = String::new();
        print_option();
        std::io::stdin().read_line(&mut input).expect("Failed to read line.");
        println!("Your input is: {}", input);
        check_option(input);
    }
}

fn print_option(){
    println!("Print what you want");
    println!("I: insert, D: delete, P: print");
}

fn check_option(option: String){
    /*
    삽질 리스트
    match 활용하려 했는데, match와 if-else 문의 비교방식 차이로 인해서 활용을 못함. 추후 도전 예정
    ㄴ 출처: https://stackoverflow.com/questions/49886160/why-can-i-compare-a-string-to-a-str-using-if-but-not-when-using-match
    if-else 문에서 비교문을 통과하는 현상 발생. 뭔가 했더니 read_line읽을 때 개행문자도 읽어서 그런거였음
    ㄴ trim()을 넣거나 비교문에 개행문자를 넣으면 해결됨.
    ㄴ 출처: http://danielnill.com/rust_tip_compairing_strings
    */

    println!("option: {}", option);
    if option.trim() == "I" {
        println!("Begin data input");
    }
    else if option.trim() == "D" {
        println!("Begin data delete");
    }
    else if option.trim() == "P" {
        println!("Begin data print");
    }
    else{
        println!("Input Error. Check what you input.")
    }
}