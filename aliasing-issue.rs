use anyhow::Result;
use compositor_api::types::Resolution;
use serde_json::json;
use std::time::Duration;

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
        "image/example_png/register",
        &json!({
            "asset_type": "png",
            "url": "https://i.imgur.com/mEDDq9z.png",
        }),
    )?;

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
                    "root": {
                        "type": "view",
                        "background_color_rgba": "#000000FF",
                        "children": [
                            {
                                "type": "view",
                                "bottom": 300,
                                "left": 425,
                                "width": 500,
                                "height": 500,
                                "children": [
                                    {
                                        "type": "rescaler",
                                        "mode": "fit",
                                        "child": {
                                            "type": "image",
                                            "image_id": "example_png",
                                        }
                                    }
                                ]
                            },
                            {
                                "type": "view",
                                "bottom": 425,
                                "left": 950,
                                "width": 250,
                                "height": 250,
                                "children": [
                                    {
                                        "type": "rescaler",
                                        "mode": "fit",
                                        "child": {
                                            "type": "image",
                                            "image_id": "example_png",
                                        }
                                    }
                                ]
                            },
                            {
                                "type": "view",
                                "bottom": 485,
                                "left": 1225,
                                "width": 125,
                                "height": 125,
                                "children": [
                                    {
                                        "type": "rescaler",
                                        "mode": "fit",
                                        "child": {
                                            "type": "image",
                                            "image_id": "example_png",
                                        }
                                    }
                                ]
                            },
                            {
                                "type": "view",
                                "bottom": 525,
                                "left": 1375,
                                "width": 50,
                                "height": 50,
                                "children": [
                                    {
                                        "type": "rescaler",
                                        "mode": "fit",
                                        "child": {
                                            "type": "image",
                                            "image_id": "example_png",
                                        }
                                    }
                                ]
                            },
                        ]
                    }
                }
            }
        }),
    )?;

    std::thread::sleep(Duration::from_millis(500));

    examples::post("start", &json!({}))?;
    Ok(())
}
