tests need to be run with one thread, since they access one database

  cargo test -- --test-threads=1
