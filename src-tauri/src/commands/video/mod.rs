/**
 * Copyright 2025 The VOID Authors. All Rights Reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_util::codec::{BytesCodec, FramedRead};
use warp::Filter;

pub struct VideoState {
    pub video_path: Mutex<Option<PathBuf>>,
}

impl VideoState {
    pub fn new() -> Self {
        Self {
            video_path: Mutex::new(None),
        }
    }
}

pub fn init_video_server() -> Arc<VideoState> {
    let video_state = Arc::new(VideoState::new());
    start_video_server(Arc::clone(&video_state));
    video_state
}

#[tauri::command]
pub async fn check_file_exists(path: String) -> Result<bool, String> {
    let path = PathBuf::from(path);
    Ok(path.exists())
}

#[tauri::command]
pub async fn get_video_url(path: String) -> Result<String, String> {
    let absolute_path = std::fs::canonicalize(&path)
        .map_err(|e| format!("Failed to get absolute path: {}", e))?
        .to_string_lossy()
        .to_string()
        .replace("\\\\?\\", "");

    Ok(format!(
        "http://localhost:1421/video?path={}",
        urlencoding::encode(&absolute_path)
    ))
}

#[tauri::command]
pub async fn set_video_path(
    path: String,
    state: tauri::State<'_, Arc<VideoState>>,
) -> Result<(), String> {
    let mut video_path = state.video_path.lock().await;
    *video_path = Some(PathBuf::from(path));
    Ok(())
}

fn start_video_server(state: Arc<VideoState>) {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            use std::fs::File;
            use std::io::{Read, Seek, SeekFrom};
            use warp::http::header::{CONTENT_LENGTH, CONTENT_RANGE, RANGE};
            use warp::hyper::Body;
            use warp::hyper::StatusCode;

            let video_route = warp::path("video")
                .and(warp::get())
                .and(warp::query::<std::collections::HashMap<String, String>>())
                .and(warp::header::headers_cloned())
                .and_then(
                    move |query: std::collections::HashMap<String, String>,
                          headers: warp::http::HeaderMap| {
                        let state = Arc::clone(&state);
                        async move {
                            let path = if let Some(path_str) = query.get("path") {
                                let decoded_path = urlencoding::decode(path_str)
                                    .map_err(|_| warp::reject::not_found())?;
                                PathBuf::from(decoded_path.as_ref())
                            } else {
                                let path = state.video_path.lock().await.clone();
                                path.ok_or(warp::reject::not_found())?
                            };
                            if !path.exists() {
                                return Err(warp::reject::not_found());
                            }
                            let mime_type = if let Some(ext) = path.extension() {
                                match ext.to_str().unwrap_or("").to_lowercase().as_str() {
                                    "mp4" => "video/mp4",
                                    "mov" => "video/quicktime",
                                    "avi" => "video/x-msvideo",
                                    _ => "video/mp4",
                                }
                            } else {
                                "video/mp4"
                            };
                            let file_size = std::fs::metadata(&path)
                                .map_err(|_| warp::reject::not_found())?
                                .len();
                            if let Some(range_header) = headers.get(RANGE) {
                                let range_str = range_header.to_str().unwrap_or("");
                                if let Some(range) = range_str.strip_prefix("bytes=") {
                                    let mut parts = range.split('-');
                                    let start =
                                        parts.next().unwrap_or("").parse::<u64>().unwrap_or(0);
                                    let end = parts
                                        .next()
                                        .and_then(|s| s.parse::<u64>().ok())
                                        .unwrap_or(file_size - 1);
                                    let chunk_size = end - start + 1;
                                    let mut file =
                                        File::open(&path).map_err(|_| warp::reject::not_found())?;
                                    file.seek(SeekFrom::Start(start))
                                        .map_err(|_| warp::reject::not_found())?;
                                    let mut buffer = vec![0; chunk_size as usize];
                                    file.read_exact(&mut buffer)
                                        .map_err(|_| warp::reject::not_found())?;
                                    let mut response = warp::http::Response::builder()
                                        .status(StatusCode::PARTIAL_CONTENT)
                                        .header("Content-Type", mime_type)
                                        .header("Accept-Ranges", "bytes")
                                        .header(CONTENT_LENGTH, buffer.len().to_string())
                                        .header(
                                            CONTENT_RANGE,
                                            format!("bytes {}-{}/{}", start, end, file_size),
                                        )
                                        .body(Body::from(buffer))
                                        .unwrap();
                                    response.headers_mut().insert(
                                        "Access-Control-Allow-Origin",
                                        "*".parse().unwrap(),
                                    );
                                    response.headers_mut().insert(
                                        "Access-Control-Allow-Methods",
                                        "GET, OPTIONS".parse().unwrap(),
                                    );
                                    response.headers_mut().insert(
                                        "Access-Control-Allow-Headers",
                                        "Content-Type".parse().unwrap(),
                                    );
                                    return Ok(response);
                                }
                            }
                            let file = tokio::fs::File::open(&path)
                                .await
                                .map_err(|_| warp::reject::not_found())?;
                            let stream = FramedRead::new(file, BytesCodec::new());
                            let body = Body::wrap_stream(stream);
                            let mut response = warp::hyper::Response::new(body);
                            response
                                .headers_mut()
                                .insert("Content-Type", mime_type.parse().unwrap());
                            response
                                .headers_mut()
                                .insert("Accept-Ranges", "bytes".parse().unwrap());
                            response
                                .headers_mut()
                                .insert("Access-Control-Allow-Origin", "*".parse().unwrap());
                            response.headers_mut().insert(
                                "Access-Control-Allow-Methods",
                                "GET, OPTIONS".parse().unwrap(),
                            );
                            response.headers_mut().insert(
                                "Access-Control-Allow-Headers",
                                "Content-Type".parse().unwrap(),
                            );
                            Ok(response)
                        }
                    },
                );
            let routes = video_route;
            let addr: SocketAddr = "127.0.0.1:1421".parse().unwrap();
            println!("Video server starting on {}", addr);
            warp::serve(routes).run(addr).await;
        });
    });
}
