#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES" <<-EOSQL
  CREATE DATABASE codo_maton_db;
  \c codo_maton_db
  create extension if not exists "uuid-ossp";
EOSQL