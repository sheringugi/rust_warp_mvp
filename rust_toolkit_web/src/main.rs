use serde::Deserialize;
use warp::Filter;

#[derive(Deserialize)]
struct GreetQuery {
    name: Option<String>,
}

#[tokio::main]
async fn main() {
    let index = warp::path::end()
        .and(warp::query::<GreetQuery>())
        .map(|query: GreetQuery| {
            let greeting = match query.name {
                Some(name) => format!("Hello, {},", name),
                None => "".to_string(),
            };

            let glow_css = if greeting.trim() != "" {
                "color: #f8f3f3; text-shadow: 0 0 8px #60a5fa, 0 0 16px #60a5fa, 0 0 24px #60a5fa;"
            } else {
                ""
            };

            let html_body = format!(r#"
<!DOCTYPE html>
<html>
<head>
    <title>Rust Toolkit</title>
    <style>
        body {{
            font-family: 'Segoe UI', sans-serif;
            background-color: #04153b;
            color: #f8f3f3;
            text-align: center;
            padding: 3rem;
        }}

        h1 {{
            font-size: 2.5rem;
            margin-bottom: 1rem;
            {glow_css}
        }}

        p {{
            font-size: 1.2rem;
            margin-bottom: 1.5rem;
        }}

        button {{
            background-color: #ffffff;
            color: #04153b;
            border: none;
            border-radius: 8px;
            padding: 0.7rem 1.5rem;
            font-size: 1.1rem;
            cursor: pointer;
            transition: background-color 0.3s ease, color 0.3s ease, box-shadow 0.3s ease;
        }}

        button:hover {{
            background-color: #60a5fa;
            color: #04153b;
            box-shadow: 0 0 12px #60a5fa;
        }}
    </style>
</head>
<body>
    <h1>{greeting} Welcome to Rust Toolkit Web</h1>
    <p>This is your MVP frontend served directly from Warp.</p>
    <button onclick="showMessage(); showSparks()">Click me</button>
    <p id="msg"></p>

    <!-- Confetti JS -->
    <script src="https://cdn.jsdelivr.net/npm/canvas-confetti@1.6.0/dist/confetti.browser.min.js"></script>
    <script>
        async function showMessage() {{
            const response = await fetch('/hello');
            const data = await response.json();
            document.getElementById('msg').innerText = data.message;
        }}

        // Spark effect on button click
        function showSparks() {{
            confetti({{
                particleCount: 50,
                spread: 60,
                startVelocity: 30,
                decay: 0.9,
                origin: {{ y: 0.5 }}
            }});
        }}

        // Trigger confetti if a name is provided
        const greetingText = "{greeting}";
        if (greetingText.trim() !== "") {{
            confetti({{
                particleCount: 150,
                spread: 70,
                origin: {{ y: 0.6 }}
            }});
        }}
    </script>
</body>
</html>
"#, greeting = greeting, glow_css = glow_css);

            warp::reply::html(html_body)
        });

    let hello = warp::path("hello")
        .and(warp::path::end())
        .map(|| warp::reply::json(&serde_json::json!({"message": "Hello from Rust warp backend!"})));

    let routes = index.or(hello);

    let addr = ([127, 0, 0, 1], 3000);
    println!("Server running at http://127.0.0.1:{}", addr.1);

    warp::serve(routes).run(addr).await;
}
