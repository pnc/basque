.load ./target/debug/libbasque
select basque_cmd("ls", "-lh");
select basque_cmd("df", "-h");
