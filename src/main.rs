use axum::http::Method;
use axum::{
    extract::Path,
    http::{header, HeaderMap, StatusCode},
    routing::get,
    Router,
};
use std::fs;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

const PORT: u16 = 80;
const HOST: &str = "0.0.0.0";

const USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0";

const CACHE_DIR: &str = "cache";
const SERVERS: [&str; 9] = [
    "https://www.google.com/maps/vt/lyrs=s&x={x}&y={y}&z={z}",
    "https://mt0.google.com/vt/lyrs=s&x={x}&y={y}&z={z}",
    "https://mt1.google.com/vt/lyrs=s&x={x}&y={y}&z={z}",
    "https://mt2.google.com/vt/lyrs=s&x={x}&y={y}&z={z}",
    "https://mt3.google.com/vt/lyrs=s&x={x}&y={y}&z={z}",
    "https://khms0.google.com/kh/v=968?x={x}&y={y}&z={z}",
    "https://khms1.google.com/kh/v=968?x={x}&y={y}&z={z}",
    "https://khms2.google.com/kh/v=968?x={x}&y={y}&z={z}",
    "https://khms3.google.com/kh/v=968?x={x}&y={y}&z={z}",
];

#[tokio::main]
async fn main() {
    // create caceh dir if it doesnt exist
    fs::create_dir_all(CACHE_DIR).unwrap();

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET])
        // allow requests from any origin
        .allow_origin(Any);

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/map/:z/:x/:y", get(get_tile))
        // Create state
        .with_state(())
        // Create middleware
        .layer(ServiceBuilder::new().layer(cors));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", HOST, PORT))
        .await
        .unwrap();

    // run the server
    println!("Listening on port {}", PORT);
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "GeoFS Cache Server"
}

// get a tile and cache it if it is not alread, else send the image
async fn get_tile(Path((z, x, y)): Path<(u8, u32, u32)>) -> impl axum::response::IntoResponse {
    // create image
    let img: Vec<u8>;

    // check if cache exists
    let cache_path = format!("{}/{}/{}/{}.jpg", CACHE_DIR, z, x, y);
    if tokio::fs::metadata(&cache_path).await.is_ok() {
        println!("Cache hit: {}", cache_path);
        img = tokio::fs::read(cache_path).await.unwrap();
    } else {
        println!("Cache miss: {}", cache_path);

        let client = reqwest::Client::new();
        let mut attempt: u8 = 0;
        loop {
            let server: String = SERVERS[(std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_micros() as u64
                % SERVERS.len() as u64) as usize]
                .to_string();
            
            let url = server
                .replace("{x}", &x.to_string())
                .replace("{y}", &y.to_string())
                .replace("{z}", &z.to_string());

            let res = client
                .get(&url)
                .header("User-Agent", USER_AGENT)
                .send()
                .await;

            match res {
                Ok(response) => {
                    if response.status() == 200 {
                        let bytes = response.bytes().await.unwrap();
                        img = bytes.to_vec();
                        break;
                    } else {
                        attempt += 1;
                        if attempt >= 3 {
                            let mut headers = HeaderMap::new();
                            headers.insert(header::CONTENT_TYPE, "text/plain".parse().unwrap());
                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                headers,
                                "Internal Server Error".as_bytes().to_vec(),
                            );
                        }
                    }
                }
                Err(_) => {
                    attempt += 1;
                    if attempt >= 3 {
                        let mut headers = HeaderMap::new();
                        headers.insert(header::CONTENT_TYPE, "text/plain".parse().unwrap());
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            headers,
                            "Internal Server Error".as_bytes().to_vec(),
                        );
                    }
                }
            }
        }

        // save to cache async (dont wait for it to finish)
        let img = img.clone();
        tokio::spawn(async move {
            let dir = format!("{}/{}/{}", CACHE_DIR, z, x);
            if tokio::fs::metadata(&dir).await.is_err() {
                tokio::fs::create_dir_all(&dir).await.unwrap();
            }

            let path = format!("{}/{}.jpg", dir, y);
            tokio::fs::write(path, img).await.unwrap();
        });
    }

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "image/jpeg".parse().unwrap());
    headers.insert(header::CACHE_CONTROL, "max-age=31536000".parse().unwrap());

    (StatusCode::OK, headers, img)
}
