if [ -n "${POPULATE_DB}" ]; then
    echo "Populating database..."
	psql -v ON_ERROR_STOP=1 -U $DB_USER -d postgres -p $DB_PORT -f /populate.sql
fi
