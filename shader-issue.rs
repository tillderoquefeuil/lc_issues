use anyhow::Result;
use compositor_api::types::Resolution;
use serde_json::json;
use std::{thread, time::Duration};

use integration_tests::{
    examples::{self, run_example},
    ffmpeg::start_ffmpeg_receive,
};

const VIDEO_RESOLUTION: Resolution = Resolution {
    width: 1920,
    height: 1080,
};

const IP: &str = "127.0.0.1";
const OUTPUT_PORT: u16 = 8004;

fn main() {
    run_example(client_code);
}

fn client_code() -> Result<()> {
    start_ffmpeg_receive(Some(OUTPUT_PORT), None)?;

    examples::post(
        "image/example_jpeg/register",
        &json!({
            "asset_type": "jpeg",
            "url": "https://wallpapers.com/images/hd/grid-1920-x-1080-picture-b2qbbb0cef1l59ah.jpg",
        }),
    )?;


    let shader_source = include_str!("./round.wgsl");
    examples::post(
        "shader/shader_example_1/register",
        &json!({
            "source": shader_source,
        }),
    )?;

    let time_per_scene = 10; // in seconds
    let transitions = json!({
        "duration_ms": 5000,
        "easing_function": {
            "function_name": "cubic_bezier",
            "points": [0.33, 1, 0.68, 1]
        }
    });

    let fullscreen = json!({
        "type": "view",
        "background_color_rgba": "#333333FF",
        "children": [{
            "type": "rescaler",
            "mode": "fill",
            "id": "main_rescaler",
            "transition": transitions,
            "bottom": 0,
            "right": 0,
            "width": 1920,
            "height": 1080,
            "child": {
                "type": "image",
                "image_id": "example_jpeg",
            },
        },{
            "type": "text",
            "text": "FULLSCREEN",
            "font_size": 50.0,
        }]
    });

    let rescaled = json!({
        "type": "view",
        "background_color_rgba": "#333333FF",
        "children": [{
            "type": "rescaler",
            "mode": "fill",
            "id": "main_rescaler",
            "transition": transitions,
            "bottom": 290,
            "right": 710,
            "width": 500,
            "height": 500,
            "child": {
                "type": "shader",
                "resolution": { "width": 500, "height": 500 },
                "shader_id": "shader_example_1",
                "shader_param": {
                    "type": "f32",
                    "value": 0.025
                },
                "children": [{
                    "type": "image",
                    "image_id": "example_jpeg",
                }]
            }
        },{
            "type": "text",
            "text": "RESCALED WITH SHADER",
            "font_size": 50.0,
        }]
    });

    let rescaled_without_shader = json!({
        "type": "view",
        "background_color_rgba": "#333333FF",
        "children": [{
            "type": "rescaler",
            "mode": "fill",
            "id": "main_rescaler",
            "transition": transitions,
            "bottom": 290,
            "right": 710,
            "width": 500,
            "height": 500,
            "child": {
                "type": "image",
                "image_id": "example_jpeg",
            }
        },{
            "type": "text",
            "text": "RESCALED WITHOUT SHADER",
            "font_size": 50.0,
        }]
    });

    examples::post(
        "output/output_1/register",
        &json!({
            "type": "rtp_stream",
            "port": OUTPUT_PORT,
            "ip": IP,
            "video": {
                "resolution": {
                    "width": VIDEO_RESOLUTION.width,
                    "height": VIDEO_RESOLUTION.height,
                },
                "encoder": {
                    "type": "ffmpeg_h264",
                    "preset": "ultrafast"
                },
                "initial": {
                    "root": fullscreen
                }
            }
        }),
    )?;

    thread::sleep(Duration::from_millis(500));

    examples::post("start", &json!({}))?;
    
    thread::sleep(Duration::from_secs(time_per_scene/2));

    loop {

        
        examples::post(
            "output/output_1/update",
            &json!({
                "video": {
                    "root": rescaled_without_shader,
                }
            }),
        )?;
        thread::sleep(Duration::from_secs(time_per_scene));

        examples::post(
            "output/output_1/update",
            &json!({
                "video": {
                    "root": fullscreen
                },
            }),
        )?;
        thread::sleep(Duration::from_secs(time_per_scene));

        examples::post(
            "output/output_1/update",
            &json!({
                "video": {
                    "root": rescaled,
                }
            }),
        )?;
        thread::sleep(Duration::from_secs(time_per_scene));

        examples::post(
            "output/output_1/update",
            &json!({
                "video": {
                    "root": fullscreen,
                }
            }),
        )?;
        thread::sleep(Duration::from_secs(time_per_scene));
    }

    #[allow(unreachable_code)]
    Ok(())
}
