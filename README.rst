Simple log based HTTP statistics script
=======================================

**WARNING:** This simple script is basically my way to learn `Rust
programming language <http://www.rust-lang.org/>`_.

Usage
-----

Compile ``httpstats`` with the following command::

    $ make

And then run it in the following way::

    $ ./httpstats path/to/access.log

Limitations
-----------

- Currently only `Rust 0.12
  <https://github.com/mozilla/rust/wiki/Doc-releases>`_ is supported
- Currently ``httpstats`` only parse `Nginx <http://nginx.org/>`_ logs with the
  following format::

    log_format full '$remote_addr $remote_user [$time_local] '
                    '$host $pipe $request_time "$request" '
                    '$status $body_bytes_sent '
                    '"$http_referer" "$http_user_agent"';
