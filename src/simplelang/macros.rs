macro_rules! map(
    { $($kindey:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($kindey, $value);
            )+
            m
        }
     };
);

macro_rules! dump {
	($value:expr $(, $values:expr)*) => {{
		print!("{:?}", $value);
		$( print!(" {:?}", $values); )*
		println!();
	}}
}

/*
 * tupple assignment:
 *   ( [let] a, [let] b ) = tuple
 */
macro_rules! assign {
	{ ( $item1:ident, $item2:ident ) = $rhs:expr }  => {
		let tuple_value = $rhs;
		$item1 = tuple_value.0;
		$item2 = tuple_value.1;
	};
	{ ( let $item1:ident, $item2:ident ) = $rhs:expr }  => {
		let tuple_value = $rhs;
		let $item1 = tuple_value.0;
		$item2 = tuple_value.1;
	};
	{ ( $item1:ident, let $item2:ident ) = $rhs:expr }  => {
		let tuple_value = $rhs;
		$item1 = tuple_value.0;
		let $item2 = tuple_value.1;
	};
	{ ( let $item1:ident, let $item2:ident ) = $rhs:expr }  => {
		let tuple_value = $rhs;
		let $item1 = tuple_value.0;
		let $item2 = tuple_value.1;
	};
}
