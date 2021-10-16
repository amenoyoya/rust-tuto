// macroquad::prelude クレートをグローバルスコープに導入
use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        // 背景色を薄灰色に設定
        clear_background(LIGHTGRAY);
      
        // 青い棒を描画
        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
      
        // 緑の四角を描画
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
      
        // 黄色い丸を描画
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
      
        // 濃灰色で文字を描画
        draw_text("HELLO", 20.0, 20.0, 30.0, DARKGRAY);
      
        // 次のフレーム描画を待機
        next_frame().await
    }
}