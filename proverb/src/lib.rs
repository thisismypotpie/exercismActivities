pub fn build_proverb(_list: &[&str]) -> String {
    //unimplemented!("build a proverb from this list of items: {:?}", list)
    let mut proverb = "".to_string();
    for i in 0.._list.len() {
        if i == _list.len() - 1 {
            proverb += &*format!("And all for the want of a {}.", _list[0]);
        } else {
            proverb += &*format!(
                "For want of a {} the {} was lost.\n",
                _list[i],
                _list[i + 1]
            );
        }
    }

    proverb.to_string()
}
