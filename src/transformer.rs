// 3x3 Image, (2 * 9 + 2)* 3 = 60
//  rgb rgb rgb  rgb rgb rgb  rgb rgb rgb
//  rgb rgb rgb  rgb rgb rgb  rgb rgb rgb
//  rgb rgb rgb  rgb rgb rgb  rgb rgb rgb
//
//  rgb rgb rgb  rgb rgb rgb  rgb rgb rgb
//  rgb rgb rgb  rgb rgb rgb  rgb rgb rgb
//  rgb rgb rgb  rgb rgb (rgb)  rgb rgb rgb
//
//  rgb rgb rgb  rgb rgb rgb  rgb rgb rgb
//  rgb rgb rgb  rgb rgb rgb  rgb rgb rgb
//  rgb rgb rgb  rgb rgb rgb  rgb rgb rgb
//
//  To access block at (block_x, block_y):
//  block_y=1 block_x=1 width 3
//  block_pos = block_y * 3 + block_x
//
//  To access pixel in block (x,y):
//  block_pos + (y * 3 + x)

use jpeg_decoder::PixelFormat;

pub fn down_sample(
    vec: Vec<u8>,
    width: usize,
    height: usize,
    new_height: usize,
    new_width: usize,
    pixel_format: PixelFormat,
) -> Vec<u8> {
    /*
     * Given kxk pixel block
     * Block = [
     * [R₁,G₁,B₁], [R₂,G₂,B₂], ..., [Rₖ,Gₖ,Bₖ],
     * ...
     * ]
     *
     * Compute average
     * R_avg = (R₁ + R₂ + ... + Rₖ) / (k * k)
     * G_avg = ...
     * B_avg = ...
     *
     * P_avg = [R_avg, G_avg, B_avg]
     */

    let block_size_x = width / new_width;
    let block_size_y = height / new_height;
    // assert_eq!(
    //     block_size_x, block_size_y,
    //     "Non-square blocks not supported"
    // );

    let pixel_bytes: usize = pixel_format.pixel_bytes().try_into().unwrap();
    let mut downsampled_vec: Vec<u8> = Vec::with_capacity(new_height * new_width * pixel_bytes);
    for block_y in 0..new_height {
        for block_x in 0..new_width {
            let mut r_sum: usize = 0;
            let mut g_sum: usize = 0;
            let mut b_sum: usize = 0;
            let mut count: usize = 0;
            for y in 0..block_size_y {
                for x in 0..block_size_x {
                    let pixel_x = block_x * block_size_x + x;
                    let pixel_y = block_y * block_size_y + y;
                    let idx = (pixel_y * width + pixel_x) * pixel_bytes;

                    r_sum += vec[idx] as usize;
                    g_sum += vec[idx + 1] as usize;
                    b_sum += vec[idx + 2] as usize;
                    count += 1;
                }
            }
            // compute the average
            downsampled_vec.push((r_sum / count) as u8);
            downsampled_vec.push((g_sum / count) as u8);
            downsampled_vec.push((b_sum / count) as u8);
        }
    }

    downsampled_vec
}

pub fn up_sample(
    vec: Vec<u8>,
    original_height: usize,
    original_width: usize,
    new_height: usize,
    new_width: usize,
    pixel_format: PixelFormat,
) -> Vec<u8> {
    let pixel_bytes: usize = pixel_format.pixel_bytes().try_into().unwrap();
    let mut upsampled_vec: Vec<u8> = vec![0u8; new_width * new_height * pixel_bytes];

    let block_size_x = new_width / original_width;
    let block_size_y = new_height / original_height;
    for y in 0..original_height {
        for x in 0..original_width {
            let idx = (y * original_width + x) * pixel_bytes;
            let r = vec[idx];
            let g = vec[idx + 1];
            let b = vec[idx + 2];

            for dy in 0..block_size_y {
                for dx in 0..block_size_x {
                    let out_x = x * block_size_x + dx;
                    let out_y = y * block_size_y + dy;
                    let out_idx = (out_y * new_width + out_x) * pixel_bytes;

                    upsampled_vec[out_idx] = r;
                    upsampled_vec[out_idx + 1] = g;
                    upsampled_vec[out_idx + 2] = b;
                }
            }
        }
    }

    upsampled_vec
}
