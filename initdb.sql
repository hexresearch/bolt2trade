DO
$do$
BEGIN
   IF NOT EXISTS (
      SELECT FROM pg_catalog.pg_roles  -- SELECT list can be empty for this
      WHERE  rolname = 'boltdb') THEN

      CREATE ROLE boltdb LOGIN PASSWORD 'boltdb';
   END IF;
END
$do$;

SELECT 'CREATE DATABASE boltdb'
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'boltdb')\gexec