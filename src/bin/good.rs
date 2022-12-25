use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

fn main() {
    let mp = MultiProgress::new();
    let pb = ProgressBar::new(0);
    let style = ProgressStyle::with_template("{msg}").unwrap();
    pb.set_style(style);
    mp.add(pb.clone());

    let pb2 = ProgressBar::new(100);
    let style2 = ProgressStyle::with_template("{bar}").unwrap();
    pb2.set_style(style2);
    mp.add(pb2.clone());
    pb2.set_position(0);
    pb2.finish_and_clear();

    pb.set_message("Good!");
    pb.finish();
}
