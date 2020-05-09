CREATE TABLE products (
	id SERIAL PRIMARY KEY,
	product_id VARCHAR NOT NULL,
	name VARCHAR NOT NULL,
	bottled_volume REAL NOT NULL,
	alcohol_by_volume REAL NOT NULL,
	country_of_origin VARCHAR NOT NULL,
	is_available BOOLEAN NOT NULL DEFAULT 'f',
	container_type VARCHAR NOT NULL,
	style VARCHAR NOT NULL,
	sub_style VARCHAR NOT NULL,
	producer VARCHAR NOT NULL,
	short_description VARCHAR NOT NULL,
	date_on_market DATE NOT NULL,
	season VARCHAR NOT NULL
);

CREATE TABLE product_prices (
	id SERIAL PRIMARY KEY,
	products_id INTEGER NOT NULL,
	price REAL NOT NULL,
	timestamp TIMESTAMP NOT NULL
);
