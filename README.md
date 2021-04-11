pwned-search
============

Rust implementation of pwned password API lookup as described in this
computerphile video https://www.youtube.com/watch?v=hhUb5iknVJs.

See https://github.com/mikepound/pwned-search for Dr Mike Pound's
python code that was shown in the video.

Usage:

- Run without arguments to read passwords from stdin (one per line)
- Or provide passwords as command line arguments (beware the password
may be saved in shell history and that other users on the system may be
able to observe the command line).
