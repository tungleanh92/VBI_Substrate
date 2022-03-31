# curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "genKitty_push"}' http://localhost:9933/
# curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "kittyInfo_get"}' http://localhost:9933/
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "kittyQuantity_get"}' http://localhost:9933/

