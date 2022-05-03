# Algorand OAS-Generated Clients

`algod` client generated with:

```bash
docker run --rm \
  -v ${PWD}:/local openapitools/openapi-generator-cli generate \
  -i /local/algod.oas3.yml \
  -g rust \
  -o /local/algod \
  --skip-validate-spec
```

`indexer` client generated with:

```bash
docker run --rm \
  -v ${PWD}:/local openapitools/openapi-generator-cli generate \
  -i /local/indexer.oas3.yml \
  -g rust \
  -o /local/indexer \
  --skip-validate-spec
```
