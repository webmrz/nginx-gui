fn main() {
    tauri_build::build();
    
    // 获取项目根目录
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_dir = std::path::Path::new(&manifest_dir);
    
    // 检查 nginx-1.24.0 目录是否存在
    let nginx_dir = manifest_dir.join("resources").join("nginx-1.24.0");
    println!("[DEBUG] Checking Nginx directory: {:?}", nginx_dir);
    
    if !nginx_dir.exists() {
        println!("[WARN] Nginx directory does not exist: {:?}", nginx_dir);
        println!("[INFO] Please ensure the nginx-1.24.0 directory exists in the resources folder");
        println!("[INFO] Expected path: {:?}", nginx_dir);
    } else {
        println!("[INFO] Nginx directory found at: {:?}", nginx_dir);
    }
}
