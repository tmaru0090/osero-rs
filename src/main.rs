mod alias;
use crate::alias::Res;
use std::io;
use colored::*;
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

// 初期盤面のデータ生成
fn init_board(size: usize) -> Vec<i8> {
    let mut board = vec![2; size * size];
    let mid = size / 2;
    board[(mid * size + mid - 1) as usize] = 0;
    board[(mid * size + mid) as usize] = 1;
    board[(mid * size + mid - size - 1) as usize] = 1;
    board[(mid * size + mid - size) as usize] = 0;
    board
}
fn choose_best_move(size: i32, board_data: &[i8], color: i8) -> Option<(i32, i32)> {
    let valid_moves = get_valid_moves(size, board_data, color);
    let mut best_move = None;
    let mut max_flips = 0;

    for (x, y) in valid_moves {
        let mut temp_board = board_data.to_vec();
        let flips = count_flips(size, &mut temp_board, x, y, color);

        if flips > max_flips {
            max_flips = flips;
            best_move = Some((x, y));
        }
    }

    best_move
}

// 駒を置いたときにひっくり返る駒の数を数える関数
fn count_flips(size: i32, board_data: &mut [i8], x: i32, y: i32, color: i8) -> i32 {
    let directions = [
        (1, 0),  // 右
        (0, 1),  // 下
        (-1, 0), // 左
        (0, -1), // 上
        (1, 1),  // 右下
        (1, -1), // 右上
        (-1, 1), // 左下
        (-1, -1) // 左上
    ];

    let mut total_flips = 0;

    for (dx, dy) in directions.iter() {
        let mut cur_x = x + dx;
        let mut cur_y = y + dy;
        let mut flips = 0;

        while cur_x >= 0 && cur_x < size && cur_y >= 0 && cur_y < size {
            let index = (cur_x * size + cur_y) as usize;
            match board_data[index] {
                2 => break,
                _ if board_data[index] == color => {
                    total_flips += flips;
                    break;
                }
                _ => {
                    flips += 1;
                }
            }
            cur_x += dx;
            cur_y += dy;
        }
    }

    total_flips
}
// どこに駒をひっくり返すことができるか
fn get_valid_moves(size: i32, board_data: &[i8], color: i8) -> Vec<(i32, i32)> {
    let directions = [
        (1, 0),  // 右
        (0, 1),  // 下
        (-1, 0), // 左
        (0, -1), // 上
        (1, 1),  // 右下
        (1, -1), // 右上
        (-1, 1), // 左下
        (-1, -1) // 左上
    ];
    let mut valid_moves = Vec::new();

    for x in 0..size {
        for y in 0..size {
            if board_data[(x * size + y) as usize] == 2 {
                for (dx, dy) in directions.iter() {
                    let mut cur_x = x + dx;
                    let mut cur_y = y + dy;
                    let mut has_opponent_between = false;

                    while cur_x >= 0 && cur_x < size && cur_y >= 0 && cur_y < size {
                        let index = (cur_x * size + cur_y) as usize;
                        match board_data[index] {
                            2 => break,
                            _ if board_data[index] == color => {
                                if has_opponent_between {
                                    valid_moves.push((x, y));
                                }
                                break;
                            }
                            _ => {
                                has_opponent_between = true;
                            }
                        }
                        cur_x += dx;
                        cur_y += dy;
                    }
                }
            }
        }
    }

    valid_moves
}

// 勝敗を判定する関数
fn determine_winner(board_data: &[i8]) -> i8 {
    // プレイヤーとCPUの駒の数をカウント
    let (player_count, cpu_count) = board_data.iter().fold((0, 0), |(player, cpu), &stone| {
        match stone {
            0 => (player + 1, cpu),
            1 => (player, cpu + 1),
            _ => (player, cpu),
        }
    });

    // 勝者を決定
    if player_count > cpu_count {
        0 // プレイヤーの勝利
    } else if cpu_count > player_count {
        1 // CPUの勝利
    } else {
        -1 // 引き分け
    }
}
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
fn check_player_input(size:i32,input: &str) -> Result<(), String> {
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
    let y = input[1].parse::<i32>()?;
    Ok((x,y))
}
// 指定インデックス-1のタプルを返すだけ
fn offset_pos(x:i32,y:i32)->(i32,i32){
    (x-1,y-1)
}
/*
// 駒をひっくり返す
fn flip_stone(size: i32, board_data: &mut [i8], x: i32, y: i32, color: i8){
    
}
*/ 



// 駒をひっくり返す
fn flip_stone(size: i32, board_data: &mut [i8], x: i32, y: i32, color: i8) {
    let directions = [
        (1, 0),  // 右
        (0, 1),  // 下
        (-1, 0), // 左
        (0, -1), // 上
        (1, 1),  // 右下
        (1, -1), // 右上
        (-1, 1), // 左下
        (-1, -1) // 左上
    ];

    for (dx, dy) in directions.iter() {
        let mut flip_positions:Vec<(i32,i32)> = Vec::new();
        let mut cur_x = x + dx;
        let mut cur_y = y + dy;

        while cur_x >= 0 && cur_x < size && cur_y >= 0 && cur_y < size {
            let index = (cur_x * size + cur_y) as usize;
            match board_data[index] {
                2 => break, // 空きマスにぶつかったら終了
                _ if board_data[index] == color => {
                    // 同じ色の駒に出会ったら、リストに追加してひっくり返す
                    for pos in flip_positions {
                        let flip_index = (pos.0 * size + pos.1) as usize;
                        board_data[flip_index] = color;
                    }
                    break;
                }
                _ => flip_positions.push((cur_x, cur_y)) // 違う色の駒をリストに追加
            }
            cur_x += dx;
            cur_y += dy;
        }
    }
}
// 駒を置く
fn place_stone(size: i32, board_data: &mut [i8], x: i32, y: i32, color: i8) -> bool {
    // 譜面サイズに収まっているなら
    if x >= 0 && y >= 0 && x < size && y < size {
        // 二次元座標を一次元配列のインデックスに変換
        let index = x * size + y;
        // もしその譜面が空(2)なら
        if board_data[index as usize] == 2 {
            // 駒を置く
            board_data[index as usize] = color;
            // ひっくり返せる場合はひっくり返す
            flip_stone(size,&mut board_data.as_mut(),x,y,color);
            return true;
        }
    }
    return false;
}
// ランダムにxy座標を生成
fn generate_random_pos() -> (i32, i32) {
    let mut rng = rand::thread_rng(); // スレッドローカルの乱数生成器を作成
    let x: i32 = rng.gen_range(0..8); // 0から99の範囲で乱数を生成
    let y: i32 = rng.gen_range(0..8); // 0から99の範囲で乱数を生成
    (x, y) // タプルを返す
}
fn init_message(){
    println!("{}","おせろー".white().on_blue());
    println!("{}","やり方は--helpで確認してくださいー".white().on_blue());
}
// 盤面の描画
fn draw_board(board_data: &[i8], size: usize, kuro: &str, shiro: &str) {
    // 譜面枠の描画(上)
    for _ in 0..5 {
        print!("{}","+--".white().on_green());
    }
    print!("{}","+".white().on_green());

    // 譜面データの描画
    for (i, &data) in board_data.iter().enumerate() {
        if i % size == 0 {
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
}
/**おせろのコード**/
/**基本的にプレイヤーが先行***/

fn main() -> Res<()> {
    // 譜面サイズ
    const BOARD_SIZE: usize = 16;
    let kuro = "◯ ";
    let shiro = "● ";
    let mut input_color = String::new();    // プレイヤーの色 
    let mut winner = -1; // 勝敗(プレイヤー: 0 CPU: 1)
                                   // 初期譜面データ
                                   // 白譜面: 0 黒譜面: 1 空譜面: 2
    let mut board_data:Vec<i8> = init_board(BOARD_SIZE);
    init_message();
    println!("board size: {}x{}", BOARD_SIZE, BOARD_SIZE);
    println!("おせろの色を選択してくださーい(白:0 黒:1)");
    io::stdin().read_line(&mut input_color)?;
    

    loop {
        // 盤面の描画
        draw_board(&board_data, BOARD_SIZE, kuro, shiro);
        // プレイヤー
        println!("プレイヤーの番です");
        let mut input = String::new(); // プレイヤーの入力
        // プレイヤーの入力を受け取る
        io::stdin().read_line(&mut input)?;
        let input = input.trim().clone();
        // 入力値のチェック
        check_player_input(BOARD_SIZE as i32,&input).unwrap();
        // 入力値を数値に変換
        let (x,y) = get_input_pos(&input)?;
        let (x, y) = offset_pos(x, y); // 1-based から 0-based に変換
        let color = input_color.trim().parse::<i8>().unwrap();
        //println!("x: {} y: {} input: {}",x,y,input);
        place_stone(BOARD_SIZE as i32,&mut board_data.as_mut_slice(), x,y,color);
        // 盤面の描画
        draw_board(&board_data, BOARD_SIZE, kuro, shiro);
        // CPU
        println!("CPUの番です");
        sleep(Duration::new(2, 0)); // 2秒待機

       if let Some((cx, cy)) = choose_best_move(BOARD_SIZE as i32, &board_data, 1 - color) {
            check_range_input(BOARD_SIZE as i32,cx,cy);    
           place_stone(BOARD_SIZE as i32, &mut board_data.as_mut_slice(), cx, cy, 1 - color);
       }
        // 勝敗の判定
        winner = determine_winner(&board_data);
        
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
    }else {
        println!("引き分け!");
    }
    Ok(())
}
