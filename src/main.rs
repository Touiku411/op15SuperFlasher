use std::{fs, io::{self, Read, Write}, path::Path, process::Command};

fn pause_terminal() {
                print!("按 Enter 鍵繼續...");
                io::stdout().flush().unwrap();
                let mut temp = String::new();
                io::stdin().read_line(&mut temp).unwrap();
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
                io::stdout().flush() .unwrap();
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
                        pause_terminal();
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
                        pause_terminal();
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
fn main() {
        let _ = fs::create_dir_all("images");
        let _ = fs::create_dir_all("tools");
        let _ = fs::create_dir_all("ota");
        loop {
                let choice = enter_choice();
                match choice {
                        1 => println!("執行：一鍵安裝驅動..."),
                        2 => environment(),
                        3 => println!("執行：解包全量包..."),
                        4 => clear_ota(),
                        5 => clear_images(),
                        6 => println!("執行：開始刷機..."),
                        7=> {
                                println!("結束");
                                break;
                        }
                        _ => println!("無效選擇"),
                }
                println!();
        }
}
