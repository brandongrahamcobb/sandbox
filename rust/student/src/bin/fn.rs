struct Template {}

fn template_function(f: impl FnOnce(&mut Template)) {}
// impl FnOnce is like a lambda which gives you the abilitity
// to call
//
// template_function(|any_name| {
//     any_name.property = String::new()
// });
