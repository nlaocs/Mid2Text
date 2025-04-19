use clap::{Parser, Subcommand};
use mid_text_converter::instruments::{InstrumentKind, Instruments};
use mid_text_converter::song::mid::mid_to_track;
use mid_text_converter::song::Song;
use mid_text_converter::utils;
use arboard::Clipboard;

#[derive(Parser, Debug)]
#[command(version)]
#[clap(disable_help_flag = true)]
#[clap(disable_help_subcommand = true)]
#[clap(disable_version_flag = true)]
#[clap(arg_required_else_help = true)]
struct Args {
    #[clap(subcommand)]
    mode: Option<Mode>,

    /// 使い方を表示する
    #[arg(long, action = clap::ArgAction::Help)]
    help: Option<bool>,
    /// バージョンを表示する
    #[arg(long, action = clap::ArgAction::Version)]
    version: Option<bool>,
}

#[derive(Debug, Subcommand)]
enum Mode {
    #[clap(arg_required_else_help = true)]
    #[clap(about = "midiファイルから曲を作る")]
    #[clap(visible_alias = "c")]
    Create(Box<InstArgs>),
    #[clap(arg_required_else_help = true)]
    #[clap(about = "複数の文字列をマージする")]
    #[clap(visible_alias = "m")]
    Merge {
        /// マージしたい二個以上の文字列
        #[arg(required = true, num_args = 2..)]
        songs: Vec<String>,

        /// マージした文字列をクリップボードにコピーする
        #[arg(short = 'c', long)]
        copy: bool,
    },
}

#[derive(Debug, clap::Args)]
struct InstArgs {
    /// plingに変換するmidファイル
    #[arg(short = 'p', long, num_args = 0..)]
    pling: Vec<String>,
    /// hatに変換するmidファイル
    #[arg(short = 'h', long, num_args = 0..)]
    hat: Vec<String>,
    /// snareに変換するmidファイル
    #[arg(short = 's', long, num_args = 0..)]
    snare: Vec<String>,
    /// bassdrumに変換するmidファイル
    #[arg(short = 'b', long, num_args = 0..)]
    bassdrum: Vec<String>,
    /// bassに変換するmidファイル
    #[arg(long, num_args = 0..)]
    bass: Vec<String>,
    /// bellに変換するmidファイル
    #[arg(long, num_args = 0..)]
    bell: Vec<String>,
    /// chimeに変換するmidファイル
    #[arg(long, num_args = 0..)]
    chime: Vec<String>,
    /// fluteに変換するmidファイル
    #[arg(short = 'f', long, num_args = 0..)]
    flute: Vec<String>,
    /// guitarに変換するmidファイル
    #[arg(short = 'g', long, num_args = 0..)]
    guitar: Vec<String>,
    /// harpに変換するmidiファイル
    #[arg(long, num_args = 0..)]
    harp: Vec<String>,
    /// xylophoneに変換するmidファイル
    #[arg(short = 'x', long, num_args = 0..)]
    xylophone: Vec<String>,

    /// 範囲外の音を範囲内のオクターブへ相対的に移動する
    #[arg(short = 'r', long, )]
    relative: bool,
    
    /// 作成した文字列をクリップボードにコピーする
    #[arg(short = 'c', long)]
    copy: bool,
}

macro_rules! add_instruments {
    ($song:expr, $args:expr, $( ($field:ident, $kind:expr) ),* ) => {
        $(
            if !$args.$field.is_empty() {
                for path in &$args.$field {
                    let track = mid_to_track(path)?;
                    let instrument = Instruments::new($kind, track);
                    $song.add_track(instrument);
                }
            }
        )*
    };
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match &args.mode {
        Some(Mode::Create(create_args)) => {
            let mut song = Song::new();

            add_instruments!(
                song,
                create_args,
                (pling, InstrumentKind::Pling),
                (hat, InstrumentKind::Hat),
                (snare, InstrumentKind::Snare),
                (bassdrum, InstrumentKind::BassDrum),
                (bass, InstrumentKind::Bass),
                (bell, InstrumentKind::Bell),
                (chime, InstrumentKind::Chime),
                (flute, InstrumentKind::Flute),
                (guitar, InstrumentKind::Guitar),
                (harp, InstrumentKind::Harp),
                (xylophone, InstrumentKind::Xylophone)
            );

            let result = song.to_text(create_args.relative);
            
            match result {
                Ok(r) => {
                    if r.is_empty() {
                        println!("Midi file is empty");
                    } else {
                        println!("{}", &r);
                        if create_args.copy {
                            let mut clipboard = Clipboard::new()?;
                            clipboard.set_text(r)?;
                        }
                        
                    }
                }
                Err(any) => {
                    println!("Error: {}", any);
                }
            }

            Ok(())
        }
        Some(Mode::Merge { songs, copy }) => {
            let result = utils::merge_string(songs);
            println!("{}", &result);
            if *copy {
                let mut clipboard = Clipboard::new()?;
                clipboard.set_text(result)?;
            }
            Ok(())
        }
        _ => {
            unreachable!()
        }
    }
}
