fn main() {
    /* Plan
      "II" or VI or  IV
    1. current val = 0  & prev = 0 & total = 0
    if current> prev => total(1000) = total(0) + current(1000)  =>> prev =current(1000)
    if current == 1,10,100 => total(900) = total(1000) - current(100) ==> prev = current(100)

    2. Normal case : just add current val in total (prev >= current)
    3 Edge case : prev<current => total = current - prev
I =1 , X=10 , C=100
                     L V III => 50 ,5 ,1+1+1 = 58
    */
    let s = "LVIII"; // 1000 + (1000-100) + (100-10) + 5-1 = 1994
    let data = s.chars().rev();  // 1000 - 100
    let mut current = 0;
    let mut prev = 0;
    let mut total = 0;
    for i in data {
      current = match i {
             'I' => 1,
             'V' => 5,
             'X' => 10,
             'L' => 50,
             'C' => 100,
             'D' => 500,
             'M' => 1000,
              _ =>  99
         };
        println!("Curent : {current}, prev : {prev}");
        if current >= prev{
           total += current;
        }

        else{
            total -= current
        }

        prev = current;

        println!("{current}, {prev} = {total}");
    }
    println!("{total}");
}


