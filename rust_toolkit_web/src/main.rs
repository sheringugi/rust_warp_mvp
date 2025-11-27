use warp::Filter;

#[tokio::main]
async fn main() {
    // Serve the index page at "/"
    let index = warp::path::end().map(|| {
        warp::reply::html(r#"
            <!DOCTYPE html>
            <html>
            <head>
                <title>Rust Toolkit</title>
                <style>
                    body { font-family: Arial, sans-serif; margin: 2rem; }
                    h1 { color: #2c3e50; }
                    p { font-size: 1.2rem; }
                    button { padding: 0.5rem 1rem; font-size: 1rem; }
                </style>
            </head>
            <body>
                <h1>Welcome to Rust Toolkit Web</h1>
                <p>This is your MVP frontend served directly from Warp.</p>
                <button onclick="showMessage()">Click me</button>
                <p id="msg"></p>
                <script>
                    async function showMessage() {
                        const response = await fetch('/hello');
                        const data = await response.json();
                        document.getElementById('msg').innerText = data.message;
                    }
                </script>
            </body>
            </html>
        "#)
    });

    // JSON endpoint for button click
    let hello = warp::path("hello").map(|| {
        warp::reply::json(&serde_json::json!({
            "message": "Hello from Rust warp backend!"
        }))
    });

    let routes = index.or(hello);

    let addr = ([127, 0, 0, 1], 3000);
    println!("Server running at http://{}:{}", addr.0[0], addr.1);

    warp::serve(routes).run(addr).await;
}
