use std::{fs, io::{self, /*Read,*/ Write}, path::{Path}, process::Command, thread, time::Duration};

fn pause_terminal(msg: &str) {
                print!("{}",msg);
                io::stdout().flush().unwrap();
                let mut temp = String::new();
                io::stdin().read_line(&mut temp).unwrap();
}
fn clear_screen() {
        let _ = Command::new("clear").status();
}
fn enter_choice() -> i32{
        println!("       ---------ONEPLUS 15 升降級工具 (Rust版)---------");
        println!("\x1b[1;31m====================================================");
        println!("!僅限[一加15]使用，若使用其他機型導致變磚，一概不負責!");
        println!("!請將ota包更改名稱為update.zip，並放入ota資料夾下!");
        println!("!若未解鎖bootloader，請勿使用本工具!");
        println!("!請先將手機重啟至bootloader模式再開始刷機!");
        println!("====================================================\x1b[0m");
        println!("=> 輸入你的選擇");
        println!("1. 一鍵安裝驅動 (Linux暫不支援 DPInst)");
        println!("2. 測試環境");
        println!("3. 解包全量包");
        println!("4. 清空ota資料夾");
        println!("5. 清空images資料夾");
        println!("6. 開始刷機");
        println!("7. END");
        print!("? ");

        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("讀取失敗");
        match input.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            println!("請輸入有效的數字！");
            0 // 回傳 0 讓迴圈繼續
        }
    }
}

fn environment() {
        loop{
                println!("\n[一些實用指令]");
                println!("1. adb devices");
                println!("2. fastboot devices");
                println!("3. 重啟至 bootloader");
                println!("4. 重啟至 Recovery");
                println!("5. 返回");
                print!("? ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                match input.trim().parse::<i32>() {
                        Ok(1) => {
                                println!("\n若只出現 List of devices attached 代表沒識別到裝置...\n");
                                Command::new("adb")
                                .arg("devices")
                                .status()
                                .expect("❌ 執行 adb 失敗，請確認已安裝 android-tools");
                        }
                        Ok(2) =>{
                                println!("\n若什麼都沒顯示代表未連接，請確認處於 fastboot 模式...\n");
                                Command::new("fastboot")
                                .arg("devices")
                                .status()
                                .expect(" ❌ 執行 fastboot 失敗");
                        }
                        Ok(3) => boot_to_bootloader(),
                        Ok(4) => boot_to_recovery(),
                        Ok(5) => break,
                        _ => println!("無效的選擇！"),
                }
        }
}
fn boot_to_bootloader() {
        println!("\n正在重啟至 bootloader...\n");
    
        // let _ = 代表：「我知道這指令可能會報錯(例如手機不在adb模式)，但我不在乎，繼續執行」
        // .args([...]) 可以直接傳入一個陣列，比拼接字串直覺太多了！
        let _ = Command::new("adb").args(["reboot", "bootloader"]).status();
        let _ = Command::new("fastboot").args(["reboot", "bootloader"]).status();
        println!("指令發送完畢。\n");
}
fn boot_to_recovery() {
        println!("\n正在重啟至 Recovery...\n");
        
        let _ = Command::new("adb").args(["reboot", "recovery"]).status();
        let _ = Command::new("fastboot").args(["reboot", "recovery"]).status();
        
        println!("指令發送完畢。\n");
}
fn clear_ota(){
        let dir = Path::new("ota");
        if dir.exists() {
                print!("確認刪除 ota 資料夾內容?[y/n] ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                if input.trim().eq_ignore_ascii_case("y") {
                        let _ = fs::remove_dir_all(dir);
                        let _ = fs::create_dir_all(dir);
                        pause_terminal("按 Enter 鍵繼續...");
                }
                else{
                        println!("已取消刪除");
                }
        }
        else{
                let _ = fs::create_dir_all(dir);
                println!("ota 資料夾不存在，已自動建立。");
        }
}
fn clear_images() {
        let dir = Path::new("images");
        if dir.exists() {
                println!("確認刪除 images 資料夾內容?[y/n]");
                io::stdout().flush().unwrap();
                let mut input =  String::new();
                io::stdin().read_line(&mut input).unwrap();
                if input.trim().eq_ignore_ascii_case("y") {
                        let _ = fs::remove_dir_all(dir);
                        let _ = fs::create_dir_all(dir);
                        pause_terminal("按 Enter 鍵繼續...");
                }
                else{
                        println!("已取消刪除");
                }
        }
        else{
                let _ = fs::create_dir_all(dir);
                println!("images 資料夾不存在，已自動建立。");
        }
}

fn unpack() {
        let ota = Path::new("ota");
        let images = Path::new("images");
        let zip = Path::new("ota/update.zip");
        let bin = Path::new("ota/payload.bin");
        if !zip.exists() {
                println!("ERROR:系統找不到 /ota/update.zip ");
                println!("請確認是否已將檔案改名為update.zip並放入ota資料夾下 ");
                pause_terminal("按 Enter 鍵繼續...");
                return;
        }
        let zip_result = Command::new("7z")
                .arg("x")
                .arg(zip)
                .arg("payload.bin")
                .arg(format!("-o{}",ota.display()))
                .status()
                .expect("❌ 呼叫 7z 失敗，請確認是否安裝 p7zip");
        if zip_result.success() {
                println!("解壓縮update.zip成功");
                pause_terminal("按任意鍵繼續提取payload.bin....");
        }
        else {
                println!("ERROR: 解壓縮失敗，請確認 update.zip 是否損壞。");
                pause_terminal("按 Enter 鍵繼續...");
                clear_screen();
                return;
        }
        if !bin.exists() {
                println!("提取payload.bin失敗");
                println!("確認檔案是否存在");
                pause_terminal("按 Enter 鍵繼續...");
                clear_screen();
                return;
        }
        println!("開始解包payload.bin");
        println!();
        let bin_result = Command::new("payload-dumper-go")
        .arg("-o")
        .arg(images)
        .arg(bin)
        .status()
        .expect("執行 payload-dumper-go 失敗，請確認已安裝");
        if bin_result.success() {
                println!("解包payload.bin成功");
        }
        pause_terminal("按 Enter 鍵繼續...");
}

fn start(){
        loop{
                println!("1.升/降級?");
                println!("2.救磚(限同版本刷入)");
                println!("3.返回");
                print!("?");
                io::stdout().flush().unwrap();
                let mut input =  String::new();
                io::stdin().read_line(&mut input).unwrap();
                match input.trim().parse::<i32>(){
                        Ok(1) => {
                                shengijanji();
                                break;
                        }
                        Ok(2) => {
                                jiuzhuan();
                                break;
                        }
                        _ => {
                                println!("Invalid choice")
                        }
                }
        }
}
fn shengijanji(){
        //let mut v: Vec<String> = Vec::new();
        let cow_partitions = [ "system", "system_dlkm", "system_ext", "vendor","product",
        "odm","my_bigball","my_carrier","my_engineering","my_heytap",
        "my_manifest","my_product","my_region", "my_stock", "odm_dlkm", "vendor_dlkm"
        ];
        let fw_partitions = [
                 "vbmeta", "vbmeta_system", "vbmeta_vendor", "vendor_boot", "init_boot", "boot",
        "recovery", "bluetooth", "cpucp", "cpucp_dtb", "dsp", "dtbo", "engineering_cdt",
        "featenabler", "oplus_sec", "shrm", "splash", "uefi", "aop", "aop_config",
        "devcfg", "hyp", "imagefv", "keymaster", "oplusstanvbk", "qupfw", "tz",
        "uefisecapp", "abl", "xbl", "xbl_config", "xbl_ramdump"
        ];
        let sys_partitions = [
                "my_bigball", "my_carrier", "my_engineering", "my_heytap", "my_manifest",
        "my_product", "my_region", "my_stock", "odm", "product", "system",
        "system_dlkm", "system_ext", "vendor", "vendor_dlkm"
        ];

        let img_dir = Path::new("images");

        let modem_path = Path::new("images/modem.img");
        if modem_path.exists() {
                for slot in ["a","b"]{
                        let partition_name = format!("modem_{}",slot);
                        let _ = Command::new("fastboot")
                        .args(["flash",&partition_name])
                        .arg(modem_path)
                        .status();
                }   
        }
        println!("\n正在進入Fastboot模式，請勿動手機和電腦");
        let _ = Command::new("fastboot").args(["reboot","fastboot"]).status();
        println!("Please wait for 10 seconds...");
        thread::sleep(Duration::from_secs(10));
        for s in cow_partitions{
                for slot in ["a","b"]{
                        let partition_name = format!("{}_{}-cow",s,slot);//{system}_{a}-cow
                        let _ = Command::new("fastboot")
                        .args(["delete-logical-partition",&partition_name])
                        .status();
                }
        }
        for s in fw_partitions{
                let img = format!("{}.img",s);
                let img_path = img_dir.join(&img);
                if img_path.exists() {
                        for slot in ["a","b"]{
                                let partition_name = format!("{}_{}",s , slot);
                                let _ = Command::new("fastboot")
                                .args(["flash",&partition_name])
                                .arg(&img_path)
                                .status();
                        }
                }
                else {
                        if s == "cpucp_dtb" || s == "oplusstanvbk" {
                                println!("\n[略過] 未找到可選分區 {}, 已自動忽略", img);
                        }
                }
        }
        for s in sys_partitions{
                let img = format!("{}.img",s);
                let img_path = img_dir.join(img);
                if img_path.exists() {
                        let _ = Command::new("fastboot")
                        .args(["flash",s])
                        .arg(img_path)
                        .status();
                }
        }
        println!("\n請在手機選擇語言，並選擇格式化數據後重啟");
        pause_terminal("\n刷機完成，按任意鍵返回主頁");
}
fn jiuzhuan(){

}


fn main() {
        let _ = fs::create_dir_all("images");
        let _ = fs::create_dir_all("tools");
        let _ = fs::create_dir_all("ota");
        loop {
                let choice = enter_choice();
                match choice {
                        1 => println!("待刪"),
                        2 => environment(),
                        3 => unpack(),
                        4 => clear_ota(),
                        5 => clear_images(),
                        6 => start(),
                        7=> {
                                println!("end");
                                break;
                        }
                        _ => println!("invalid choice"),
                }
                println!();
        }
}
