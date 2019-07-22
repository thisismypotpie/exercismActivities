fn ones_place_to_string(n: char)->String{
    if n == '1'
    {
      return "one".to_string();
    }
    else if n == '2'
    {
      return"two".to_string(); 
    }
    else if n =='3' 
    {
      return "three".to_string(); 
    }
    else if n =='4' 
    {
      return "four".to_string(); 
    }
    else if n =='5'

    {
      return "five".to_string(); 
    }
    else if n =='6'

    {
      return "six".to_string(); 
    }
    else if n =='7'

    {
      return "seven".to_string(); 
    }
    else if n =='8'
    {
      return "eight".to_string(); 
    }
    else if n =='9'

    {
      return "nine".to_string(); 
    }
  "".to_string()
}

fn tens_place_to_string(x: char, y:char) ->String{
  if x =='1'
  {
    if y == '0'
    {
      return "ten".to_string();
    }
    else if y == '1'
    {
      return "eleven".to_string();
    }
    else if y == '2'
    {
      return "twelve".to_string();
    }
    else if y == '3'
    {
      return "thirteen".to_string();
    }
    else if y =='4'
    {
      return "fourteen".to_string();
    }
    else if y== '5'
    {
      return "fifteen".to_string();
    }
    else
    {
      return ones_place_to_string(y).to_string() + &"teen".to_string();
    }
  }
  else if x == '2'
  {
    return "twenty-".to_string();
  }
  else if x=='3'
  {
    return "thirty-".to_string();
  }
  else if x=='4'
  {
    return "fourty-".to_string();
  }
  else if x=='5'
  {
    return "fifty-".to_string();
  }
  else
  {
    return ones_place_to_string(y).to_string() + &"ty-".to_string();
  }
}

pub fn encode(n: u64) -> String {
//  let mut n_string:[String;12] = ["0".to_string(),"0".to_string(),"0".to_string(),"0".to_string(),"0".to_string(),"0".to_string(),"0".to_string(),"0".to_string(),"0".to_string(),"0".to_string(),"0".to_string(),"0".to_string()];
  let mut return_string = "".to_string();
  let mut digit = "".to_string();
  let mut place = n.to_string().len();
  let mut skip = false;
  if n > 999999999999
  {
    return "Out of Range!".to_string();
  }
  for i in n.to_string().chars()
  {
    if(skip == true)
    {
      skip = false;
      continue;
    }
    digit = ones_place_to_string(i).to_string();
    if(digit != "".to_string())
    {
      if(place%3 == 2)//hundreds place
      {
        digit = digit+ &" hundred".to_string();
      }
      else if(place%3 == 1 && i != '0')//if tens place is not zero
      {
        skip = true; 
	place -= 1;
      }
    }
    place -= 1;
  }
  //return_string = digits.to_string();
  
  /*
  for i in n.to_string().chars()
  {
    if i.to_string() != "0".to_string()
    {
      return_string = "".to_string();
    }
    if i.to_string() == "1".to_string()

    {
      n_string[index] = "one".to_string();
    }
    else if i.to_string() == "2".to_string()

    {
      n_string[index] = "two".to_string(); 
    }
    else if i.to_string() == "3".to_string()

    {
      n_string[index] = "three".to_string(); 
    }
    else if i.to_string() == "4".to_string()

    {
      n_string[index] = "four".to_string(); 
    }
    else if i.to_string() == "5".to_string()

    {
      n_string[index] = "five".to_string(); 
    }
    else if i.to_string() == "6".to_string()

    {
      n_string[index] = "six".to_string(); 
    }
    else if i.to_string() == "7".to_string()

    {
      n_string[index] = "seven".to_string(); 
    }
    else if i.to_string() == "8".to_string()

    {
      n_string[index] = "eight".to_string(); 
    }
    else if i.to_string() == "9".to_string()

    {
      n_string[index] = "nine".to_string(); 
    }
    index += 1; 
  }   
  for i in 0..n_string.len()
  {
    /*
    if i %3 ==1 && n_string[i].to_string() != "0".to_string()
    {
      if n_string[i].to_string() == "one".to_string()
      {
        if n_string[i-1] == "0"
	{
	  n_string[i] = "ten".to_string(); 
	}
        else if  n_string[i-1] == "one".to_string()
	{
	  n_string[i] = "eleven".to_string();
        }
        else if n_string[i-1] == "two".to_string()
	{
	  n_string[i] = "twelve".to_string();
        }
        else if n_string[i-1] == "three".to_string()
	{
	  n_string[i] = "thirteen".to_string();
        }
        else if n_string[i-1] == "four".to_string()
	{
	  n_string[i] = "fourteen".to_string();
        }
        else if n_string[i-1] == "five".to_string()
	{
	  n_string[i] = "fifteen".to_string();
        }
	else
        {
	  n_string[i] = n_string[i].to_string() + &"teen".to_string();
        }
        n_string[i-1]= "0".to_string();
      }
      else if n_string[i].to_string() == "two".to_string()
      {
	n_string[i] = "twenty-".to_string();
      }
      else if n_string[i].to_string() == "three".to_string()
      {
	n_string[i] = "thirty-".to_string();
      }
      else if n_string[i].to_string() == "five".to_string()
      {
	n_string[i] = "fifty-".to_string();
      }
      else
      {
	n_string[i] = n_string[i].to_string() + &"ty-".to_string();
      }
    }
    else if i %3 ==2 
    {
      if n_string[i].to_string() != "0".to_string()
      {
        n_string[i] = n_string[i].to_string() + &" hundred".to_string(); 
      } 
        if n_string[i].to_string() != "0".to_string()
        {
	  if i ==5
	  {
            n_string[i] = n_string[i].to_string() + &" thousand".to_string();  
	  }
	  else if i ==8
	  {
            n_string[i] = n_string[i].to_string() + &" million".to_string();  
	  }
	  else if i ==11
	  {
            n_string[i] = n_string[i].to_string() + &" billion".to_string();  
	  }
	}
	else if n_string[i-1].to_string() != "0".to_string()
	{
	  if i ==5
	  {
            n_string[i-1] = n_string[i-1].to_string() + &" thousand".to_string();  
	  }
	  else if i ==8
	  {
            n_string[i-1] = n_string[i-1].to_string() + &" million".to_string();  
	  }
	  else if i ==11
	  {
            n_string[i-1] = n_string[i-1].to_string() + &" billion".to_string();  
	  }

	}
	else if n_string[i-2].to_string() != "0".to_string()
	{
	  if i ==5
	  {
            n_string[i-2] = n_string[i-2].to_string() + &" thousand".to_string();  
	  }
	  else if i ==8
	  {
            n_string[i-2] = n_string[i-2].to_string() + &" million".to_string();  
	  }
	  else if i ==11
	  {
            n_string[i-2] = n_string[i-2].to_string() + &" billion".to_string();  
	  }

	}
    }*/
  if 1==1// n_string[i].to_string() != "0".to_string()
  {
    return_string = return_string.to_string() + &n_string[i].to_string(); 
    return_string = return_string.to_string() + &" ".to_string();
  }

  }  
  */
  return_string.to_string()
}
