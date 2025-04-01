use log2::*;

fn main() {
    let _log2 = log2::open("log.txt")
        .size(1024)
        .rotate(10)
        .compress(true)
        .start();

    let text = r#"On either side the river lie
Long fields of barley and of rye,
That clothe the wold and meet the sky;
And thro' the field the road runs by
To many-tower'd Camelot;
And up and down the people go,
Gazing where the lilies blow
Round an island there below,
The island of Shalott."#;

    for i in 0..15 {
        info!("current test id = {}", i);
        trace!("{}", text);
        debug!("{}", text);
        info!("{}", text);
        error!("{}", text);
        warn!("{}", text);
    }
}
