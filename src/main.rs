use clap::Parser;
use serde::Serialize;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

/// コマンドライン引数を定義する構造体
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 入力ファイル（- で標準入力）
    #[arg(required = true)]
    files: Vec<PathBuf>,

    /// 行数のみをカウント
    #[arg(short, long)]
    lines: bool,

    /// 単語数のみをカウント
    #[arg(short, long)]
    words: bool,

    /// 文字数のみをカウント
    #[arg(short, long)]
    chars: bool,

    /// JSON形式で出力
    #[arg(short, long)]
    json: bool,
}

/// ファイルの統計情報を保持する構造体
#[derive(Serialize)]
struct Stats {
    filename: String,  // ファイル名
    lines: usize,      // 行数
    words: usize,      // 単語数
    chars: usize,      // 文字数
}

/// テキストの統計情報をカウントする関数
/// ファイル全体をバッファに読み込んでカウントする
fn count_stats<R: Read>(mut reader: R) -> io::Result<(usize, usize, usize)> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let lines = buf.lines().count();
    let words = buf.split_whitespace().count();
    let chars = buf.chars().count();
    Ok((lines, words, chars))
}

/// ファイルを処理して統計情報を取得する関数
/// 
/// # 引数
/// * `path` - 処理するファイルのパス
/// 
/// # 戻り値
/// * `Stats` - ファイルの統計情報
fn process_file(path: &PathBuf) -> io::Result<Stats> {
    let filename = path.to_string_lossy().to_string();
    // 標準入力の場合はstdinを使用、それ以外はファイルを開く
    let reader: Box<dyn Read> = if path.to_string_lossy() == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(path)?)
    };

    let (lines, words, chars) = count_stats(reader)?;
    Ok(Stats {
        filename,
        lines,
        words,
        chars,
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    // コマンドライン引数を解析
    let args = Args::parse();
    let mut stats = Vec::new();

    // 各ファイルを処理
    for path in &args.files {
        match process_file(path) {
            Ok(stat) => stats.push(stat),
            Err(e) => {
                eprintln!("Error processing {}: {}", path.display(), e);
                // エラーが発生した場合、プログラムを終了
                return Err(Box::new(e));
            }
        }
    }

    // 出力形式に応じて結果を表示
    if args.json {
        // JSON形式で出力
        println!("{}", serde_json::to_string_pretty(&stats)?);
    } else {
        // テーブル形式のヘッダーを表示（全項目を表示する場合のみ）
        if !args.lines && !args.words && !args.chars {
            println!("{:>12} {:>12} {:>12} {:>12}", "FILE", "LINES", "WORDS", "CHARS");
            println!("{:>12} {:>12} {:>12} {:>12}", "----", "-----", "-----", "-----");
        }

        // 各ファイルの統計情報を表示
        for stat in stats {
            if args.lines {
                println!("{}: {} lines", stat.filename, stat.lines);
            } else if args.words {
                println!("{}: {} words", stat.filename, stat.words);
            } else if args.chars {
                println!("{}: {} chars", stat.filename, stat.chars);
            } else {
                println!(
                    "{:>12} {:>12} {:>12} {:>12}",
                    stat.filename, stat.lines, stat.words, stat.chars
                );
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_count_stats() -> io::Result<()> {
        // 一時ファイルを作成
        let mut temp_file = NamedTempFile::new()?;
        temp_file.write_all("Hello, World!\nThis is a test.\n".as_bytes())?;
        
        let file = File::open(temp_file.path())?;
        let (lines, words, chars) = count_stats(file)?;
        
        assert_eq!(lines, 2);
        assert_eq!(words, 6);  // "Hello,", "World!", "This", "is", "a", "test."
        assert_eq!(chars, 30); // 13+1+14+1+1=30 (including all newlines)
        
        Ok(())
    }

    #[test]
    fn test_count_stats_empty() -> io::Result<()> {
        let mut temp_file = NamedTempFile::new()?;
        temp_file.write_all("".as_bytes())?;
        
        let file = File::open(temp_file.path())?;
        let (lines, words, chars) = count_stats(file)?;
        
        assert_eq!(lines, 0);
        assert_eq!(words, 0);
        assert_eq!(chars, 0);
        
        Ok(())
    }

    #[test]
    fn test_count_stats_multiple_lines() -> io::Result<()> {
        let mut temp_file = NamedTempFile::new()?;
        temp_file.write_all("Line 1\nLine 2\nLine 3\n".as_bytes())?;
        
        let file = File::open(temp_file.path())?;
        let (lines, words, chars) = count_stats(file)?;
        
        assert_eq!(lines, 3);
        assert_eq!(words, 6);  // "Line", "1", "Line", "2", "Line", "3"
        assert_eq!(chars, 21); // 6+6+6+3(\n) = 21
        
        Ok(())
    }
}
