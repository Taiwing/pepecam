FROM httpd:2.4

WORKDIR /usr/local/apache2/

ARG API_PORT
ARG SUPERPOSABLES_SIDE
ARG PICTURES_DIR
ARG SUPERPOSABLES_DIR

COPY <<EOF ./htdocs/environment.js
export default {
	API_PORT: '$API_PORT',
	SUPERPOSABLES_SIDE: '$SUPERPOSABLES_SIDE',
	PICTURES_DIR: '$PICTURES_DIR',
	SUPERPOSABLES_DIR: '$SUPERPOSABLES_DIR',
}
EOF

COPY ./src/*.html ./htdocs/
COPY ./src/*.js ./htdocs/
COPY ./src/style ./htdocs/style
COPY ./favicon ./htdocs/favicon
COPY ./icons ./htdocs/icons
COPY ./httpd.conf ./conf/httpd.conf
