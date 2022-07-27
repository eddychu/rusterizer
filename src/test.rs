// let mut ppm_buffer = vec![Vec3::new(0.0, 0.0, 0.0); WIDTH * HEIGHT];
//     render(&mut ppm_buffer);
//     let mut file = fs::OpenOptions::new()
//         .write(true)
//         .append(true)
//         .create(true)
//         .open("file.ppm")
//         .unwrap();
//     file.write(format!("P3\n{} {}\n255\n", WIDTH, HEIGHT).as_bytes())
//         .expect("FAILED");
//     for j in 0..HEIGHT {
//         for i in 0..WIDTH {
//             let index = j * WIDTH + i;
//             let color = ppm_buffer[index];
//             let (r, g, b) = (
//                 (color.x * 255.9) as u32,
//                 (color.y * 255.9) as u32,
//                 (color.z * 255.9) as u32,
//             );
//             file.write(format!("{} {} {}\n", r, g, b).as_bytes())
//                 .expect("something wrong");
//         }
//     }
