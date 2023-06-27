FROM postgres:14.3

COPY ./create_tables.sql /docker-entrypoint-initdb.d/create_tables.sql
COPY ./populate.sql /populate.sql
COPY ./populate.sh /docker-entrypoint-initdb.d/populate.sh
