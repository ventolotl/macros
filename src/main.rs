fn main() {
    compile!(
        let a "h" struct impl fn=>
        let b "i" struct impl fn=>
        let c " " struct impl fn=>
        let d "t" struct impl fn=>
        let e "h" struct impl fn=>
        let f "e" struct impl fn=>
        let g "r" struct impl fn=>
        let h "e" struct impl fn
    )
}

#[macro_export]
macro_rules! compile {
    ($(
    $($($($($($($($($($($($($($($($($($($($($(
    $($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($(
    $($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($(
    $($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($(
    $($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($(
    $($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($(
    $($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($(
    $($($($($($($($($(
        $v: vis $a: ident $b: ident $c: literal $d: ident $e: ident $f: ident $g: literal
    )=>*)
    _*)_*)_*)
    _*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)
    _*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)
    _*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)
    _*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)
    _*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)
    _*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)
    _*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)_*)
    _*)_*)_*)_*)_*)_*)_*)_*)_*)
    => {{
        $($($($($($($($($($($($($($($($($($($($($($(
        $($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($(
        $($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($(
        $($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($(
        $($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($(
        $($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($(
        $($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($($(
        $v $d $b {}
        $e $b {
            $f x() -> String {
                $c.to_string()
            }
        }
        )*
        let mut vec = vec![];
        $(vec.push(<$b>::x());)*
        println!("{}", vec.join(""));
        )*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*
        )*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*
        )*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*
        )*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*
        )*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*
        )*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*
        )*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*)*
    }};
}
