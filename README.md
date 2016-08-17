This is a simple URL shortener developed in the Rust programming language.

## Setup and usage

First, you'll want to install the [Rust toolchain](https://www.rustup.rs/).

    curl https://sh.rustup.rs -sSf | sh

Then, clone this repository.

    git clone git@github.com:fbecart/url-shortener.git
    cd url-shortener

Finally, use Cargo to test, run or build the project locally.

    # Run the tests
    cargo test
    
    # Compile and run the program
    cargo run
    
    # Generate an optimized binary located at target/release/url-shortener
    cargo build --release

## What is actually implemented?

Once launched, the server will generate short URLs.

    curl -i -X POST --data "url=https://www.helloclue.com/" https://localhost:3000

The short URL will be returned as HTTP header `Location`.

    HTTP/1.1 201 Created
    Content-Length: 0
    Location: http://localhost:3000/JqC6S
    Date: Tue, 16 Aug 2016 20:31:35 GMT

Then, the short URL can be queried very simply using a GET or HEAD HTTP request.

    curl -i http://localhost:3000/JqC6S

If the short URL exists, the response will be a redirection to the longer URL.

    HTTP/1.1 302 Found
    Location: https://www.helloclue.com/
    Content-Length: 0
    Date: Tue, 16 Aug 2016 20:31:56 GMT

The prefix of the short URLs can be defined with an environment variable. This will affect the `Location` header of in the response of successful POST requests.

    export SHORT_URL_PREFIX=https://bit.ly/

So far, short URLs are only stored in memory. It would be interesting to store them in a file or on a Redis instance instead.

## Why Rust?

I'm interested by this language for many reasons that I will not describe here.

I don't have so much experience with it yet, and I'm always looking for new opportunities to experiment.

## Lessons learned

Overall that was a great coding experience. The language seems simple and intuitive, with very powerful features. The compiler is awesome, and usually, if it compiles, it will run the way you expected.

Despite the Rust promises of being extremely efficient (at execution), some basic libraries are still missing for this specific problem. A high-level async Web framework would have been great, as well as an efficient concurrent HashMap.

Because I'm new to the language, I obviously struggled on very simple things. There are really cool features I definitely need to understand better, such as higher order functions.