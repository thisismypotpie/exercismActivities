pub fn is_leap_year(year: u64) -> bool {
    // unimplemented!("true if {} is a leap year", year)
    if year as i32 % 4 == 0 {
        if year as i32 % 100 == 0 {
            if year as i32 % 400 == 0 {
                return true;
            }
            return false;
        }
        return true;
    }
    return false;
}
