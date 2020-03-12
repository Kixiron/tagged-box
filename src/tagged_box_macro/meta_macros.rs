// TODO: Currently broken from rust/#52234
// https://github.com/rust-lang/rust/pull/52234
#[allow(unused_macros)]
macro_rules! generate_tuple_expansion {
    (($dollar:tt, $others:ident) $(($field:tt, $ident:ident)),*) => {
        generate_tuple_expansion! {
            @inner ($dollar, $others, tuple)
                [
                // ($variant:path, $expr:expr) => {
                //     $variant()
                // };
                ($dollar variant:path, $dollar expr:expr) => {
                    $dollar variant()
                };
                // ($variant:path, $expr:expr,) => {
                //     $variant()
                // };
                ($dollar variant:path, $dollar expr:expr,) => {
                    $dollar variant()
                };
            ]
                [] [] $( ($field, $ident) )*
        }
    };

    (@inner ($dollar:tt, $others:ident, $tuple:ident) [$($finished:tt)*] [$($match:tt)*] [$($access:tt)*] ($field:tt, $ident:ident) $(($rest_fields:tt, $rest_idents:ident))*) => {
        generate_tuple_expansion! {
            @inner ($dollar, $others, $tuple)
                [
                    $( $finished )*
                // ($variant:path, $expr:expr, <rest of accesses>, <next access = $N:ty>) => {{
                //      let tuple = $expr;
                //      $variant(<accesses = tuple.N>)
                // }};
                ($dollar variant:path, $dollar expr:expr, $( $match )* $dollar $ident:ty) => {{
                    let $tuple = $dollar expr;
                    $dollar variant($( $access )* $tuple . $field)
                }};
            ]
                [ $( $match )* $dollar $ident:ty, ]
                [ $( $access )* $tuple . $field, ]
            $( ($rest_fields, $rest_idents) )*
        }
    };

    (@inner ($dollar:tt, $others:ident, $tuple:ident) [$($finished:tt)*] [$($match:tt)*] [$($access:tt)*]) => {
        generate_tuple_expansion! {
            @finish
                $( $finished )*
            // ($variant:path, $expr:expr, $($others:ty),*) => {
            //      compile_error!("Only enum tuple variants of up to 32 elements are supported");
            // };
            ($dollar variant:path, $dollar expr:expr, $dollar ( $dollar $others:ty),*) => {
                compile_error!("Only enum tuple variants of up to 32 elements are supported");
            };
        }
    };

    (@finish $($finished:tt)*) => {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! __expand_tuple {
                $( $finished )*
        }
    };
}

// Unexpanded Macro
// generate_tuple_expansion! {
//     ($, others)
//     (0, a), (1, b), (2, c), (3, d),
//     (4, e), (5, f), (6, g), (7, h),
//     (8, i), (9, j), (10, k), (11, l),
//     (12, m), (13, n), (14, o), (15, p),
//     (16, q), (17, r), (18, s), (19, t),
//     (20, u), (21, v), (22, w), (23, x),
//     (24, y), (25, z), (26, aa), (27, bb),
//     (28, cc), (29, dd), (30, ee), (31, ff)
// }

// Macro Expansion
#[doc(hidden)]
#[macro_export]
macro_rules! __expand_tuple {
    ($variant:path, $expr:expr) => {
        $variant()
    };
    ($variant:path, $expr:expr,) => {
        $variant()
    };
    ($variant:path, $expr:expr, $a:ty) => {{
        let tuple = $expr;
        $variant(tuple.0)
    }};
    ($variant:path, $expr:expr, $a:ty, $b:ty) => {{
        let tuple = $expr;
        $variant(tuple.0, tuple.1)
    }};
    ($variant:path, $expr:expr, $a:ty, $b:ty, $c:ty) => {{
        let tuple = $expr;
        $variant(tuple.0, tuple.1, tuple.2)
    }};
    ($variant:path, $expr:expr, $a:ty, $b:ty, $c:ty, $d:ty) => {{
        let tuple = $expr;
        $variant(tuple.0, tuple.1, tuple.2, tuple.3)
    }};
    ($variant:path, $expr:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty) => {{
        let tuple = $expr;
        $variant(tuple.0, tuple.1, tuple.2, tuple.3, tuple.4)
    }};
    ($variant:path, $expr:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty) => {{
        let tuple = $expr;
        $variant(tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5)
    }};
    ($variant:path, $expr:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty) => {{
        let tuple = $expr;
        $variant(
            tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5, tuple.6,
        )
    }};
    ($variant:path, $expr:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty) => {{
        let tuple = $expr;
        $variant(
            tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5, tuple.6, tuple.7,
        )
    }};
    ($variant:path, $expr:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty) => {{
        let tuple = $expr;
        $variant(
            tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5, tuple.6, tuple.7, tuple.8,
        )
    }};
    ($variant:path, $expr:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty) => {{
        let tuple = $expr;
        $variant(
            tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5, tuple.6, tuple.7, tuple.8,
            tuple.9,
        )
    }};
    ($variant:path, $expr:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty) => {{
        let tuple = $expr;
        $variant(
            tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5, tuple.6, tuple.7, tuple.8,
            tuple.9, tuple.10,
        )
    }};
    ($variant:path, $expr:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty) => {{
        let tuple = $expr;
        $variant(
            tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5, tuple.6, tuple.7, tuple.8,
            tuple.9, tuple.10, tuple.11,
        )
    }};
    ($variant:path, $expr:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty) => {{
        let tuple = $expr;
        $variant(
            tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5, tuple.6, tuple.7, tuple.8,
            tuple.9, tuple.10, tuple.11, tuple.12,
        )
    }};
    ($variant:path, $expr:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty) => {{
        let tuple = $expr;
        $variant(
            tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5, tuple.6, tuple.7, tuple.8,
            tuple.9, tuple.10, tuple.11, tuple.12, tuple.13,
        )
    }};
    ($variant:path, $expr:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty) => {{
        let tuple = $expr;
        $variant(
            tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5, tuple.6, tuple.7, tuple.8,
            tuple.9, tuple.10, tuple.11, tuple.12, tuple.13, tuple.14,
        )
    }};
    ($variant:path, $expr:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty) => {{
        let tuple = $expr;
        $variant(
            tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5, tuple.6, tuple.7, tuple.8,
            tuple.9, tuple.10, tuple.11, tuple.12, tuple.13, tuple.14, tuple.15,
        )
    }};
    ($variant:path, $expr:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty, $q:ty) => {{
        let tuple = $expr;
        $variant(
            tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5, tuple.6, tuple.7, tuple.8,
            tuple.9, tuple.10, tuple.11, tuple.12, tuple.13, tuple.14, tuple.15, tuple.16,
        )
    }};
    ($variant:path, $expr:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty, $q:ty, $r:ty) => {{
        let tuple = $expr;
        $variant(
            tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5, tuple.6, tuple.7, tuple.8,
            tuple.9, tuple.10, tuple.11, tuple.12, tuple.13, tuple.14, tuple.15, tuple.16,
            tuple.17,
        )
    }};
    ($variant:path, $expr:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty, $q:ty, $r:ty, $s:ty) => {{
        let tuple = $expr;
        $variant(
            tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5, tuple.6, tuple.7, tuple.8,
            tuple.9, tuple.10, tuple.11, tuple.12, tuple.13, tuple.14, tuple.15, tuple.16,
            tuple.17, tuple.18,
        )
    }};
    ($variant:path, $expr:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty, $q:ty, $r:ty, $s:ty, $t:ty) => {{
        let tuple = $expr;
        $variant(
            tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5, tuple.6, tuple.7, tuple.8,
            tuple.9, tuple.10, tuple.11, tuple.12, tuple.13, tuple.14, tuple.15, tuple.16,
            tuple.17, tuple.18, tuple.19,
        )
    }};
    ($variant:path, $expr:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty, $q:ty, $r:ty, $s:ty, $t:ty, $u:ty) => {{
        let tuple = $expr;
        $variant(
            tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5, tuple.6, tuple.7, tuple.8,
            tuple.9, tuple.10, tuple.11, tuple.12, tuple.13, tuple.14, tuple.15, tuple.16,
            tuple.17, tuple.18, tuple.19, tuple.20,
        )
    }};
    ($variant:path, $expr:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty, $q:ty, $r:ty, $s:ty, $t:ty, $u:ty, $v:ty) => {{
        let tuple = $expr;
        $variant(
            tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5, tuple.6, tuple.7, tuple.8,
            tuple.9, tuple.10, tuple.11, tuple.12, tuple.13, tuple.14, tuple.15, tuple.16,
            tuple.17, tuple.18, tuple.19, tuple.20, tuple.21,
        )
    }};
    ($variant:path, $expr:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty, $q:ty, $r:ty, $s:ty, $t:ty, $u:ty, $v:ty, $w:ty) => {{
        let tuple = $expr;
        $variant(
            tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5, tuple.6, tuple.7, tuple.8,
            tuple.9, tuple.10, tuple.11, tuple.12, tuple.13, tuple.14, tuple.15, tuple.16,
            tuple.17, tuple.18, tuple.19, tuple.20, tuple.21, tuple.22,
        )
    }};
    ($variant:path, $expr:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty, $q:ty, $r:ty, $s:ty, $t:ty, $u:ty, $v:ty, $w:ty, $x:ty) => {{
        let tuple = $expr;
        $variant(
            tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5, tuple.6, tuple.7, tuple.8,
            tuple.9, tuple.10, tuple.11, tuple.12, tuple.13, tuple.14, tuple.15, tuple.16,
            tuple.17, tuple.18, tuple.19, tuple.20, tuple.21, tuple.22, tuple.23,
        )
    }};
    ($variant:path, $expr:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty, $q:ty, $r:ty, $s:ty, $t:ty, $u:ty, $v:ty, $w:ty, $x:ty, $y:ty) => {{
        let tuple = $expr;
        $variant(
            tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5, tuple.6, tuple.7, tuple.8,
            tuple.9, tuple.10, tuple.11, tuple.12, tuple.13, tuple.14, tuple.15, tuple.16,
            tuple.17, tuple.18, tuple.19, tuple.20, tuple.21, tuple.22, tuple.23, tuple.24,
        )
    }};
    ($variant:path, $expr:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty, $q:ty, $r:ty, $s:ty, $t:ty, $u:ty, $v:ty, $w:ty, $x:ty, $y:ty, $z:ty) => {{
        let tuple = $expr;
        $variant(
            tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5, tuple.6, tuple.7, tuple.8,
            tuple.9, tuple.10, tuple.11, tuple.12, tuple.13, tuple.14, tuple.15, tuple.16,
            tuple.17, tuple.18, tuple.19, tuple.20, tuple.21, tuple.22, tuple.23, tuple.24,
            tuple.25,
        )
    }};
    ($variant:path, $expr:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty, $q:ty, $r:ty, $s:ty, $t:ty, $u:ty, $v:ty, $w:ty, $x:ty, $y:ty, $z:ty, $aa:ty) => {{
        let tuple = $expr;
        $variant(
            tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5, tuple.6, tuple.7, tuple.8,
            tuple.9, tuple.10, tuple.11, tuple.12, tuple.13, tuple.14, tuple.15, tuple.16,
            tuple.17, tuple.18, tuple.19, tuple.20, tuple.21, tuple.22, tuple.23, tuple.24,
            tuple.25, tuple.26,
        )
    }};
    ($variant:path, $expr:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty, $q:ty, $r:ty, $s:ty, $t:ty, $u:ty, $v:ty, $w:ty, $x:ty, $y:ty, $z:ty, $aa:ty, $bb:ty) => {{
        let tuple = $expr;
        $variant(
            tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5, tuple.6, tuple.7, tuple.8,
            tuple.9, tuple.10, tuple.11, tuple.12, tuple.13, tuple.14, tuple.15, tuple.16,
            tuple.17, tuple.18, tuple.19, tuple.20, tuple.21, tuple.22, tuple.23, tuple.24,
            tuple.25, tuple.26, tuple.27,
        )
    }};
    ($variant:path, $expr:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty, $q:ty, $r:ty, $s:ty, $t:ty, $u:ty, $v:ty, $w:ty, $x:ty, $y:ty, $z:ty, $aa:ty, $bb:ty, $cc:ty) => {{
        let tuple = $expr;
        $variant(
            tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5, tuple.6, tuple.7, tuple.8,
            tuple.9, tuple.10, tuple.11, tuple.12, tuple.13, tuple.14, tuple.15, tuple.16,
            tuple.17, tuple.18, tuple.19, tuple.20, tuple.21, tuple.22, tuple.23, tuple.24,
            tuple.25, tuple.26, tuple.27, tuple.28,
        )
    }};
    ($variant:path, $expr:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty, $q:ty, $r:ty, $s:ty, $t:ty, $u:ty, $v:ty, $w:ty, $x:ty, $y:ty, $z:ty, $aa:ty, $bb:ty, $cc:ty, $dd:ty) => {{
        let tuple = $expr;
        $variant(
            tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5, tuple.6, tuple.7, tuple.8,
            tuple.9, tuple.10, tuple.11, tuple.12, tuple.13, tuple.14, tuple.15, tuple.16,
            tuple.17, tuple.18, tuple.19, tuple.20, tuple.21, tuple.22, tuple.23, tuple.24,
            tuple.25, tuple.26, tuple.27, tuple.28, tuple.29,
        )
    }};
    ($variant:path, $expr:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty, $q:ty, $r:ty, $s:ty, $t:ty, $u:ty, $v:ty, $w:ty, $x:ty, $y:ty, $z:ty, $aa:ty, $bb:ty, $cc:ty, $dd:ty, $ee:ty) => {{
        let tuple = $expr;
        $variant(
            tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5, tuple.6, tuple.7, tuple.8,
            tuple.9, tuple.10, tuple.11, tuple.12, tuple.13, tuple.14, tuple.15, tuple.16,
            tuple.17, tuple.18, tuple.19, tuple.20, tuple.21, tuple.22, tuple.23, tuple.24,
            tuple.25, tuple.26, tuple.27, tuple.28, tuple.29, tuple.30,
        )
    }};
    ($variant:path, $expr:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty, $q:ty, $r:ty, $s:ty, $t:ty, $u:ty, $v:ty, $w:ty, $x:ty, $y:ty, $z:ty, $aa:ty, $bb:ty, $cc:ty, $dd:ty, $ee:ty, $ff:ty) => {{
        let tuple = $expr;
        $variant(
            tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5, tuple.6, tuple.7, tuple.8,
            tuple.9, tuple.10, tuple.11, tuple.12, tuple.13, tuple.14, tuple.15, tuple.16,
            tuple.17, tuple.18, tuple.19, tuple.20, tuple.21, tuple.22, tuple.23, tuple.24,
            tuple.25, tuple.26, tuple.27, tuple.28, tuple.29, tuple.30, tuple.31,
        )
    }};
    ($variant:path, $expr:expr, $($others:ty),*) => {
        compile_error!("Only enum tuple variants of up to 32 elements are supported");
    };
}

// TODO: Currently broken from rust/#52234
// https://github.com/rust-lang/rust/pull/52234
#[allow(unused_macros)]
macro_rules! generate_tuple_arm_expansion {
    (($dollar:tt, $others:ident) $($ident:ident),*) => {
        generate_tuple_arm_expansion! {
            @inner ($dollar, $others)
                [
                // ($tagged:expr, $enum:ident, $counter:ident, $variant:ident [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
                //     $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
                //          [
                //              $( $finished )*
                //              $enum::$variant() => $crate::TaggedBox::dangling($counter::$variant as _),
                //         ]
                //         $( $rest )*
                //     )
                // };
                ($dollar tagged:expr, $dollar enum:ident, $dollar counter:ident, $dollar variant:ident [$dollar ($dollar finished:tt)*] [$dollar ($dollar rest:tt)*] $dollar($dollar ty:ty),*) => {
                    $dollar crate::__taggable_into_box!(@inner $dollar tagged, $dollar enum, $dollar counter
                            [
                            $dollar ( $dollar finished )*
                            $dollar enum::$dollar variant() => $dollar crate::TaggedBox::dangling($dollar counter::$dollar variant as _),
                        ]
                        $dollar ( $dollar rest )*
                )
                };
                // ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
                //     $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
                //          [
                //              $( $finished )*
                //              $enum::$variant() => $crate::TaggedBox::<$enum>::dangling($counter::$variant as _),
                //         ]
                //         $( $rest )*
                //     )
                // };
                ($dollar tagged:expr, $dollar enum:ident, $dollar counter:ident, $dollar variant:ident, [$dollar ($dollar finished:tt)*] [$dollar ($dollar rest:tt)*] $dollar($dollar ty:ty),*) => {
                    $dollar crate::__taggable_into_box!(@inner $dollar tagged, $dollar enum, $dollar counter
                            [
                            $dollar ( $dollar finished )*
                            $dollar enum::$dollar variant() => $dollar crate::TaggedBox::<$dollar enum>::dangling($dollar counter::$dollar variant as _),
                        ]
                        $dollar ( $dollar rest )*
                )
                };
            ]
                [] [] $( $ident )*
        }
    };

    (@inner ($dollar:tt, $others:ident) [$($finished:tt)*] [$($match:tt)*] [$($access:tt)*] $ident:ident $($rest:ident)*) => {
        generate_tuple_arm_expansion! {
            @inner ($dollar, $others)
                [
                    $( $finished )*
                // ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty, $b:ty, $c:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
                //     $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
                //          [
                //              $( $finished )*
                //             $enum::$variant(a, b, c) => {
                //                  #[repr(C)]
                //                  struct $variant($( $ty ),*);
                //
                //                  $crate::TaggedBox::<$enum>::new($variant(a, b, c), $counter::$variant as _)
                //              }
                //         ]
                //         $( $rest )*
                //     )
                // };
                ($dollar tagged:expr, $dollar enum:ident, $dollar counter:ident, $dollar variant:ident, $( $match )* $dollar $ident:ty [$dollar ($dollar finished:tt)*] [$dollar ($dollar rest:tt)*] $dollar($dollar ty:ty),*) => {
                    $dollar crate::__taggable_into_box!(@inner $dollar tagged, $dollar enum, $dollar counter
                            [
                            $dollar ( $dollar finished )*
                            $dollar enum::$dollar variant($( $access, )* $ident) => {
                                #[repr(C)]
                                struct $dollar variant($dollar ($dollar ty),*);

                                $dollar crate::TaggedBox::<$dollar enum>::new::<$dollar variant>($dollar variant($( $access, )* $ident), $dollar counter::$dollar variant as _)
                            }
                        ]
                        $dollar ( $dollar rest )*
                )
                };
            ]
                [ $( $match )* $dollar $ident:ty, ]
                [ $( $access )* $ident ]
            $( $rest )*
        }
    };

    (@inner ($dollar:tt, $others:ident) [$($finished:tt)*] [$($match:tt)*] [$($access:tt)*]) => {
        generate_tuple_arm_expansion! {
            @finish
                $( $finished )*
            // ($variant:path, $tuple:expr, $($others:ty),* [$($finished_match:tt)*] [$($rest_match:tt)*]) => {
            //      compile_error!("Only enum tuple variants of up to 32 elements are supported");
            // };
            ($dollar tagged:expr, $dollar enum:ident, $dollar counter:ident, $dollar variant:ident, $dollar ( $dollar $others:ty),* [$dollar($dollar finished:tt)*] [$dollar($dollar rest:tt)*]) => {
                compile_error!("Only enum tuple variants of up to 32 elements are supported");
            };
        }
    };

    (@finish $($finished:tt)*) => {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! __expand_tuple_arm {
                $( $finished )*
        }
    };
}

// Unexpanded Macro
// generate_tuple_arm_expansion! {
//     ($, others)
//     a, b, c, d, e, f, g,
//     h, i, j, k, l, m, n,
//     o, p, q, r, s, t, u,
//     v, w, x, y, z, aa,
//     bb, cc, dd, ee, ff
// }

// Expanded Macro
#[doc(hidden)]
#[macro_export]
macro_rules! __expand_tuple_arm {
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident[$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
            [
                $( $finished )*
                $enum::$variant() => $crate::TaggedBox::<$enum>::dangling($counter::$variant as _),
            ]
            $( $rest )*
        )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident,[$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
            [
                $( $finished )*
                $enum::$variant() => $crate::TaggedBox::<$enum>::dangling($counter::$variant as _),
            ]
            $( $rest )*
        )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
        [
            $( $finished )*
            $enum::$variant(a) => {
                #[repr(C)]
                struct $variant($( $ty ),*);

                $crate::TaggedBox::new::<$variant>($variant(a), $counter::$variant as _)
            }
        ]
        $( $rest )*
      )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty, $b:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
        [
            $( $finished )*
            $enum::$variant(a,b) => {
                #[repr(C)]
                struct $variant($( $ty ),*);

                $crate::TaggedBox::new::<$variant>($variant(a,b), $counter::$variant as _)
            }
        ]
        $( $rest )*
      )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty, $b:ty, $c:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
        [
            $( $finished )*
            $enum::$variant(a,b,c) => {
                #[repr(C)]
                struct $variant($( $ty ),*);

                $crate::TaggedBox::new::<$variant>($variant(a,b,c), $counter::$variant as _)
            }
        ]
        $( $rest )*
      )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty, $b:ty, $c:ty, $d:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
        [
            $( $finished )*
            $enum::$variant(a,b,c,d) => {
                #[repr(C)]
                struct $variant($( $ty ),*);

                $crate::TaggedBox::new::<$variant>($variant(a,b,c,d), $counter::$variant as _)
            }
        ]
        $( $rest )*
      )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
        [
            $( $finished )*
            $enum::$variant(a,b,c,d,e) => {
                #[repr(C)]
                struct $variant($( $ty ),*);

                $crate::TaggedBox::new::<$variant>($variant(a,b,c,d,e), $counter::$variant as _)
            }
        ]
        $( $rest )*
      )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
        [
            $( $finished )*
            $enum::$variant(a,b,c,d,e,f) => {
                #[repr(C)]
                struct $variant($( $ty ),*);

                $crate::TaggedBox::new::<$variant>($variant(a,b,c,d,e,f), $counter::$variant as _)
            }
        ]
        $( $rest )*
      )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
        [
            $( $finished )*
            $enum::$variant(a,b,c,d,e,f,g) => {
                #[repr(C)]
                struct $variant($( $ty ),*);

                $crate::TaggedBox::new::<$variant>($variant(a,b,c,d,e,f,g), $counter::$variant as _)
            }
        ]
        $( $rest )*
      )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
        [
            $( $finished )*
            $enum::$variant(a,b,c,d,e,f,g,h) => {
                #[repr(C)]
                struct $variant($( $ty ),*);

                $crate::TaggedBox::new::<$variant>($variant(a,b,c,d,e,f,g,h), $counter::$variant as _)
            }
        ]
        $( $rest )*
      )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
        [
            $( $finished )*
            $enum::$variant(a,b,c,d,e,f,g,h,i) => {
                #[repr(C)]
                struct $variant($( $ty ),*);

                $crate::TaggedBox::new::<$variant>($variant(a,b,c,d,e,f,g,h,i), $counter::$variant as _)
            }
        ]
        $( $rest )*
      )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
        [
            $( $finished )*
            $enum::$variant(a,b,c,d,e,f,g,h,i,j) => {
                #[repr(C)]
                struct $variant($( $ty ),*);

                $crate::TaggedBox::new::<$variant>($variant(a,b,c,d,e,f,g,h,i,j), $counter::$variant as _)
            }
        ]
        $( $rest )*
      )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
        [
            $( $finished )*
            $enum::$variant(a,b,c,d,e,f,g,h,i,j,k) => {
                #[repr(C)]
                struct $variant($( $ty ),*);

                $crate::TaggedBox::new::<$variant>($variant(a,b,c,d,e,f,g,h,i,j,k), $counter::$variant as _)
            }
        ]
        $( $rest )*
      )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
        [
            $( $finished )*
            $enum::$variant(a,b,c,d,e,f,g,h,i,j,k,l) => {
                #[repr(C)]
                struct $variant($( $ty ),*);

                $crate::TaggedBox::new::<$variant>($variant(a,b,c,d,e,f,g,h,i,j,k,l), $counter::$variant as _)
            }
        ]
        $( $rest )*
      )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
        [
            $( $finished )*
            $enum::$variant(a,b,c,d,e,f,g,h,i,j,k,l,m) => {
                #[repr(C)]
                struct $variant($( $ty ),*);

                $crate::TaggedBox::new::<$variant>($variant(a,b,c,d,e,f,g,h,i,j,k,l,m), $counter::$variant as _)
            }
        ]
        $( $rest )*
      )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
        [
            $( $finished )*
            $enum::$variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n) => {
                #[repr(C)]
                struct $variant($( $ty ),*);

                $crate::TaggedBox::new::<$variant>($variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n), $counter::$variant as _)
            }
        ]
        $( $rest )*
      )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
        [
            $( $finished )*
            $enum::$variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o) => {
                #[repr(C)]
                struct $variant($( $ty ),*);

                $crate::TaggedBox::new::<$variant>($variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o), $counter::$variant as _)
            }
        ]
        $( $rest )*
      )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
        [
            $( $finished )*
            $enum::$variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p) => {
                #[repr(C)]
                struct $variant($( $ty ),*);

                $crate::TaggedBox::new::<$variant>($variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p), $counter::$variant as _)
            }
        ]
        $( $rest )*
      )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty, $q:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
        [
            $( $finished )*
            $enum::$variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q) => {
                #[repr(C)]
                struct $variant($( $ty ),*);

                $crate::TaggedBox::new::<$variant>($variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q), $counter::$variant as _)
            }
        ]
        $( $rest )*
      )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty, $q:ty, $r:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
        [
            $( $finished )*
            $enum::$variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r) => {
                #[repr(C)]
                struct $variant($( $ty ),*);

                $crate::TaggedBox::new::<$variant>($variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r), $counter::$variant as _)
            }
        ]
        $( $rest )*
      )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty, $q:ty, $r:ty, $s:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
        [
            $( $finished )*
            $enum::$variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s) => {
                #[repr(C)]
                struct $variant($( $ty ),*);

                $crate::TaggedBox::new::<$variant>($variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s), $counter::$variant as _)
            }
        ]
        $( $rest )*
      )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty, $q:ty, $r:ty, $s:ty, $t:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
        [
            $( $finished )*
            $enum::$variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t) => {
                #[repr(C)]
                struct $variant($( $ty ),*);

                $crate::TaggedBox::new::<$variant>($variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t), $counter::$variant as _)
            }
        ]
        $( $rest )*
      )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty, $q:ty, $r:ty, $s:ty, $t:ty, $u:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
        [
            $( $finished )*
            $enum::$variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u) => {
                #[repr(C)]
                struct $variant($( $ty ),*);

                $crate::TaggedBox::new::<$variant>($variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u), $counter::$variant as _)
            }
        ]
        $( $rest )*
      )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty, $q:ty, $r:ty, $s:ty, $t:ty, $u:ty, $v:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
        [
            $( $finished )*
            $enum::$variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v) => {
                #[repr(C)]
                struct $variant($( $ty ),*);

                $crate::TaggedBox::new::<$variant>($variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v), $counter::$variant as _)
            }
        ]
        $( $rest )*
      )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty, $q:ty, $r:ty, $s:ty, $t:ty, $u:ty, $v:ty, $w:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
        [
            $( $finished )*
            $enum::$variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w) => {
                #[repr(C)]
                struct $variant($( $ty ),*);

                $crate::TaggedBox::new::<$variant>($variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w), $counter::$variant as _)
            }
        ]
        $( $rest )*
      )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty, $q:ty, $r:ty, $s:ty, $t:ty, $u:ty, $v:ty, $w:ty, $x:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
        [
            $( $finished )*
            $enum::$variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x) => {
                #[repr(C)]
                struct $variant($( $ty ),*);

                $crate::TaggedBox::new::<$variant>($variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x), $counter::$variant as _)
            }
        ]
        $( $rest )*
      )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty, $q:ty, $r:ty, $s:ty, $t:ty, $u:ty, $v:ty, $w:ty, $x:ty, $y:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
        [
            $( $finished )*
            $enum::$variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,y) => {
                #[repr(C)]
                struct $variant($( $ty ),*);

                $crate::TaggedBox::new::<$variant>($variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,y), $counter::$variant as _)
            }
        ]
        $( $rest )*
      )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty, $q:ty, $r:ty, $s:ty, $t:ty, $u:ty, $v:ty, $w:ty, $x:ty, $y:ty, $z:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
        [
            $( $finished )*
            $enum::$variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,y,z) => {
                #[repr(C)]
                struct $variant($( $ty ),*);

                $crate::TaggedBox::new::<$variant>($variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,y,z), $counter::$variant as _)
            }
        ]
        $( $rest )*
      )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty, $q:ty, $r:ty, $s:ty, $t:ty, $u:ty, $v:ty, $w:ty, $x:ty, $y:ty, $z:ty, $aa:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
        [
            $( $finished )*
            $enum::$variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,y,z,aa) => {
                #[repr(C)]
                struct $variant($( $ty ),*);

                $crate::TaggedBox::new::<$variant>($variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,y,z,aa), $counter::$variant as _)
            }
        ]
        $( $rest )*
      )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty, $q:ty, $r:ty, $s:ty, $t:ty, $u:ty, $v:ty, $w:ty, $x:ty, $y:ty, $z:ty, $aa:ty, $bb:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
        [
            $( $finished )*
            $enum::$variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,y,z,aa,bb) => {
                #[repr(C)]
                struct $variant($( $ty ),*);

                $crate::TaggedBox::new::<$variant>($variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,y,z,aa,bb), $counter::$variant as _)
            }
        ]
        $( $rest )*
      )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty, $q:ty, $r:ty, $s:ty, $t:ty, $u:ty, $v:ty, $w:ty, $x:ty, $y:ty, $z:ty, $aa:ty, $bb:ty, $cc:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
        [
            $( $finished )*
            $enum::$variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,y,z,aa,bb,cc) => {
                #[repr(C)]
                struct $variant($( $ty ),*);

                $crate::TaggedBox::new::<$variant>($variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,y,z,aa,bb,cc), $counter::$variant as _)
            }
        ]
        $( $rest )*
      )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty, $q:ty, $r:ty, $s:ty, $t:ty, $u:ty, $v:ty, $w:ty, $x:ty, $y:ty, $z:ty, $aa:ty, $bb:ty, $cc:ty, $dd:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
        [
            $( $finished )*
            $enum::$variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,y,z,aa,bb,cc,dd) => {
                #[repr(C)]
                struct $variant($( $ty ),*);

                $crate::TaggedBox::new::<$variant>($variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,y,z,aa,bb,cc,dd), $counter::$variant as _)
            }
        ]
        $( $rest )*
      )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty, $q:ty, $r:ty, $s:ty, $t:ty, $u:ty, $v:ty, $w:ty, $x:ty, $y:ty, $z:ty, $aa:ty, $bb:ty, $cc:ty, $dd:ty, $ee:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
        [
            $( $finished )*
            $enum::$variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,y,z,aa,bb,cc,dd,ee) => {
                #[repr(C)]
                struct $variant($( $ty ),*);

                $crate::TaggedBox::new::<$variant>($variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,y,z,aa,bb,cc,dd,ee), $counter::$variant as _)
            }
        ]
        $( $rest )*
      )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty, $m:ty, $n:ty, $o:ty, $p:ty, $q:ty, $r:ty, $s:ty, $t:ty, $u:ty, $v:ty, $w:ty, $x:ty, $y:ty, $z:ty, $aa:ty, $bb:ty, $cc:ty, $dd:ty, $ee:ty, $ff:ty [$($finished:tt)*] [$($rest:tt)*] $($ty:ty),*) => {
      $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
        [
            $( $finished )*
            $enum::$variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,y,z,aa,bb,cc,dd,ee,ff) => {
                #[repr(C)]
                struct $variant($( $ty ),*);

                $crate::TaggedBox::new::<$variant>($variant(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,y,z,aa,bb,cc,dd,ee,ff), $counter::$variant as _)
            }
        ]
        $( $rest )*
      )
    };
    ($tagged:expr, $enum:ident, $counter:ident, $variant:ident, $($others:ty),*[$($finished:tt)*] [$($rest:tt)*]) => {
      compile_error!("Only enum tuple variants of up to 32 elements are supported");
    };
  }
