# STOSP
Open source project for school trips

1. install Cargo
2. install diesel-cli
    ``brew install mysql-client``
    After installing mysql-client
        If you need to have mysql-client first in your PATH, run:
        echo 'export PATH="/usr/local/opt/mysql-client/bin:$PATH"' >> ~/.zshrc

        For compilers to find mysql-client you may need to set:
        export LDFLAGS="-L/usr/local/opt/mysql-client/lib"
        export CPPFLAGS="-I/usr/local/opt/mysql-client/include"

``cargo install diesel_cli``

3. diesel migration run
4. cargo run-script start

to create migrations
    diesel migration generate <file_name>


Cargo Dependencies: 
    1. cargo-run-script
    2. cargo watch
    3. diesel