pub fn check(candidate: &str) -> bool {
  for i in candidate.chars()
  {
    for j in candidate.chars()
    {
      if i == j && i !='-' && i != ' '
      {
	return false;
      }
    }
  }
  true
}
//will always return false since each letter compares itself.
