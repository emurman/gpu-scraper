-- Your SQL goes here
CREATE TABLE price (
    id SERIAL PRIMARY KEY,
    model_name VARCHAR NOT NULL,
    product_name VARCHAR NOT NULL,
    value DOUBLE PRECISION NOT NULL,
    "date" TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT fk_model
      FOREIGN KEY(model_name) 
	  REFERENCES model("name")
)