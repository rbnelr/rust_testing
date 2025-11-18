
// This is stupid, as this literally produces 0+1+1+1 etc.
macro_rules! _ident_count {
	//($a:ident) => { 1 };
	//($($a:ident),*) => {
	//	0 $(
	//	+ count!($a)
	//	)*
	//};
	
	($a:ident) => { 1 };
	($a:ident,$b:ident) => { 2 };
	($a:ident,$b:ident,$c:ident) => { 3 };
	($a:ident,$b:ident,$c:ident,$d:ident) => { 4 };
	($a:ident,$b:ident,$c:ident,$d:ident,$e:ident) => { 5 };
	($a:ident,$b:ident,$c:ident,$d:ident,$e:ident,$f:ident) => { 6 };
	($a:ident,$b:ident,$c:ident,$d:ident,$e:ident,$f:ident,$g:ident) => { 7 };
	($a:ident,$b:ident,$c:ident,$d:ident,$e:ident,$f:ident,$g:ident,$h:ident) => { 8 };
	($a:ident,$b:ident,$c:ident,$d:ident,$e:ident,$f:ident,$g:ident,$h:ident, $($rest:ident),*) => {
		8 + _ident_count!($($rest),*)
	};
}

pub(crate) use _ident_count;
