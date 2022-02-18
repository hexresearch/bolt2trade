let
sources = import ./nix/sources.nix;
pkgs = import sources.nixpkgs {};
in with pkgs;
let merged-openssl = symlinkJoin { name = "merged-openssl"; paths = [ openssl.out openssl.dev ]; };
in stdenv.mkDerivation rec {
  name = "rust-env";
  env = buildEnv { name = name; paths = buildInputs; };

  buildInputs = [
    rustup
    clang
    llvmPackages_11.libclang
    openssl
    valgrind
    kdeApplications.kcachegrind
    llvm_11
    postgresql
  ];
  shellHook = ''
  export LIBCLANG_PATH="${llvmPackages.libclang}/lib"
  export OPENSSL_DIR="${merged-openssl}"
  export PGDATA=./db-bolt
  export PGPORT=5435
  export LANG=C
  export LC_ALL=C
  export DATABASE_URL=postgres://boltdb:boltdb@localhost/boltdb
  
  function cleanup {
    echo "Stopping DB..."
    pg_ctl stop
  }
  trap cleanup EXIT

  if [ ! -d "$PGDATA" ]; then
    echo "Deploying local PostgreSQL"
    initdb $PGDATA --auth=trust
    echo "unix_socket_directories = '$PWD'" >> $PGDATA/postgresql.conf
    pg_ctl start -D $PGDATA -l $PGDATA.log
    sleep 1
    psql --host=$PWD -d postgres -f ./initdb.sql
    sleep 2
  else
    pg_ctl -D $PGDATA -l $PGDATA.log start
    sleep 1
  fi
  '';
}
