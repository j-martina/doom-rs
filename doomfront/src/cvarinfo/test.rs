use super::*;

#[test]
fn smoke() {
	const SOURCE: &str = r#"
server int egghead_roundabout;
user float acidSurge = 0.4;
cheat noarchive nosave string BONELESS_VENTURES = "Welcome to the Company !";
"#;

	let pt = parse_recov(SOURCE).unwrap();

	for err in pt.errors() {
		println!("{err}");
	}

	let defs: Vec<ast::CVar> = pt.ast().collect();

	assert_eq!(defs[0].name().text(), "egghead_roundabout");
	assert_eq!(defs[1].name().text(), "acidSurge");
	assert_eq!(defs[2].name().text(), "BONELESS_VENTURES");

	assert_eq!(defs[0].type_spec().kind(), Syn::TypeInt);
	assert_eq!(defs[1].type_spec().kind(), Syn::TypeFloat);
	assert_eq!(defs[2].type_spec().kind(), Syn::TypeString);

	let default_0 = defs[0].default();
	let default_1 = defs[1].default().unwrap();
	let default_2 = defs[2].default().unwrap();

	assert_eq!(default_0, None);
	assert_eq!(default_1.literal().kind(), Syn::LitFloat);
	assert_eq!(default_1.literal().text(), "0.4");
	assert_eq!(default_2.literal().kind(), Syn::LitString);
	assert_eq!(default_2.literal().text(), "\"Welcome to the Company !\"");
}