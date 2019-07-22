//Credit for the idea of this exercise found at: https://exercism.io/tracks/rust/exercises/say/solutions/ce2193baf195421eb337d4c9013e4e82
//NOTE!!!: I worked on this for two days and could not figure this out so I went to this link for inspiration on how to find an asnwer, thus I made this function with the skills I had vs what was shown to me.
//NOTE!!!: There are two tests that are fail no matter what as they are beyond the expected range for this program but still expect an answer.
pub fn encode(n: u64) -> String {
  let mut return_string = "".to_string(); 
    /*if n == 0
    {
      return_string =  "zero".to_string();
    }*/
    if n == 1
    {
      return_string =  "one".to_string();
    }
    else if n == 2
    {
      return_string ="two".to_string(); 
    }
    else if n == 3 
    {
      return_string = "three".to_string(); 
    }
    else if n == 4 
    {
      return_string = "four".to_string(); 
    }
    else if n == 5

    {
      return_string = "five".to_string(); 
    }
    else if n == 6

    {
      return_string = "six".to_string(); 
    }
    else if n == 7

    {
      return_string = "seven".to_string(); 
    }
    else if n == 8
    {
      return_string = "eight".to_string(); 
    }
    else if n == 9

    {
      return_string = "nine".to_string(); 
    }
    else if n == 10

    {
      return_string = "ten".to_string(); 
    }
    else if n == 11

    {
      return_string = "eleven".to_string(); 
    }
    else if n == 12

    {
      return_string = "twelve".to_string(); 
    }
    else if n == 13

    {
      return_string = "thirteen".to_string(); 
    }
    else if n == 14

    {
      return_string = "fourteen".to_string(); 
    }
    else if n == 15

    {
      return_string = "fifteen".to_string(); 
    }
    else if n == 16

    {
      return_string = "sixteen".to_string(); 
    }
    else if n == 17

    {
      return_string = "seventeen".to_string(); 
    }
    else if n == 18

    {
      return_string = "eightteen".to_string(); 
    }
    else if n == 19

    {
      return_string = "nineteen".to_string(); 
    }
    else if n == 20

    {
      return_string = "twenty".to_string(); 
    }
    else if n == 30

    {
      return_string = "thirty".to_string(); 
    }
    else if n == 40

    {
      return_string = "forty".to_string(); 
    }
    else if n == 50

    {
      return_string = "fifty".to_string(); 
    }
    else if n == 60

    {
      return_string = "sixty".to_string(); 
    }
    else if n == 70

    {
      return_string = "seventy".to_string(); 
    }
    else if n == 80

    {
      return_string = "eighty".to_string(); 
    }
    else if n == 90

    {
      return_string = "ninety".to_string(); 
    }
    else if n > 20 && n < 100
    {
      return_string = encode(10*(n/10)) + &"-".to_string()+ &encode(n%10);
    }
    else if n >= 100 && n < 1000
    {
      return_string = encode(n/100) + &" hundred ".to_string() + &encode(n%100);
    }
    else if n >= 1000 && n < 1000000
    {
       return_string = encode(n/1000) +&" thousand ".to_string() + &encode(n%1000);
    }
    else if n >= 1000000 && n < 999999999
    {
       return_string = encode(n/1000000) +&" million ".to_string() + &encode(n%1000000);
    }
    else if n >= 1000000000 && n < 999999999999
    {
       return_string = encode(n/1000000000) +&" billion ".to_string() + &encode(n%1000000000);
    }
    else if n >= 1000000000000 && n < 999999999999999
    {
       return_string = encode(n/1000000000000) +&" trillion ".to_string() + &encode(n%1000000000000);
    }
    else if n > 999999999999
    {
      return_string = "out of range, input is greater than 999,999,999,999.".to_string();
    }    
    if return_string.ends_with(' ')
    {
      let trim = &return_string[..return_string.len()-1];
      return_string = trim.to_string();
    }
  return_string.to_string()
}
