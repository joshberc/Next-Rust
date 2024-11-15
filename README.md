# Next-Rust
Simple monorepo app built with **Next** & **Rust**.


### Installing and running this solution
In order to run, you must have the latest version of VS Code, Node >= 14.0.0, npm >= 5.6 & Rust >= 1.82.0 installed on your machine.

1. Download this solution as a ZIP and Un-zip or clone the repo to a local directory.
2. Navigate to folder **/backend** in your editor of choice.
3. In the terminal run the following to start the server @ http://localhost:8000: 
    - `cargo build`
    - `cargo run`
4. Test the API with the following curl requests:
    - `curl -X POST http://localhost:8000/api/prime-check \
     -H "Content-Type: application/json" \
     -d '{"number": 29}'`
    - `curl -X POST http://127.0.0.1:8000/api/median-primes \
     -H "Content-Type: application/json" \
     -d '{"n": 1000}'`
5. Navigate to **/frontend** and run:
    - `npm i`
    - `npm run dev`