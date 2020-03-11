fn main() {
    if let Ok(var) = std::env::var("TAGGED_BOX_RESERVED_WIDTH") {
        match &*var {
            "63bits" => println!("cargo:rustc-cfg=tagged_box_reserve_63bits"),
            "62bits" => println!("cargo:rustc-cfg=tagged_box_reserve_62bits"),
            "61bits" => println!("cargo:rustc-cfg=tagged_box_reserve_61bits"),
            "60bits" => println!("cargo:rustc-cfg=tagged_box_reserve_60bits"),
            "59bits" => println!("cargo:rustc-cfg=tagged_box_reserve_59bits"),
            "58bits" => println!("cargo:rustc-cfg=tagged_box_reserve_58bits"),
            "57bits" => println!("cargo:rustc-cfg=tagged_box_reserve_57bits"),
            "56bits" => println!("cargo:rustc-cfg=tagged_box_reserve_56bits"),
            "55bits" => println!("cargo:rustc-cfg=tagged_box_reserve_55bits"),
            "54bits" => println!("cargo:rustc-cfg=tagged_box_reserve_54bits"),
            "53bits" => println!("cargo:rustc-cfg=tagged_box_reserve_53bits"),
            "52bits" => println!("cargo:rustc-cfg=tagged_box_reserve_52bits"),
            "51bits" => println!("cargo:rustc-cfg=tagged_box_reserve_51bits"),
            "50bits" => println!("cargo:rustc-cfg=tagged_box_reserve_50bits"),
            "49bits" => println!("cargo:rustc-cfg=tagged_box_reserve_49bits"),
            "48bits" => println!("cargo:rustc-cfg=tagged_box_reserve_48bits"),
            reserved => {
                println!("cargo:warning={} is an invalid reserved pointer width for `TAGGED_BOX_RESERVED_WIDTH`", reserved);
                println!("cargo:rustc-cfg=tagged_box_reserve_60bits")
            }
        }
    } else {
        println!("cargo:rustc-cfg=tagged_box_reserve_60bits");
    }
}
