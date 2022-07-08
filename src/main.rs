fn main() {
    let lua = mlua::Lua::new();
    let x = lua.load("3.1415926535").eval::<f32>().unwrap();
    println!("{x}");
    let _ = rfd::FileDialog::new().pick_file();
    let x = lua.load("3.1415926535").eval::<f32>().unwrap();
    println!("{x}");
}
