This is a code sample that reproduces 2 bugs in Sawtooth 1.1.4

The first:

* I call the service.commit_block method from the consensus engine;
* **While a block is not committed** another call to the same method occurs (for
  me it was also for the same block);
* The validator exits with a panic.

The second:

* I call the service.commit_block method from the consensus engine;
* **After a block is committed** committed another call to the same method
  occurs (for me it was also for the same block);
* The validator exits with a panic.

## How to use this repo

* Build the consensus image with `docker build -t buggy-consensus:latest .`;
* Run this code with `docker-compose up`;
* Try committing transactions (I use settings) and you will observe the crash.

You can switch between two bugs in `main.rs`.
