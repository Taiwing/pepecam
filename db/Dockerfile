FROM postgres:14.3
COPY ./create_tables.sql /docker-entrypoint-initdb.d/create_tables.sql
COPY ./populate.sql /docker-entrypoint-initdb.d/populate.sql
