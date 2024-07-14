mod alias;
use crate::alias::Res;
use std::io;
use colored::*;

// 数値変換可能かどうか
fn is_integer(s:&str)->bool{
    s.parse::<i32>().is_ok()
}
// 入力値のxyが指定のサイズの譜面の範囲内かどうか
fn check_range_input(size:i32,x:i32,y:i32)->Res<(),String>{
    if size < 2{
        return Err("譜面サイズは少なくとも2以上必要です".to_string())
    }
    else if size < x || size < y {
        return Err("座標がサイズ範囲外です".to_string())
    }
    Ok(())
}
// オセロ用に入力値をチェック size: 譜面サイズ input: 入力値(x,y)
fn check_input(size:i32,input: &str) -> Result<(), String> {
    let input: Vec<&str> = input.split(",").collect();
    if input.len() < 2 {
        return Err("入力値が少なすぎます".to_string());
    }
    if !is_integer(input[0]) || !is_integer(input[1]) {
        return Err("入力値が正しい形式ではありません".to_string());
    }
    let n1 = input[0].parse::<i32>().unwrap();
    let n2 = input[1].parse::<i32>().unwrap();
    check_range_input(size,n1,n2)?;
    Ok(())
}
// 入力値からxy座標のタプルを取得
fn get_input_pos(input:&str)->Res<(i32,i32)>{
    let input: Vec<&str> = input.split(",").collect();
    let x = input[0].parse::<i32>()?;
    let y = input[0].parse::<i32>()?;
    Ok((x,y))
}
// 駒をひっくり返す
fn flip_stone(){
}
fn place_stone(size: i32, board_data: &mut [i8], x: i32, y: i32, color: i8) -> bool {
    // 譜面サイズに収まっているなら
    if x >= 0 && y >= 0 && x < size && y < size {
        // 二次元座標を一次元配列のインデックスに変換
        let index = x * size + y;
        // もしその譜面が空(2)なら
        if board_data[index as usize] == 2 {
            // 駒を置く
            board_data[index as usize] = color;
            return true;
        }
    }
    return false;
}
fn init_message(){
    println!("{}","おせろー".white().on_blue());
    println!("{}","やり方は--helpで確認してくださいー".white().on_blue());
}
/**おせろのコード**/
/**基本的にプレイヤーが先行***/

fn main() -> Res<()> {
    // 譜面サイズ
    const BOARD_SIZE: usize = 8;
    let kuro = "◯ ";
    let shiro = "● ";
    let mut input_color = String::new();    // プレイヤーの色 
    let mut winner = -1; // 勝敗(プレイヤー: 0 CPU: 1)
                                   // 初期譜面データ
                                   // 白譜面: 0 黒譜面: 1 空譜面: 2
    let mut board_data: [i8; BOARD_SIZE * BOARD_SIZE] = [
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 0, 2,
        2, 2, 2, 2, 2, 0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2,
    ];
    init_message();
    println!("board size: {}x{}", BOARD_SIZE, BOARD_SIZE);
    println!("おせろの色を選択してくださーい(白:0 黒:1)");
    io::stdin().read_line(&mut input_color)?;
    

    loop {
        // 譜面枠の描画(上)
        for _ in 0..5 {
            print!("{}","+--".white().on_green());
        }
        print!("{}","+".white().on_green());

        // 譜面データの描画
        for (i, &mut data) in board_data.iter_mut().enumerate() {
            if i % 8 == 0 {
                print!("\n");
            }
            if data == 2 {
                print!("□ ");
            } else if data == 0 {
                print!("{}", shiro);
            } else if data == 1 {
                print!("{}", kuro);
            }
        }
        println!();
        // 譜面枠の描画(下)
        for _ in 0..5 {
            print!("{}","+--".white().on_green());
        }
        println!("{}","+".white().on_green());
        let mut input = String::new(); // プレイヤーの入力
        // プレイヤーの入力を受け取る
        io::stdin().read_line(&mut input)?;
        let input = input.trim().clone();
        // 入力値のチェック
        check_input(BOARD_SIZE as i32,&input).unwrap();
        // 入力値を数値に変換
        let (x,y) = get_input_pos(&input)?;
        let color = input_color.trim().parse::<i8>().unwrap();
        //println!("x: {} y: {} input: {}",x,y,input);
        place_stone(BOARD_SIZE as i32,&mut board_data.as_mut_slice(), x,y,color);

        // 入力が空か勝敗が決まった場合のみ終了
        if input.is_empty() || winner != -1 {
           break;
        }
    }
    // 勝敗
    if winner == 0{
        println!("プレイヤーの勝利!");
    }else if winner == 1{
        println!("CPUの勝利!");
    }
    Ok(())
}
