CREATE TABLE product (
    id VARCHAR PRIMARY KEY,
    model_id INT NOT NULL,
    product_name VARCHAR NOT NULL,
    CONSTRAINT fk_model
      FOREIGN KEY(model_id) 
	  REFERENCES model("id")
)