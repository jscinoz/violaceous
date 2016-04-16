* Actually write everything
* Thread-safety - libpurple is inherently not even slightly thread-safe
  (globals). Use rust Mutex / other things to ensure expose API is thread safe
