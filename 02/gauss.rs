fn main() {
    // 入力促進文 //
    println!("ガウスの消去法による連立方程式の解法");
    println!("n次元の配列（ただし 1 < n < 9 ）を求めます。");
    println!("[a11 a12 ... a1n][x1] = [b1]");
    println!("[a21 a22 ... a2n][x2] = [b2]");
    println!("[... ... ... a3n][..] = [..]");
    println!("[an1 an2 ... a4n][xn] = [bn]");
    println!();
    println!("ただし、次のように入力してください。");
    println!("n");
    println!("a11 a12 ... a1n b1");
    println!("a21 a22 ... a2n b2");
    println!("... ... ... ... ..");
    println!("an1 an2 ... ann bn");
    println!();

    // 入力 //
    let n: usize = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim().parse().unwrap()
    };
    let mut a: Vec<Vec<f64>> = {
        let mut vec = Vec::new();
        for _ in 0..n {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            vec.push(s.split_whitespace().map(|e| e.parse().unwrap()).collect());
        }
        vec
    };

    // 入力した内容を表示 //
    println!("n   = {}", n);
    println!("a   = {:?} ", a);

    // 計算 //
    for i in 0..n {
        swap_lines(&mut a, i);
        let p = a[i][i];
        if p.abs() < 1.0e-6 {
            println!("一意解を持ちません。");
            std::process::exit(-1);
        }
        for j in i..(n + 1) {
            a[i][j] = a[i][j] / p;
        }
        for k in i + 1..n {
            let q = a[k][i];
            for j in i..n + 1 {
                a[k][j] = a[k][j] - a[i][j] * q;
            }
        }
    }

    // 解の配列を生成 //
    let ans: Vec<f64> = {
        let mut x: Vec<f64> = vec![0.0; n];
        for i in (0..n).rev() {
            let mut s = 0.0;
            for j in i + 1..n {
                s += a[i][j] * x[j];
            }
            x[i] = a[i][a.len()] - s;
        }
        x
    };

    // 解の表示 //
    println!("ans = {:?}", ans);
}

fn swap_lines(matrix: &mut Vec<Vec<f64>>, column_to_compare: usize) {
    let mut max_line = column_to_compare;
    for i in column_to_compare..matrix.len() {
        if matrix[max_line][column_to_compare].abs() < matrix[i][column_to_compare] {
            max_line = i;
        }
    }
    for i in 0..matrix.len() + 1 {
        let temp = matrix[max_line][i];
        matrix[max_line][i] = matrix[column_to_compare][i];
        matrix[column_to_compare][i] = temp;
    }
}
