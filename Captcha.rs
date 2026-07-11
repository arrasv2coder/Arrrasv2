use axum::{
    routing::get,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    response::Html,
    response::Response,
    extract::State,
    Json,
    Router,
};
use rand::distr::{Alphanumeric, SampleString};
use std::path::Path;
use tokio::{fs,sync::oneshot,sync::Mutex};
use image::{ImageReader,GenericImageView,Rgba,ImageFormat};
use imageproc::{drawing::draw_text};
use ab_glyph::{FontVec, PxScale};
use std::io::Cursor;
use std::sync::Arc;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
#[derive(Clone)]
struct AppState {
    number: Arc<Mutex<i32>>,
    ip: Arc<Mutex<Vec<i32>>>,
}
#[tokio::main]
async fn main() {
    let shared_state = AppState {
        number : Arc::new(Mutex::new(0)),
        ip : Arc::new(Mutex::new(vec![0])),
    };
    let app = Router::new().route("/image",axum::routing::get(giveimage)).with_state(shared_state.clone()).route("/",axum::routing::get(homepage));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    let image_path = "images/background.png";
    let font_vec = Vec::from(include_bytes!("../font.ttf") as &[u8]);
    let font = FontVec::try_from_vec(font_vec).expect("cannot open font");
    let scale = PxScale::from(200.0);
    tokio::spawn(async move {
        loop {
            let mut number=shared_state.number.lock().await;
            if *number<101{
                let randomtext = Alphanumeric.sample_string(&mut rand::rng(), 16);
                let image = match ImageReader::open(image_path).expect("cannot open image").decode() {
                    Ok(img) => img, 
                    Err(e) => {
                        println!("Error loading image: {}", e);
                        return; 
                    }
                };
                let (dimension_x, dimension_y) = image.dimensions();
                let new_image = draw_text(&image,
                    Rgba([255, 255, 200, 255]),
                    (dimension_x / 100).try_into().expect("x dimension error"),
                    (dimension_y / 8).try_into().expect("y dimension error"),
                    scale,
                    &font,
                    &randomtext
                );
                let filename = format!("images/captcha{}.png",number);
                *number+=1;
                new_image.save(filename).expect("failed to save");
            }
        }
    });
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
async fn giveimage(State(state): State<AppState>,headers: HeaderMap)-> Response{
    let mut number=state.number.lock().await;
    *number-=1;
    let filename=format!("images/captcha{}.png",number);
    let path = Path::new(&filename);
    let real_ip = headers
        .get("X-Real-IP")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("0.0.0.0");
    let user_agent = headers
        .get("user-agent") // Note: HTTP headers are case-insensitive, Axum uses lowercase keys
        .and_then(|value| value.to_str().ok())
        .unwrap_or("Missing");
    match fs::read(path).await {
        Ok(bytes) => {
            if let Err(e) = tokio::fs::remove_file(&filename).await {
                eprintln!("Failed to delete file after serving: {}", e);
            }
            let headers = [
                (header::CONTENT_TYPE, "image/png"),
                (header::CACHE_CONTROL, "no-store, no-cache, must-revalidate"),
            ];

            (StatusCode::OK, headers, bytes).into_response()
        }
        Err(_) => {
            (StatusCode::NOT_FOUND, "Image not found").into_response()
        }
    }
}
async fn homepage() -> Html<&'static str>{
    Html("<style>.responsive {width: 100%;height: auto;}</style><img src='/image' alt='captcha' class='responsive'><form action='/verify'><input type='text'></form>")
}

