fn main() {
    println!(r"cargo:rustc-link-search=native=./third-party/p2p-compat/build");
    println!(r"cargo:rustc-link-lib=static=p2p-compat");
    println!(r"cargo:rustc-link-lib=static=p2p_network");
    println!(r"cargo:rustc-link-lib=static=bitcoin_utils");
    println!(r"cargo:rustc-link-lib=static=easylogging");
    println!(r"cargo:rustc-link-lib=static=miniupnpc");
    // Ubuntu
    //println!("cargo:rustc-link-lib=dylib=stdc++");
    // Windows
    println!(r"cargo:rustc-link-search=native=C:\Program Files (x86)\Windows Kits\10\Lib\10.0.18362.0\um\x64");     
    println!("cargo:rustc-link-lib=dylib=iphlpapi");   
}