# Workout app frontend
## Requirements
- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)  

By default, the Workout frontend expects the Workout backend to be running on http://localhost:8000.

## Run the application
As usual, first step is cloning the repo :)
```
git clone https://github.com/eduriol/workout-app.git
cd workout-app/frontend
```
Then build the web into the 'static' directory and run the index with your favorite server (I use miniserve for local testing purposes):
```
wasm-pack build --target web --out-name wasm --out-dir ./static
miniserve ./static --index index.html
```
You can access http://localhost:8080 to see the Workout frontend.
