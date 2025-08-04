// use image::{ImageBuffer, Rgba};
// use screenshot::window::capture_frame_gdi;
// use std::path::Path;
// use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // //打印当前process id
    // println!("当前进程ID: {}", std::process::id());
    


    // let main_start = Instant::now();
    // println!("开始截图...");

    // // 捕获屏幕
    // if let Ok(frame_data) = capture_frame_gdi(None) {
    //     let (pixel_data, width, height) =
    //         (frame_data.pixel_data, frame_data.width, frame_data.height);
    //     println!(
    //         "成功捕获屏幕: {}x{},耗时: {:.2}ms",
    //         width,
    //         height,
    //         main_start.elapsed().as_secs_f64() * 1000.0
    //     );

    //     // 将像素数据转换为 RGBA ImageBuffer
    //     // 注意: Windows GDI 返回的是 BGRA 格式，需要转换为 RGBA
    //     let conversion_start = Instant::now();
    //     let mut rgba_data = Vec::with_capacity(pixel_data.len());

    //     // 将 BGRA 转换为 RGBA
    //     for chunk in pixel_data.chunks_exact(4) {
    //         rgba_data.push(chunk[2]); // R
    //         rgba_data.push(chunk[1]); // G
    //         rgba_data.push(chunk[0]); // B
    //         rgba_data.push(chunk[3]); // A
    //     }
    //     println!(
    //         "⏱️ BGRA→RGBA转换耗时: {:.2}ms",
    //         conversion_start.elapsed().as_secs_f64() * 1000.0
    //     );

    //     // 创建 ImageBuffer
    //     let image_creation_start = Instant::now();
    //     let img =
    //         ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width as u32, height as u32, rgba_data)
    //             .ok_or("无法创建图像缓冲区")?;
    //     println!(
    //         "⏱️ 创建ImageBuffer耗时: {:.2}ms",
    //         image_creation_start.elapsed().as_secs_f64() * 1000.0
    //     );

    //     // 保存为 PNG 文件
    //     let save_start = Instant::now();
    //     let output_path = "screenshot.png";
    //     img.save(output_path)?;
    //     println!(
    //         "⏱️ 保存PNG文件耗时: {:.2}ms",
    //         save_start.elapsed().as_secs_f64() * 1000.0
    //     );

    //     let total_time = main_start.elapsed().as_secs_f64() * 1000.0;
    //     println!(
    //         "截图已保存到: {}",
    //         Path::new(output_path).canonicalize()?.display()
    //     );
    //     println!("🏆 程序总耗时: {:.2}ms", total_time);
    // } else {
    //     eprintln!("截图失败");
    //     return Err("截图操作失败".into());
    // }

    Ok(())
}
