#!/bin/bash

docker exec -i db psql -U postgres <<EOT
select DISTINCT ON (date::timestamp::date) date::timestamp::date, value
FROM model, product, price
WHERE price.product_id = product.id and product.model_id = model.id and model.name = '$1'
ORDER BY
        date::timestamp::date,
        value ASC;
EOT
