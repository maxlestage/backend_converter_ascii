#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES" <<-EOSQL
  CREATE DATABASE codo_maton;
  \c codo_maton
  create extension if not exists "uuid-ossp";
EOSQL