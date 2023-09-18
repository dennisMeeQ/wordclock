cross build --release --target arm-unknown-linux-gnueabihf
ssh wordclock "rm /wordclock/wordclock"
scp ./target/arm-unknown-linux-gnueabihf/release/wordclock wordclock:/wordclock