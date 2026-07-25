use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use std::env;

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(WELCOME_HTML)
}

async fn health() -> impl Responder {
    HttpResponse::Ok().json(json!({ "status": "ok" }))
}

async fn api_info() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "name": "combinerail9",
        "version": env!("CARGO_PKG_VERSION"),
        "runtime": "Rust / Actix-Web",
        "cloud": "AWS",
        "region": "us-east-1",
        "target": "EC2"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);
    log::info!("combinerail9 listening on 0.0.0.0:{}", port);
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/health", web::get().to(health))
            .route("/api/info", web::get().to(api_info))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

const WELCOME_HTML: &str = r##"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>combinerail9 — Deployed by UDAP</title>
  <style>
    *, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }

    body {
      min-height: 100vh;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      background: #0d1117;
      color: #e6edf3;
      font-family: 'Segoe UI', system-ui, -apple-system, sans-serif;
    }

    .card {
      background: #161b22;
      border: 1px solid #30363d;
      border-radius: 16px;
      padding: 48px 56px;
      max-width: 560px;
      width: 92%;
      text-align: center;
      box-shadow: 0 8px 40px rgba(0,0,0,.5);
    }

    .badge {
      display: inline-block;
      background: #238636;
      color: #fff;
      font-size: 12px;
      font-weight: 600;
      letter-spacing: .6px;
      text-transform: uppercase;
      padding: 4px 14px;
      border-radius: 20px;
      margin-bottom: 28px;
    }

    h1 {
      font-size: 2.4rem;
      font-weight: 700;
      color: #58a6ff;
      letter-spacing: -0.5px;
      margin-bottom: 12px;
    }

    .subtitle {
      color: #8b949e;
      font-size: 1rem;
      line-height: 1.6;
      margin-bottom: 36px;
    }

    .meta {
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 14px;
      margin-bottom: 36px;
    }

    .meta-item {
      background: #0d1117;
      border: 1px solid #21262d;
      border-radius: 10px;
      padding: 14px 16px;
    }

    .meta-item .label {
      font-size: 11px;
      color: #8b949e;
      text-transform: uppercase;
      letter-spacing: .6px;
      margin-bottom: 4px;
    }

    .meta-item .value {
      font-size: 14px;
      font-weight: 600;
      color: #e6edf3;
    }

    .status-dot {
      display: inline-block;
      width: 8px;
      height: 8px;
      background: #3fb950;
      border-radius: 50%;
      margin-right: 6px;
      animation: pulse 2s ease-in-out infinite;
    }

    @keyframes pulse {
      0%, 100% { opacity: 1; }
      50%       { opacity: .4; }
    }

    .health {
      display: flex;
      align-items: center;
      justify-content: center;
      gap: 6px;
      font-size: 14px;
      color: #3fb950;
      font-weight: 600;
      margin-bottom: 32px;
    }

    .links {
      display: flex;
      gap: 12px;
      justify-content: center;
      flex-wrap: wrap;
    }

    .links a {
      display: inline-block;
      padding: 10px 22px;
      border-radius: 8px;
      font-size: 14px;
      font-weight: 600;
      text-decoration: none;
      transition: opacity .15s;
    }

    .links a:hover { opacity: .8; }
    .links .primary   { background: #238636; color: #fff; }
    .links .secondary {
      background: transparent;
      border: 1px solid #30363d;
      color: #58a6ff;
    }

    footer {
      margin-top: 48px;
      color: #484f58;
      font-size: 12px;
    }

    footer a { color: #58a6ff; text-decoration: none; }
  </style>
</head>
<body>
  <div class="card">
    <div class="badge">Deployment Successful</div>

    <h1>combinerail9</h1>

    <p class="subtitle">
      Your Rust application is live and running.<br>
      Deployed automatically by <strong>UDAP</strong>.
    </p>

    <div class="health">
      <span class="status-dot"></span>
      Service is online
    </div>

    <div class="meta">
      <div class="meta-item">
        <div class="label">Cloud</div>
        <div class="value">AWS</div>
      </div>
      <div class="meta-item">
        <div class="label">Target</div>
        <div class="value">EC2</div>
      </div>
      <div class="meta-item">
        <div class="label">Region</div>
        <div class="value">us-east-1</div>
      </div>
      <div class="meta-item">
        <div class="label">Runtime</div>
        <div class="value">Rust / Actix-Web</div>
      </div>
    </div>

    <div class="links">
      <a class="primary" href="/health">Health Check</a>
      <a class="secondary" href="/api/info">API Info</a>
    </div>
  </div>

  <footer>
    Powered by <a href="https://udap.app" target="_blank" rel="noopener">UDAP</a>
  </footer>
</body>
</html>
"##;
