pub fn encode(n: u64) -> String {
  let mut n_string:[String;12] = ["0".to_string(),"0".to_string(),"0".to_string(),"0".to_string(),"0".to_string(),"0".to_string(),"0".to_string(),"0".to_string(),"0".to_string(),"0".to_string(),"0".to_string(),"0".to_string()];
  let mut return_string = "zero";
  let mut index = 0;
  if n > 999999999999
  {
    return "Out of Range!".to_string();

  }
  for i in n.to_string().chars()
  {
    if i.to_string() == "1"
    {
      n_string[index] = "one".to_string();
    }
    else if i.to_string() == "2"
    {
      n_string[index] = "two".to_string(); 
    }
    else if i.to_string() == "3"
    {
      n_string[index] = "three".to_string(); 
    }
    else if i.to_string() == "4"
    {
      n_string[index] = "four".to_string(); 
    }
    else if i.to_string() == "5"
    {
      n_string[index] = "five".to_string(); 
    }
    else if i.to_string() == "6"
    {
      n_string[index] = "six".to_string(); 
    }
    else if i.to_string() == "7"
    {
      n_string[index] = "seven".to_string(); 
    }
    else if i.to_string() == "8"
    {
      n_string[index] = "eight".to_string(); 
    }
    else if i.to_string() == "9"
    {
      n_string[index] = "nine".to_string(); 
    }
    index += 1; 
  }   
  for i in 0..n_string.len()
  {
    if(i %3 ==2)
    {
      if i==2 
      {
	//hundreds
      }
      else if i == 5
      {
        //thousands
      }
      else if i== 8
      {
        //millions
      }
      else if i== 11
      {
        //billions
      }
    }
  }  
  return_string.to_string()
}
